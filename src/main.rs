//! six7 - A secure peer-to-peer chatroom CLI built on Korium
//!
//! This CLI implements a decentralized chatroom using Korium's adaptive networking
//! fabric with PubSub messaging, direct messaging, and automatic peer discovery.
//!
//! Message Protocol Version: 1.0
//! All messages use JSON format for interoperability with the Six7 mobile app.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncBufReadExt;
use tokio::sync::RwLock;
use uuid::Uuid;

use korium::Node;

// ============================================================================
// Six7 Message Protocol v1.0
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
            id: Uuid::new_v4().to_string(),
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
/// Topic: six7-group:{groupId}
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
    /// UUID v4 group identifier
    pub group_id: String,
}

impl GroupMessage {
    pub fn new(content: &str, group_id: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            content: content.to_string(),
            timestamp: current_timestamp_ms(),
            group_id: group_id.to_string(),
        }
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
        serde_json::to_vec(self).unwrap_or_else(|_| b"{\"ack\":true}".to_vec())
    }
}

/// Vibe payload types for anonymous matching
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum VibePayload {
    #[serde(rename = "commitment")]
    Commitment {
        vibe_id: String,
        commitment: String,
    },
    #[serde(rename = "reveal")]
    Reveal {
        vibe_id: String,
        secret: String,
    },
}

// Topic prefixes
pub const TOPIC_PREFIX_GROUP: &str = "six7-group:";
pub const TOPIC_PREFIX_PRESENCE: &str = "six7-presence-inbox:";
pub const TOPIC_VIBES: &str = "six7-vibes-v1";

// Size limits
pub const MAX_MESSAGE_SIZE_BYTES: usize = 65536;
pub const MAX_TOPIC_LENGTH: usize = 256;
pub const MAX_IDENTITY_LENGTH: usize = 64;
pub const GROUP_ID_LENGTH: usize = 36;

// Timing constants (seconds)
pub const HEARTBEAT_INTERVAL_SEC: u64 = 30;
pub const OFFLINE_THRESHOLD_SEC: u64 = 75;

/// Get current timestamp in milliseconds since Unix epoch
fn current_timestamp_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

// ============================================================================
// CLI Application
// ============================================================================

/// A secure peer-to-peer chatroom built on Korium's adaptive networking fabric
#[derive(Parser, Debug)]
#[command(name = "six7")]
#[command(version)]
#[command(about = "A secure peer-to-peer chatroom CLI built on Korium")]
#[command(long_about = "six7 is a decentralized chatroom that uses Korium's adaptive networking \
                        fabric for secure, NAT-traversing peer-to-peer communication. \
                        Features include PubSub messaging, direct messaging, and automatic peer discovery.\n\n\
                        Message Protocol Version: 1.0 - Compatible with Six7 mobile app.")]
struct Args {
    /// Your display name in the chatroom
    #[arg(short, long, default_value = "anon")]
    name: String,

    /// Name of the chatroom to join
    #[arg(short, long, default_value = "lobby")]
    room: String,

    /// Port to bind to (0 for random)
    #[arg(short, long, default_value = "0")]
    port: u16,

    /// Bootstrap peer address in format: <address>/<identity_hex>
    #[arg(short = 'B', long = "bootstrap")]
    bootstrap: Option<String>,

    /// Bootstrap using public Korium network nodes
    #[arg(short = 'P', long = "public")]
    public: bool,

    /// Enable debug logging
    #[arg(short = 'd', long = "debug")]
    debug: bool,
}

type PeerRegistry = Arc<RwLock<HashMap<String, String>>>;

fn parse_bootstrap(s: &str) -> Result<(SocketAddr, String)> {
    let parts: Vec<&str> = s.splitn(2, '/').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid bootstrap format. Expected: <address>/<identity_hex>");
    }
    let addr: SocketAddr = parts[0]
        .parse()
        .with_context(|| format!("Invalid socket address: {}", parts[0]))?;
    let identity = parts[1].to_string();
    if identity.len() != 64 {
        anyhow::bail!("Identity must be 64 hex characters");
    }
    hex::decode(&identity).with_context(|| "Identity must be valid hex")?;
    Ok((addr, identity))
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
    println!("  /dm <identity> <message>  - Send direct message (JSON protocol)");
    println!("  /contact <identity>       - Send contact request");
    println!("  /peers                    - List known peers from room messages");
    println!("  /fabric                   - Show all peers in fabric (connection state)");
    println!("  /routing                  - Show DHT routing table");
    println!("  /dht                      - Show DHT store entries");
    println!("  /telemetry                - Show node statistics");
    println!("  /help                     - Show this help");
    println!("  /quit                     - Exit");
    println!();
    println!("Type anything else to broadcast to the room.");
    println!("Messages use Six7 Protocol v1.0 (JSON format).");
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

    // Bind node to address (includes PoW mining which takes time)
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

    // Handle bootstrapping
    if args.public {
        println!("\nBootstrapping from public Korium network...");
        match node.bootstrap_public().await {
            Ok(()) => println!("Bootstrap successful!"),
            Err(e) => eprintln!("Bootstrap failed: {e}"),
        }
    } else if let Some(bootstrap_str) = &args.bootstrap {
        let (addr, peer_identity) = parse_bootstrap(bootstrap_str)?;
        println!("\nBootstrapping from {addr}...");
        match node.bootstrap(&peer_identity, &[addr.to_string()]).await {
            Ok(()) => println!("Bootstrap successful!"),
            Err(e) => eprintln!("Bootstrap failed: {e}"),
        }
    } else {
        println!("\nNo bootstrap peer specified. This node will be the first in the network.");
        println!("Other nodes can connect using the bootstrap string above.");
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

    // Spawn PubSub message handler
    tokio::spawn(async move {
        while let Some(msg) = pubsub_rx.recv().await {
            if msg.topic == format!("chat/{room_filter}") {
                let sender_id = &msg.from;

                // Skip our own messages
                if sender_id == &my_identity {
                    continue;
                }

                // Try to parse as JSON GroupMessage first, fall back to legacy format
                // Note: sender_id comes from Korium's authenticated transport, not the message payload
                let (sender_name, display_content) = match serde_json::from_slice::<GroupMessage>(&msg.data) {
                    Ok(group_msg) => {
                        // JSON protocol message - use Korium-authenticated sender_id
                        let name = {
                            let peers = peers_for_pubsub.read().await;
                            peers.get(&sender_id[..8]).cloned()
                        }.unwrap_or_else(|| sender_id.clone());
                        (name.clone(), format!("{}@{}: {}", name, sender_id, group_msg.content))
                    }
                    Err(_) => {
                        // Legacy format: "name@id_prefix: message"
                        let text = String::from_utf8_lossy(&msg.data);
                        let sender_name = text
                            .split_once(": ")
                            .and_then(|(name_id, _)| name_id.split_once('@'))
                            .map(|(name, _)| name.to_string())
                            .unwrap_or_else(|| sender_id.clone());
                        (sender_name, text.to_string())
                    }
                };

                // Track peer
                let id_prefix = &sender_id[..8];
                let needs_insert = {
                    let peers = peers_for_pubsub.read().await;
                    !peers.contains_key(id_prefix)
                };
                if needs_insert {
                    let mut peers = peers_for_pubsub.write().await;
                    peers
                        .entry(id_prefix.to_string())
                        .or_insert_with(|| sender_name.clone());
                }

                println!("\x1b[32m[room]\x1b[0m {display_content}");
            }
        }
    });

    // Spawn DM handler
    tokio::spawn(async move {
        while let Some((from, data, response_tx)) = dm_rx.recv().await {
            // Try to parse as JSON DirectMessage
            match serde_json::from_slice::<DirectMessage>(&data) {
                Ok(dm) => {
                    let type_indicator = match dm.message_type.as_str() {
                        "text" => "",
                        "contactRequest" => " [contact request]",
                        "contactAccepted" => " [contact accepted]",
                        "readReceipt" => " [read receipt]",
                        "groupInvite" => " [group invite]",
                        "vibe" => " [vibe]",
                        other => {
                            println!("\x1b[35m[dm ← {}]\x1b[0m [{}] {}", from, other, dm.content);
                            let _ = response_tx.send(AckResponse::success().to_bytes());
                            continue;
                        }
                    };
                    println!(
                        "\x1b[35m[dm ← {}]\x1b[0m{} {}",
                        from,
                        type_indicator,
                        dm.content
                    );
                    // Send JSON ACK response per protocol
                    let _ = response_tx.send(AckResponse::success().to_bytes());
                }
                Err(_) => {
                    // Legacy format - plain text
                    let text = String::from_utf8_lossy(&data);
                    println!("\x1b[35m[dm ← {}]\x1b[0m {}", from, text);
                    // Send legacy acknowledgment for backward compatibility
                    let _ = response_tx.send(b"received".to_vec());
                }
            }
        }
    });

    print_help();

    let stdin = tokio::io::stdin();
    let mut stdin_reader = tokio::io::BufReader::new(stdin).lines();

    while let Some(line) = stdin_reader.next_line().await? {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if line == "/quit" {
            println!("Goodbye!");
            break;
        }

        if line == "/help" {
            print_help();
            continue;
        }

        if line == "/peers" {
            let peers_guard = peers.read().await;
            if peers_guard.is_empty() {
                println!("No peers discovered yet. Send messages to the room to discover peers.");
            } else {
                println!("Known peers:");
                for (id_prefix, name) in peers_guard.iter() {
                    println!("  {name} ({id_prefix})");
                }
            }
            continue;
        }

        if line == "/routing" {
            let contacts = node.get_peers().await;
            if contacts.is_empty() {
                println!("Routing table is empty.");
            } else {
                println!("Routing table ({} contacts):", contacts.len());
                for contact in contacts {
                    let id_hex = hex::encode(contact.identity.as_bytes());
                    println!("  {} -> {:?}", id_hex, contact.addrs);
                }
            }
            continue;
        }

        if line == "/fabric" {
            let all_contacts = node.all_contacts();
            let connected_contacts = node.connected_contacts();
            if all_contacts.is_empty() {
                println!("Fabric is empty (no known peers).");
            } else {
                println!(
                    "Fabric ({} peers, {} connected):",
                    all_contacts.len(),
                    connected_contacts.len()
                );
                let connected_ids: std::collections::HashSet<_> = connected_contacts
                    .iter()
                    .map(|c| hex::encode(c.identity.as_bytes()))
                    .collect();
                for contact in all_contacts {
                    let id_hex = hex::encode(contact.identity.as_bytes());
                    let status = if connected_ids.contains(&id_hex) {
                        "\x1b[32mconnected\x1b[0m"
                    } else {
                        "\x1b[31mdisconnected\x1b[0m"
                    };
                    println!("  {} [{}] {:?}", id_hex, status, contact.addrs);
                }
            }
            continue;
        }

        if line == "/telemetry" {
            let t = node.telemetry().await;
            println!("╔════════════════════════════════════════════════════════════════╗");
            println!("║                         Telemetry                              ║");
            println!("╠════════════════════════════════════════════════════════════════╣");
            println!(
                "║ DHT Store        : {:>6} keys                                 ║",
                t.stored_keys
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
                "║ RPC Sent         : {:>6}                                       ║",
                t.rpc_requests_sent
            );
            println!(
                "║ RPC Received     : {:>6}                                       ║",
                t.rpc_requests_received
            );
            println!(
                "║ RPC Errors       : {:>6}                                       ║",
                t.rpc_errors
            );
            println!(
                "║ Connections      : {:>6} cached                                ║",
                t.rpc_connections_cached
            );
            println!(
                "║ Relay Packets    : {:>6}                                       ║",
                t.relay_packets_relayed
            );
            println!("╚════════════════════════════════════════════════════════════════╝");
            continue;
        }

        if line == "/dht" {
            let entries = node.list_dht_store().await;
            if entries.is_empty() {
                println!("DHT store is empty.");
            } else {
                println!("DHT store ({} entries):", entries.len());
                for (key, value_len, stored_by) in entries {
                    println!(
                        "  {} ({} bytes, by {})",
                        key,
                        value_len,
                        stored_by
                    );
                }
            }
            continue;
        }

        if line.starts_with("/dm ") {
            let parts: Vec<&str> = line.splitn(3, ' ').collect();
            if parts.len() < 3 {
                println!("Usage: /dm <identity_hex> <message>");
                println!("Example: /dm 5821a288e16c6491abcdef1234567890abcdef1234567890abcdef12345678 Hello!");
                continue;
            }

            let peer_identity = parts[1];
            let message = parts[2];

            if peer_identity.len() != 64 || hex::decode(peer_identity).is_err() {
                println!("Invalid identity. Must be 64 hex characters.");
                continue;
            }

            // Create JSON DirectMessage per protocol
            let dm = DirectMessage::text(message);
            let dm_payload = serde_json::to_vec(&dm).expect("Failed to serialize message");

            match tokio::time::timeout(Duration::from_secs(10), node.send(peer_identity, dm_payload)).await {
                Ok(Ok(response)) => {
                    // Try to parse JSON ACK response
                    let ack_status = match serde_json::from_slice::<AckResponse>(&response) {
                        Ok(ack) if ack.ack => "✓",
                        _ => {
                            let text = String::from_utf8_lossy(&response);
                            if text == "received" { "✓" } else { "?" }
                        }
                    };
                    println!(
                        "\x1b[33m[dm → {}]\x1b[0m {} [{}]",
                        peer_identity,
                        message,
                        ack_status
                    );
                }
                Ok(Err(e)) => {
                    eprintln!("\x1b[31m[dm error]\x1b[0m Failed to send: {e}");
                }
                Err(_) => {
                    eprintln!("\x1b[31m[dm error]\x1b[0m Timeout: peer unreachable");
                }
            }
            continue;
        }

        // Contact request command
        if line.starts_with("/contact ") {
            let parts: Vec<&str> = line.splitn(2, ' ').collect();
            if parts.len() < 2 {
                println!("Usage: /contact <identity_hex>");
                println!("Example: /contact 5821a288e16c6491abcdef1234567890abcdef1234567890abcdef12345678");
                continue;
            }

            let peer_identity = parts[1];

            if peer_identity.len() != 64 || hex::decode(peer_identity).is_err() {
                println!("Invalid identity. Must be 64 hex characters.");
                continue;
            }

            // Create JSON ContactRequest per protocol
            let request = DirectMessage::contact_request(&args.name);
            let payload = serde_json::to_vec(&request).expect("Failed to serialize contact request");

            match tokio::time::timeout(Duration::from_secs(10), node.send(peer_identity, payload)).await {
                Ok(Ok(response)) => {
                    let ack_status = match serde_json::from_slice::<AckResponse>(&response) {
                        Ok(ack) if ack.ack => "sent",
                        _ => "sent (legacy peer)",
                    };
                    println!(
                        "\x1b[36m[contact request → {}]\x1b[0m {} [{}]",
                        peer_identity,
                        args.name,
                        ack_status
                    );
                }
                Ok(Err(e)) => {
                    eprintln!("\x1b[31m[contact error]\x1b[0m Failed to send request: {e}");
                }
                Err(_) => {
                    eprintln!("\x1b[31m[contact error]\x1b[0m Timeout: peer unreachable");
                }
            }
            continue;
        }

        // Regular message - broadcast to room using JSON GroupMessage format
        // Note: For the chatroom, we use the room name as group_id for simplicity
        // Sender identity is authenticated by Korium at the transport layer
        let group_msg = GroupMessage::new(line, &args.room);
        let json_payload = serde_json::to_vec(&group_msg).expect("Failed to serialize message");
        let formatted = format!("{}@{}: {}", args.name, identity, line);

        if let Err(e) = node
            .publish(&room_topic, json_payload)
            .await
        {
            eprintln!("Failed to send message: {e}");
        } else {
            println!("\x1b[32m[room]\x1b[0m {formatted}");
        }
    }

    Ok(())
}
