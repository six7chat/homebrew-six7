//! six7 - A secure peer-to-peer chatroom CLI built on Korium
//!
//! This CLI implements a decentralized chatroom using Korium's adaptive networking
//! fabric with PubSub messaging, direct messaging, and automatic peer discovery.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::{Context, Result};
use clap::Parser;
use tokio::io::AsyncBufReadExt;
use tokio::sync::RwLock;

use korium::Node;

/// The version of the korium dependency
const KORIUM_VERSION: &str = "0.6.22";

/// A secure peer-to-peer chatroom built on Korium's adaptive networking fabric
#[derive(Parser, Debug)]
#[command(name = "six7")]
#[command(version)]
#[command(about = "A secure peer-to-peer chatroom CLI built on Korium")]
#[command(long_about = "six7 is a decentralized chatroom that uses Korium's adaptive networking \
                        fabric for secure, NAT-traversing peer-to-peer communication. \
                        Features include PubSub messaging, direct messaging, and automatic peer discovery.")]
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
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║                     six7 Chatroom                              ║");
    println!("║                     powered by korium {:<24} ║", KORIUM_VERSION);
    println!("╠════════════════════════════════════════════════════════════════╣");
    println!("║ Nickname : {:<52} ║", args.name);
    println!("║ Room     : {:<52} ║", args.room);
    println!("║ Address  : {:<52} ║", display_addr);
    println!("╠════════════════════════════════════════════════════════════════╣");
    println!("║ Your Identity (for DMs):                                       ║");
    println!("║ {:<64} ║", identity);
    println!("╠════════════════════════════════════════════════════════════════╣");
    println!("║ Bootstrap string (copy this line):                             ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!("{}/{}", display_addr, identity);
}

fn print_help() {
    println!();
    println!("Commands:");
    println!("  /dm <identity> <message>  - Send direct message");
    println!("  /peers                    - List known peers from room messages");
    println!("  /fabric                   - Show all peers in fabric (connection state)");
    println!("  /routing                  - Show DHT routing table");
    println!("  /dht                      - Show DHT store entries");
    println!("  /telemetry                - Show node statistics");
    println!("  /help                     - Show this help");
    println!("  /quit                     - Exit");
    println!();
    println!("Type anything else to broadcast to the room.");
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

    // Bind node to address
    let bind_addr = format!("0.0.0.0:{}", args.port);
    let node = Arc::new(Node::bind(&bind_addr).await?);

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
                let text = String::from_utf8_lossy(&msg.data);
                let sender_id = &msg.from;

                // Skip our own messages
                if sender_id == &my_identity {
                    continue;
                }

                // Extract name from message content (format: "name@id_prefix: message")
                let sender_name = text
                    .split_once(": ")
                    .and_then(|(name_id, _)| name_id.split_once('@'))
                    .map(|(name, _)| name.to_string())
                    .unwrap_or_else(|| format!("{}...", &sender_id[..8]));

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

                println!("\x1b[32m[room]\x1b[0m {text}");
            }
        }
    });

    // Spawn DM handler
    tokio::spawn(async move {
        while let Some((from, data, response_tx)) = dm_rx.recv().await {
            let text = String::from_utf8_lossy(&data);
            println!("\x1b[35m[dm ← {}...]\x1b[0m {}", &from[..8], text);
            // Send acknowledgment
            let _ = response_tx.send(b"received".to_vec());
        }
    });

    print_help();

    let stdin = tokio::io::stdin();
    let mut stdin_reader = tokio::io::BufReader::new(stdin).lines();
    let my_id_prefix = &identity[..8];

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
                    println!("  {name} ({id_prefix}...)");
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
                    println!("  {}... -> {:?}", &id_hex[..16], contact.addrs);
                }
            }
            continue;
        }

        if line == "/fabric" {
            let fabric = node.fabric();
            let all_peers = fabric.all_peers();
            let connected_count = fabric.connected_contacts().len();
            if all_peers.is_empty() {
                println!("Fabric is empty (no known peers).");
            } else {
                println!(
                    "Fabric ({} peers, {} connected):",
                    all_peers.len(),
                    connected_count
                );
                for peer in all_peers {
                    let id_hex = hex::encode(peer.contact.identity.as_bytes());
                    let status = if peer.is_connected() {
                        "\x1b[32mconnected\x1b[0m"
                    } else {
                        "\x1b[31mdisconnected\x1b[0m"
                    };
                    println!("  {}... [{}] {:?}", &id_hex[..16], status, peer.contact.addrs);
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
                        "  {}... ({} bytes, by {}...)",
                        &key[..16],
                        value_len,
                        &stored_by[..8]
                    );
                }
            }
            continue;
        }

        if line.starts_with("/dm ") {
            let parts: Vec<&str> = line.splitn(3, ' ').collect();
            if parts.len() < 3 {
                println!("Usage: /dm <identity_hex> <message>");
                println!("Example: /dm 5821a288e16c6491... Hello!");
                continue;
            }

            let peer_identity = parts[1];
            let message = parts[2];

            if peer_identity.len() != 64 || hex::decode(peer_identity).is_err() {
                println!("Invalid identity. Must be 64 hex characters.");
                continue;
            }

            let dm_payload = format!("{}@{}: {}", args.name, my_id_prefix, message);

            match node.send(peer_identity, dm_payload.into_bytes()).await {
                Ok(response) => {
                    let response_text = String::from_utf8_lossy(&response);
                    println!(
                        "\x1b[33m[dm → {}...]\x1b[0m {} (ack: {})",
                        &peer_identity[..8],
                        message,
                        response_text
                    );
                }
                Err(e) => {
                    eprintln!("\x1b[31m[dm error]\x1b[0m Failed to send: {e}");
                }
            }
            continue;
        }

        // Regular message - broadcast to room
        let formatted = format!("{}@{}: {}", args.name, my_id_prefix, line);

        if let Err(e) = node
            .publish(&room_topic, formatted.as_bytes().to_vec())
            .await
        {
            eprintln!("Failed to send message: {e}");
        } else {
            println!("\x1b[32m[room]\x1b[0m {formatted}");
        }
    }

    Ok(())
}
