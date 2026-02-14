//! Secure peer-to-peer chatroom CLI
//!
//! Decentralized chatroom using Korium's adaptive networking fabric
//! with PubSub messaging, direct messaging, and automatic peer discovery.
//!
//! Protocol Version: 1.3
//! Binary message format using postcard serialization.

use std::collections::HashMap;
use std::io::BufRead;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use clap::Parser;
use rand::Rng;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use korium::Node;

// ============================================================================
// Six7 Message Protocol v1.3
// ============================================================================

/// Message types supported by the Six7 protocol
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum MessageType {
    Text,
    Image,
    Video,
    Audio,
    Document,
    Location,
    Contact,
    GroupInvite,
    ContactRequest,
    ContactAccepted,
    Vibe,
    ReadReceipt,
    ProfileUpdate,
}

impl std::fmt::Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageType::Text => write!(f, "text"),
            MessageType::Image => write!(f, "image"),
            MessageType::Video => write!(f, "video"),
            MessageType::Audio => write!(f, "audio"),
            MessageType::Document => write!(f, "document"),
            MessageType::Location => write!(f, "location"),
            MessageType::Contact => write!(f, "contact"),
            MessageType::GroupInvite => write!(f, "groupInvite"),
            MessageType::ContactRequest => write!(f, "contactRequest"),
            MessageType::ContactAccepted => write!(f, "contactAccepted"),
            MessageType::Vibe => write!(f, "vibe"),
            MessageType::ReadReceipt => write!(f, "readReceipt"),
            MessageType::ProfileUpdate => write!(f, "profileUpdate"),
        }
    }
}

/// Direct Chat Message (RPC)
/// Used for 1:1 direct messages between peers
/// Note: Sender identity is authenticated by Korium transport layer
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectMessage {
    /// UUID v4 unique message identifier
    pub id: String,
    /// Message content (JSON-escaped)
    pub content: String,
    /// Unix epoch milliseconds
    pub timestamp: i64,
    /// Message type enum value
    pub message_type: String,
}

impl DirectMessage {
    pub fn new(content: &str, message_type: MessageType) -> Self {
        Self {
            id: random_hex_id(),
            content: content.to_string(),
            timestamp: current_timestamp_ms(),
            message_type: message_type.to_string(),
        }
    }

    pub fn text(content: &str) -> Self {
        Self::new(content, MessageType::Text)
    }

    pub fn contact_request(display_name: &str) -> Self {
        Self::new(display_name, MessageType::ContactRequest)
    }

    pub fn contact_accepted(display_name: &str) -> Self {
        Self::new(display_name, MessageType::ContactAccepted)
    }

    pub fn read_receipt(message_ids: &[&str]) -> Self {
        Self {
            id: format!("rr-{}", current_timestamp_ms()),
            content: message_ids.join(","),
            timestamp: current_timestamp_ms(),
            message_type: MessageType::ReadReceipt.to_string(),
        }
    }
}

/// Group Message (PubSub)
/// Topic: six7-groups:{groupId}
/// Note: Sender identity is authenticated by Korium transport layer
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMessage {
    /// UUID v4 unique message identifier
    pub id: String,
    /// Message content (JSON-escaped)
    pub content: String,
    /// Unix epoch milliseconds
    pub timestamp: i64,
    /// Message type enum value
    pub message_type: String,
    /// UUID v4 group identifier
    pub group_id: String,
}

impl GroupMessage {
    pub fn new(content: &str, message_type: MessageType, group_id: &str) -> Self {
        Self {
            id: random_hex_id(),
            content: content.to_string(),
            timestamp: current_timestamp_ms(),
            message_type: message_type.to_string(),
            group_id: group_id.to_string(),
        }
    }

    pub fn text(content: &str, group_id: &str) -> Self {
        Self::new(content, MessageType::Text, group_id)
    }
}

/// Group Invite metadata (embedded in DirectMessage content as JSON string)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupInvitePayload {
    pub group_id: String,
    pub name: String,
    pub description: String,
    pub member_ids: Vec<String>,
    pub member_names: HashMap<String, String>,
    pub creator_id: String,
    pub created_at_ms: i64,
}

/// ACK Response for direct messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AckResponse {
    pub ack: bool,
}

impl AckResponse {
    pub fn success() -> Self {
        Self { ack: true }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        postcard::to_allocvec(self).expect("AckResponse serialization is infallible")
    }
}

/// Vibe payload types for anonymous matching
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum VibePayload {
    #[serde(rename = "commitment")]
    Commitment { vibe_id: String, commitment: String },
    #[serde(rename = "reveal")]
    Reveal { vibe_id: String, secret: String },
}

// Topic prefixes
pub const TOPIC_PREFIX_GROUP: &str = "six7-groups:";
pub const TOPIC_VIBES: &str = "six7-vibes";

// Size limits
pub const MAX_MESSAGE_SIZE_BYTES: usize = 65536;
pub const MAX_TOPIC_LENGTH: usize = 256;
pub const MAX_IDENTITY_LENGTH: usize = 64;
pub const GROUP_ID_LENGTH: usize = 36;

fn sanitize_text(text: &str) -> String {
    text.chars()
        .filter(|c| !c.is_control() || *c == '\n' || *c == '\t')
        .collect()
}

fn current_timestamp_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

/// Generate a random 128-bit hex identifier (replaces UUID v4).
fn random_hex_id() -> String {
    let mut bytes = [0u8; 16];
    rand::thread_rng().fill(&mut bytes);
    hex::encode(bytes)
}

// ============================================================================
// CLI
// ============================================================================

/// Secure peer-to-peer chatroom built on Korium's adaptive networking fabric.
#[derive(Parser, Debug)]
#[command(name = "six7", version)]
#[command(about = "Secure peer-to-peer chatroom CLI built on Korium")]
#[command(
    long_about = "six7 is a decentralized chatroom that uses Korium's adaptive networking \
                        fabric for secure, NAT-traversing peer-to-peer communication.\n\n\
                        Features: PubSub messaging, direct messaging, automatic peer discovery.\n\
                        Protocol Version 1.3 — Compatible with the Six7 mobile app."
)]
struct Args {
    /// Display name in the chatroom
    #[arg(short, long, default_value = "anon")]
    name: String,

    /// Chatroom to join
    #[arg(short, long, default_value = "lobby")]
    room: String,

    /// Port to bind to (0 = random)
    #[arg(short, long, default_value = "0")]
    port: u16,

    /// Bootstrap peer: `<address>/<identity_hex>`
    #[arg(short = 'B', long = "bootstrap")]
    bootstrap: Option<String>,

    /// Bootstrap from public Korium network
    #[arg(short = 'P', long = "public")]
    public: bool,

    /// Enable debug logging
    #[arg(short = 'd', long = "debug")]
    debug: bool,
}

type PeerRegistry = Arc<RwLock<HashMap<String, String>>>;

/// Parse a bootstrap string using Korium's own parser, with manual fallback
/// for the `addr/identity` format used in the CLI banner.
fn parse_bootstrap(s: &str) -> Result<(String, String)> {
    if let Some((identity, addr)) = Node::parse_bootstrap_txt(s) {
        return Ok((identity, addr));
    }
    let parts: Vec<&str> = s.splitn(2, '/').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid bootstrap format. Expected: <address>/<identity_hex>");
    }
    let addr = parts[0].to_string();
    let identity = parts[1].to_string();
    if identity.len() != MAX_IDENTITY_LENGTH {
        anyhow::bail!("Identity must be {} hex characters", MAX_IDENTITY_LENGTH);
    }
    hex::decode(&identity).context("Identity must be valid hex")?;
    Ok((identity, addr))
}

fn print_banner(args: &Args, display_addr: &str, identity: &str) {
    println!();
    println!("six7");
    println!();
    println!("Nickname : {}", args.name);
    println!("Room     : {}", args.room);
    println!("Address  : {}", display_addr);
    println!();
    println!("Your Identity (for DMs):");
    println!("{}", identity);
    println!();
    println!("Bootstrap string (copy this line):");
    println!("{}/{}", display_addr, identity);
}

fn print_help() {
    println!();
    println!("Commands:");
    println!("  /dm <identity> <message>  - Send direct message");
    println!("  /contact <identity>       - Send contact request");
    println!("  /peers                    - List peers discovered via room messages");
    println!(
        "  /list                     - Show all peer tables (fabric/transport/routing/gossipsub/dht)"
    );
    println!("  /telemetry                - Show node telemetry");
    println!("  /help                     - Show this help");
    println!("  /quit                     - Exit");
    println!();
    println!("Anything else is broadcast to the room (Protocol v1.3).");
    println!();
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize logging
    let log_level = if args.debug { "debug" } else { "warn" };
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(log_level)),
        )
        .init();

    // Build node (includes PoW identity mining)
    let bind_addr = format!("0.0.0.0:{}", args.port);

    print!("Mining identity (PoW)... ");
    std::io::Write::flush(&mut std::io::stdout()).ok();

    let node = Arc::new(Node::builder(&bind_addr).build().await?);

    println!("done!");

    let local_addr = node.local_addr()?;
    let routable_addrs = node.routable_addresses();
    let display_addr = routable_addrs
        .first()
        .map_or_else(|| local_addr.to_string(), |a| a.clone());
    let identity = node.identity();

    let peers: PeerRegistry = Arc::new(RwLock::new(HashMap::new()));

    print_banner(&args, &display_addr, &identity);

    // Bootstrap
    if args.public {
        println!("\nBootstrapping from public Korium network...");
        match node.bootstrap_public().await {
            Ok(()) => println!("Bootstrap successful!"),
            Err(e) => eprintln!("Bootstrap failed: {e}"),
        }
    } else if let Some(ref bootstrap_str) = args.bootstrap {
        let (peer_identity, addr) = parse_bootstrap(bootstrap_str)?;
        println!("\nBootstrapping from {addr}...");
        match node.bootstrap(&peer_identity, &[addr]).await {
            Ok(()) => println!("Bootstrap successful!"),
            Err(e) => eprintln!("Bootstrap failed: {e}"),
        }
    } else {
        println!("\nNo bootstrap peer specified. This node is the first in the network.");
        println!("Others can connect using the bootstrap string above.");
    }

    // Subscribe to room topic
    let room_topic = format!("chat/{}", args.room);
    node.subscribe(&room_topic).await?;
    println!("\nSubscribed to room: {}", args.room);

    // Get message receivers
    let mut pubsub_rx = node.messages().await?;
    let mut dm_rx = node.incoming_requests().await?;

    let room_filter = args.room.clone();
    let my_identity = identity.clone();
    let peers_for_pubsub = peers.clone();

    // PubSub handler
    tokio::spawn(async move {
        while let Some(msg) = pubsub_rx.recv().await {
            if msg.data.len() > MAX_MESSAGE_SIZE_BYTES {
                continue;
            }

            if msg.topic != format!("chat/{room_filter}") {
                continue;
            }

            let sender_id = &msg.from;
            if sender_id == &my_identity {
                continue;
            }

            let id_prefix = &sender_id[..8.min(sender_id.len())];

            let (sender_name, display_content) =
                match postcard::from_bytes::<GroupMessage>(&msg.data) {
                    Ok(group_msg) => {
                        let name = {
                            let peers = peers_for_pubsub.read().await;
                            peers.get(id_prefix).cloned()
                        }
                        .unwrap_or_else(|| id_prefix.to_string());
                        (
                            name.clone(),
                            format!("{}@{}: {}", name, id_prefix, group_msg.content),
                        )
                    }
                    Err(_) => {
                        // Legacy plain-text fallback
                        let text = String::from_utf8_lossy(&msg.data);
                        let sender_name = text
                            .split_once(": ")
                            .and_then(|(prefix, _)| prefix.split_once('@'))
                            .map(|(name, _)| name.to_string())
                            .unwrap_or_else(|| id_prefix.to_string());
                        (sender_name, text.to_string())
                    }
                };

            // Track peer
            {
                let mut peers = peers_for_pubsub.write().await;
                if peers.len() > 1000 {
                    peers.clear(); // Prevent unbounded growth
                }
                peers
                    .entry(id_prefix.to_string())
                    .or_insert_with(|| sender_name.clone());
            }

            println!("\x1b[32m[room]\x1b[0m {}", sanitize_text(&display_content));
        }
    });

    // DM handler
    tokio::spawn(async move {
        while let Some((from, data, response_tx)) = dm_rx.recv().await {
            if data.len() > MAX_MESSAGE_SIZE_BYTES {
                continue;
            }
            let from_short = &from[..8.min(from.len())];
            match postcard::from_bytes::<DirectMessage>(&data) {
                Ok(dm) => {
                    let tag = match dm.message_type.as_str() {
                        "text" => "",
                        "contactRequest" => " [contact request]",
                        "contactAccepted" => " [contact accepted]",
                        "readReceipt" => " [read receipt]",
                        "groupInvite" => " [group invite]",
                        "vibe" => " [vibe]",
                        "profileUpdate" => " [profile update]",
                        other => {
                            println!(
                                "\x1b[35m[dm ← {}]\x1b[0m [{}] {}",
                                from_short,
                                other,
                                sanitize_text(&dm.content)
                            );
                            let _ = response_tx.send(AckResponse::success().to_bytes());
                            continue;
                        }
                    };
                    println!(
                        "\x1b[35m[dm ← {}]\x1b[0m{} {}",
                        from_short,
                        tag,
                        sanitize_text(&dm.content)
                    );
                    let _ = response_tx.send(AckResponse::success().to_bytes());
                }
                Err(_) => {
                    let text = String::from_utf8_lossy(&data);
                    println!(
                        "\x1b[35m[dm ← {}]\x1b[0m {}",
                        from_short,
                        sanitize_text(&text)
                    );
                    let _ = response_tx.send(b"received".to_vec());
                }
            }
        }
    });

    print_help();

    // Read stdin on a blocking OS thread, bridge to async via channel.
    let (stdin_tx, mut stdin_rx) = tokio::sync::mpsc::channel::<String>(16);
    std::thread::spawn(move || {
        let stdin = std::io::stdin();
        let reader = stdin.lock();
        for line in reader.lines() {
            match line {
                Ok(l) => {
                    if stdin_tx.blocking_send(l).is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });

    while let Some(line) = stdin_rx.recv().await {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match line {
            "/quit" => {
                println!("Goodbye!");
                break;
            }
            "/help" => {
                print_help();
            }
            "/peers" => {
                let guard = peers.read().await;
                if guard.is_empty() {
                    println!("No peers discovered yet.");
                } else {
                    println!("Known peers:");
                    for (id_prefix, name) in guard.iter() {
                        println!("  {name} ({id_prefix})");
                    }
                }
            }
            "/list" => {
                // ── Fabric (QUIC) ──────────────────────────────────────
                let fab_all = node.all_contacts().await;
                let fab_connected = node.connected_contacts().await;
                let fab_connected_ids: std::collections::HashSet<_> =
                    fab_connected.iter().map(|c| c.identity).collect();

                println!();
                println!(
                    "\x1b[1m── Fabric (QUIC) ── {} peers, {} connected\x1b[0m",
                    fab_all.len(),
                    fab_connected.len()
                );
                if fab_all.is_empty() {
                    println!("  (none)");
                } else {
                    for c in &fab_all {
                        let short = &hex::encode(c.identity.as_bytes())[..16];
                        let status = if fab_connected_ids.contains(&c.identity) {
                            "\x1b[32mconnected\x1b[0m"
                        } else {
                            "\x1b[31mdisconnected\x1b[0m"
                        };
                        let addrs = c.addrs.join(", ");
                        println!("  {}..  [{}]  {}", short, status, addrs);
                    }
                }

                // ── Transport (UDP) ─────────────────────────────────────────────
                let transport_peers = node.transport_peers();
                println!();
                println!(
                    "\x1b[1m── Transport (UDP) ── {} peers\x1b[0m",
                    transport_peers.len()
                );
                if transport_peers.is_empty() {
                    println!("  (none)");
                } else {
                    for (id, addr, rtt) in &transport_peers {
                        let short = &hex::encode(id.as_bytes())[..16];
                        let rtt_str = match rtt {
                            Some(d) => format!("{:.1}ms", d.as_secs_f64() * 1000.0),
                            None => "—".to_string(),
                        };
                        println!("  {}..  rtt={:<10}  vaddr={}", short, rtt_str, addr);
                    }
                }

                // ── DHT Routing ────────────────────────────────────────
                let routing = node.get_peers().await;
                println!();
                println!("\x1b[1m── DHT Routing ── {} contacts\x1b[0m", routing.len());
                if routing.is_empty() {
                    println!("  (none)");
                } else {
                    for c in &routing {
                        let short = &hex::encode(c.identity.as_bytes())[..16];
                        let addrs = c.addrs.join(", ");
                        println!("  {}..  {}", short, addrs);
                    }
                }

                // ── GossipSub ──────────────────────────────────────────
                let topic_peers = node.gossipsub_topic_peers().await;
                let total_unique: std::collections::HashSet<_> = topic_peers
                    .iter()
                    .flat_map(|tp| tp.eager_peers.iter().chain(tp.lazy_peers.iter()))
                    .collect();
                println!();
                println!(
                    "\x1b[1m── GossipSub ── {} topics, {} unique peers\x1b[0m",
                    topic_peers.len(),
                    total_unique.len()
                );
                if topic_peers.is_empty() {
                    println!("  (none)");
                } else {
                    for tp in &topic_peers {
                        println!(
                            "  topic: {}  ({} eager, {} lazy)",
                            tp.topic,
                            tp.eager_peers.len(),
                            tp.lazy_peers.len()
                        );
                        for p in &tp.eager_peers {
                            let short = &hex::encode(p.as_bytes())[..16];
                            println!("    \x1b[32meager\x1b[0m  {}.. ", short);
                        }
                        for p in &tp.lazy_peers {
                            let short = &hex::encode(p.as_bytes())[..16];
                            println!("    \x1b[33mlazy\x1b[0m   {}.. ", short);
                        }
                    }
                }

                // ── DHT Store ──────────────────────────────────────────
                let store = node.list_dht_store().await;
                println!();
                println!("\x1b[1m── DHT Store ── {} entries\x1b[0m", store.len());
                if store.is_empty() {
                    println!("  (none)");
                } else {
                    for (key, value_len, stored_by) in &store {
                        println!("  {}  ({} bytes, by {})", key, value_len, stored_by);
                    }
                }
                println!();
            }
            "/telemetry" => {
                let t = node.telemetry().await;
                println!("╔════════════════════════════════════════════════════════════════╗");
                println!("║                         Telemetry                              ║");
                println!("╠════════════════════════════════════════════════════════════════╣");
                println!(
                    "║ DHT Store        : {:>6} keys                                 ║",
                    t.stored_keys
                );
                println!(
                    "║ DHT Replication  : {:>6}                                       ║",
                    t.replication_factor
                );
                println!(
                    "║ DHT Concurrency  : {:>6}                                       ║",
                    t.concurrency
                );
                println!(
                    "║ DHT Pressure     : {:>6.2}                                      ║",
                    t.pressure
                );
                println!(
                    "║ Routing Peers    : {:>6}                                       ║",
                    t.connected_peers
                );
                println!(
                    "║ GossipSub Mesh   : {:>6} peers                                 ║",
                    t.gossipsub_mesh_peers
                );
                println!(
                    "║ GossipSub Topics : {:>6}                                       ║",
                    t.gossipsub_topics
                );
                println!(
                    "║ Transport Sent   : {:>6}                                       ║",
                    t.transport_requests_sent
                );
                println!(
                    "║ Transport Recv   : {:>6}                                       ║",
                    t.transport_requests_received
                );
                println!(
                    "║ Transport OK     : {:>6}                                       ║",
                    t.transport_responses_success
                );
                println!(
                    "║ Transport Errors : {:>6}                                       ║",
                    t.transport_errors
                );
                println!(
                    "║ Connections      : {:>6} cached                                ║",
                    t.transport_connections_cached
                );
                println!(
                    "║ Connections Est. : {:>6}                                       ║",
                    t.transport_connections_established
                );
                if !t.tier_centroids.is_empty() {
                    let tiers: Vec<String> = t
                        .tier_centroids
                        .iter()
                        .zip(t.tier_counts.iter())
                        .map(|(c, n)| format!("{:.0}ms({})", c, n))
                        .collect();
                    println!("║ Latency Tiers    : {:<45} ║", tiers.join(", "));
                }
                println!("╚════════════════════════════════════════════════════════════════╝");
            }
            _ if line.starts_with("/dm ") => {
                let parts: Vec<&str> = line.splitn(3, ' ').collect();
                if parts.len() < 3 {
                    println!("Usage: /dm <identity_hex> <message>");
                    continue;
                }

                let peer_identity = parts[1];
                let message = parts[2];

                if message.len() > MAX_MESSAGE_SIZE_BYTES {
                    println!("Message too large (max {} bytes)", MAX_MESSAGE_SIZE_BYTES);
                    continue;
                }

                if peer_identity.len() != MAX_IDENTITY_LENGTH || hex::decode(peer_identity).is_err()
                {
                    println!(
                        "Invalid identity. Must be {} hex characters.",
                        MAX_IDENTITY_LENGTH
                    );
                    continue;
                }

                let dm = DirectMessage::text(message);
                let payload = postcard::to_allocvec(&dm).expect("Failed to serialize message");

                match tokio::time::timeout(
                    Duration::from_secs(10),
                    node.send(peer_identity, payload),
                )
                .await
                {
                    Ok(Ok(response)) => {
                        let ack = match postcard::from_bytes::<AckResponse>(&response) {
                            Ok(a) if a.ack => "✓",
                            _ => {
                                if String::from_utf8_lossy(&response) == "received" {
                                    "✓"
                                } else {
                                    "?"
                                }
                            }
                        };
                        println!(
                            "\x1b[33m[dm → {}]\x1b[0m {} [{}]",
                            &peer_identity[..8],
                            message,
                            ack
                        );
                    }
                    Ok(Err(e)) => eprintln!("\x1b[31m[dm error]\x1b[0m Failed to send: {e}"),
                    Err(_) => eprintln!("\x1b[31m[dm error]\x1b[0m Timeout: peer unreachable"),
                }
            }
            _ if line.starts_with("/contact ") => {
                let parts: Vec<&str> = line.splitn(2, ' ').collect();
                if parts.len() < 2 {
                    println!("Usage: /contact <identity_hex>");
                    continue;
                }

                let peer_identity = parts[1];

                if peer_identity.len() != MAX_IDENTITY_LENGTH || hex::decode(peer_identity).is_err()
                {
                    println!(
                        "Invalid identity. Must be {} hex characters.",
                        MAX_IDENTITY_LENGTH
                    );
                    continue;
                }

                let req = DirectMessage::contact_request(&args.name);
                let payload =
                    postcard::to_allocvec(&req).expect("Failed to serialize contact request");

                match tokio::time::timeout(
                    Duration::from_secs(10),
                    node.send(peer_identity, payload),
                )
                .await
                {
                    Ok(Ok(response)) => {
                        let status = match postcard::from_bytes::<AckResponse>(&response) {
                            Ok(a) if a.ack => "sent",
                            _ => "sent (legacy peer)",
                        };
                        println!(
                            "\x1b[36m[contact → {}]\x1b[0m {} [{}]",
                            &peer_identity[..8],
                            args.name,
                            status
                        );
                    }
                    Ok(Err(e)) => eprintln!("\x1b[31m[contact error]\x1b[0m Failed to send: {e}"),
                    Err(_) => eprintln!("\x1b[31m[contact error]\x1b[0m Timeout: peer unreachable"),
                }
            }
            _ if line.starts_with('/') => {
                println!("Unknown command. Type /help for available commands.");
            }
            _ => {
                if line.len() > MAX_MESSAGE_SIZE_BYTES {
                    println!("Message too large (max {} bytes)", MAX_MESSAGE_SIZE_BYTES);
                    continue;
                }
                // Broadcast to room
                let group_msg = GroupMessage::text(line, &args.room);
                let payload =
                    postcard::to_allocvec(&group_msg).expect("Failed to serialize message");
                let formatted = format!("{}@{}: {}", args.name, &identity[..8], line);

                if let Err(e) = node.publish(&room_topic, payload).await {
                    eprintln!("Failed to send message: {e}");
                } else {
                    println!("\x1b[32m[room]\x1b[0m {}", sanitize_text(&formatted));
                }
            }
        }
    }

    Ok(())
}
