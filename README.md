# six7

A secure peer-to-peer chatroom CLI built on [Korium's](https://korium.io) adaptive networking fabric.

## Features

- **Zero Configuration** — Self-organizing mesh with automatic peer discovery
- **NAT Traversal** — Built-in relay infrastructure and path probing
- **Secure by Default** — Ed25519 identities with mutual TLS on every connection
- **PubSub Messaging** — Efficient topic-based publish/subscribe
- **Direct Messaging** — Encrypted point-to-point communication
- **Peer Discovery** — Automatic discovery via DHT

## Installation

### Via Homebrew (macOS/Linux)

```bash
brew tap six7chat/six7
brew install six7
```

### From Source

```bash
git clone https://github.com/six7chat/homebrew-six7.git
cd homebrew-six7
cargo install --path .
```

### Using Cargo

```bash
cargo install --git https://github.com/six7chat/homebrew-six7.git
```

## Usage

### Start a New Chatroom

```bash
# Start a node (first peer in the network)
six7 --name Alice --room dev
```

This will display your bootstrap string that others can use to join:

```
╔════════════════════════════════════════════════════════════════╗
║                     six7 Chatroom                              ║
╠════════════════════════════════════════════════════════════════╣
║ Nickname : Alice                                               ║
║ Room     : dev                                                 ║
║ Address  : 192.168.1.100:4433                                  ║
╠════════════════════════════════════════════════════════════════╣
║ Your Identity (for DMs):                                       ║
║ abc123def456...                                                ║
╠════════════════════════════════════════════════════════════════╣
║ Bootstrap string (copy this line):                             ║
╚════════════════════════════════════════════════════════════════╝
192.168.1.100:4433/abc123def456...
```

### Join an Existing Chatroom

```bash
# Join using bootstrap string from another peer
six7 --name Bob --room dev --bootstrap "192.168.1.100:4433/abc123def456..."
```

### Join Public Korium Network

```bash
# Bootstrap from public Korium network nodes
six7 --name Charlie --room dev --public
```

## Commands

| Command | Description |
|---------|-------------|
| `/dm <identity> <message>` | Send a direct message to a peer |
| `/peers` | List known peers from room messages |
| `/fabric` | Show all peers in fabric (connection state) |
| `/routing` | Show DHT routing table |
| `/dht` | Show DHT store entries |
| `/telemetry` | Show node statistics |
| `/help` | Show available commands |
| `/quit` | Exit the chatroom |

## CLI Options

```
Options:
  -n, --name <NAME>        Your display name [default: anon]
  -r, --room <ROOM>        Room to join [default: lobby]
  -p, --port <PORT>        Port to bind to (0 for random) [default: 0]
  -B, --bootstrap <ADDR>   Bootstrap peer address in format: <address>/<identity_hex>
  -P, --public             Bootstrap using public Korium network nodes
  -d, --debug              Enable debug logging
  -h, --help               Print help
  -V, --version            Print version
```

## Examples

### Private Network Chat

```bash
# Terminal 1: Alice starts the chatroom
six7 --name Alice --room team-standup

# Terminal 2: Bob joins (copy bootstrap string from Alice)
six7 --name Bob --room team-standup --bootstrap "192.168.1.100:45123/abc123..."

# Terminal 3: Charlie joins
six7 --name Charlie --room team-standup --bootstrap "192.168.1.100:45123/abc123..."
```

### Direct Messaging

```bash
# Send a private message using the peer's full 64-character identity
/dm abc123def456789012345678901234567890123456789012345678901234 Hey, private message!
```

### Monitor Network Health

```bash
# View detailed node statistics
/telemetry

# See connected peers
/fabric

# View DHT routing table
/routing
```

## Architecture

six7 is built on Korium's networking stack:

- **GossipSub** — Efficient epidemic broadcast for room messages
- **DHT (Kademlia)** — Distributed peer discovery and routing
- **SmartSock** — Automatic NAT traversal with relay fallback
- **Ed25519** — Cryptographic identities for authentication

## License

MIT License - see [LICENSE](LICENSE) for details.

## Built With

- [Korium](https://korium.io) — Batteries-included adaptive networking fabric
- [Tokio](https://tokio.rs) — Async runtime for Rust
- [Clap](https://clap.rs) — Command line argument parser
