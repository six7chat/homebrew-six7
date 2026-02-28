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
six7

Nickname : Alice
Room     : dev
Address  : 192.168.1.100:4433

Your Identity (for DMs):
abc123def456...

Bootstrap string (copy this line):
192.168.1.100:4433/abc123def456...
```

### Join via a Specific Peer

```bash
# Join using bootstrap string from another peer
six7 --name Bob --room dev --join "192.168.1.100:4433/abc123def456..."
```

### Join Public Korium Network

```bash
# Bootstrap from public Korium network nodes
six7 --name Charlie --room dev --bootstrap
```

## Commands

| Command | Description |
|---------|-------------|
| `/dm <identity> <message>` | Send a direct message to a peer |
| `/contact <identity>` | Send a contact request |
| `/peers` | List known peers from room messages |
| `/list` | Show all peer tables (fabric/transport/routing/gossipsub/dht) |
| `/telemetry` | Show node statistics |
| `/help` | Show available commands |
| `/quit` | Exit the chatroom |

## CLI Options

```
Options:
  -n, --name <NAME>        Your display name [default: anon]
  -r, --room <ROOM>        Room to join [default: lobby]
  -p, --port <PORT>        Port to bind to (0 for random) [default: 0]
  -j, --join <ADDR>        Join a specific peer: <address>/<identity_hex>
  -B, --bootstrap          Bootstrap from public Korium network
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
six7 --name Bob --room team-standup --join "192.168.1.100:45123/abc123..."

# Terminal 3: Charlie joins
six7 --name Charlie --room team-standup --join "192.168.1.100:45123/abc123..."
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

# See all peer tables
/list
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
