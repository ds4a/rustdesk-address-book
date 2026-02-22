# RustDesk Address Book Server

A complete, self-hosted [RustDesk](https://rustdesk.com) server stack — open-source and free. The included Docker Compose file runs the full suite: the OSS rendezvous server (`hbbs`), relay server (`hbbr`), and this address book server, which adds the address book API and web admin console that RustDesk Pro normally charges for.

## Why?

The RustDesk OSS server handles peer discovery and relay, but has no address book, no user accounts, and no web console — those are Pro-only features. The RustDesk client always tries to connect to port 21114 for address book sync; this server implements that API for free.

## Features

- **Address book sync** — peers, tags, and tag colours sync directly with the RustDesk desktop/mobile client
- **Personal & shared address books**
- **User and group management**
- **Device tracking** — online/offline status via heartbeat
- **Audit logging** — connection and login events
- **Web admin console** — dark/light mode, matches RustDesk's UI style
- **Single binary** — frontend embedded, no separate web server needed
- **SQLite** — no external database required
- **Docker Compose** — one command to run everything

## Quick Start

### Option A — Full stack (new deployment)

Use this if you don't already have a RustDesk server. The Docker Compose file runs everything: rendezvous (`hbbs`), relay (`hbbr`), and the address book server.

**Prerequisites:** Docker and Docker Compose installed.

```bash
git clone https://github.com/ds4a/rustdesk-address-book.git
cd rustdesk-address-book
```

Create a `.env` file with your secrets:

```bash
cat > .env <<EOF
JWT_SECRET=$(openssl rand -hex 32)
ADMIN_PASSWORD=your-secure-password
EOF
```

Start everything:

```bash
docker compose up -d
```

This starts three containers:

| Container | Ports | Purpose |
|-----------|-------|---------|
| `rustdesk-hbbs` | 21115, 21116/tcp+udp, 21118 | Rendezvous / ID server |
| `rustdesk-hbbr` | 21117, 21119 | Relay server |
| `rustdesk-address-book` | 21114 | Address book API + web console |

### Option B — Address book only (existing RustDesk server)

Use this if you already have `hbbs`/`hbbr` running. Just run the address book container on the same host — it's completely independent of the relay/rendezvous servers.

```bash
docker run -d \
  --name rustdesk-address-book \
  --restart unless-stopped \
  -p 21114:21114 \
  -v $(pwd)/data/ab:/data \
  -e RUSTDESK_AB_JWT_SECRET=$(openssl rand -hex 32) \
  -e RUSTDESK_AB_ADMIN_PASSWORD=your-secure-password \
  ghcr.io/ds4a/rustdesk-address-book:latest
```

Or add just the `address-book` service to your existing `docker-compose.yml`:

```yaml
services:
  address-book:
    image: ghcr.io/ds4a/rustdesk-address-book:latest
    # To build locally instead: build: /path/to/rustdesk-address-book
    container_name: rustdesk-address-book
    ports:
      - "21114:21114"
    volumes:
      - ./data/ab:/data
    environment:
      - RUSTDESK_AB_JWT_SECRET=your-secret-here
      - RUSTDESK_AB_ADMIN_PASSWORD=your-password-here
    restart: unless-stopped
```

Open the web console at **http://your-server:21114** and log in with `admin` / your `ADMIN_PASSWORD`.

> **First run:** A default admin user is created automatically on startup if no users exist.

## Configuring the RustDesk Client

In the RustDesk client, go to **Settings → Network** and set:

- **ID Server**: `your-server-ip` (or hostname)
- **API Server**: `http://your-server-ip:21114`
- **Relay Server**: `your-server-ip`

Leave the Key field empty unless you have configured one on `hbbs`.

Then log in to your account from the client's address book panel — your peers and tags will sync automatically.

## Configuration

All settings can be provided via environment variables or a `config.toml` file (copy `config.toml.example`).

| Environment Variable | Config Key | Default | Description |
|----------------------|------------|---------|-------------|
| `RUSTDESK_AB_PORT` | `port` | `21114` | Port to listen on |
| `RUSTDESK_AB_DB_PATH` | `db_path` | `data/db.sqlite3` | SQLite database path |
| `RUSTDESK_AB_JWT_SECRET` | `jwt_secret` | *(random)* | JWT signing secret — set this in production |
| `RUSTDESK_AB_ADMIN_USERNAME` | `admin_username` | `admin` | Initial admin username (first run only) |
| `RUSTDESK_AB_ADMIN_PASSWORD` | `admin_password` | `admin` | Initial admin password (first run only) |
| `RUSTDESK_AB_TOKEN_EXPIRY_HOURS` | `token_expiry_hours` | `168` | Token lifetime in hours (default 7 days) |

> If `JWT_SECRET` is not set, a random secret is generated each startup — this means all sessions are invalidated on restart. Always set it in production.

## Running Without Docker

**Prerequisites:** Rust 1.75+, Node.js 22+.

```bash
# Build the frontend
cd web
npm install
npx vite build
cd ..

# Build and run the backend (frontend is embedded into the binary)
cargo build --release
./target/release/rustdesk-address-book
```

Or with a config file:

```bash
cp config.toml.example config.toml
# Edit config.toml
./target/release/rustdesk-address-book
```

## Ports Reference

| Port | Protocol | Service | Required? |
|------|----------|---------|-----------|
| 21114 | TCP | Address book API + web console | Yes (this server) |
| 21115 | TCP | hbbs — NAT type detection | Yes |
| 21116 | TCP+UDP | hbbs — peer registration / hole punching | Yes |
| 21117 | TCP | hbbr — relay | Yes |
| 21118 | TCP | hbbs — WebSocket | Optional |
| 21119 | TCP | hbbr — WebSocket | Optional |

Make sure these ports are open in your firewall.

## Data

All persistent data is stored under `./data/`:

```
data/
├── ab/         # Address book SQLite database
├── hbbs/       # hbbs keys and peer database
└── hbbr/       # hbbr configuration
```

Back up this directory to preserve your data.

## API Compatibility

Implements the RustDesk client API on port 21114:

| Endpoint | Description |
|----------|-------------|
| `POST /api/login` | Authenticate, returns Bearer token |
| `POST /api/logout` | Invalidate session |
| `GET /api/currentUser` | Current user info |
| `GET /api/ab/personal` | Get personal address book |
| `GET /api/ab` | Legacy address book fetch |
| `POST /api/ab` | Legacy address book update |
| `GET /api/ab/shared/profiles` | List shared address books |
| `GET /api/ab/peers` | Fetch peers (paginated) |
| `POST /api/ab/peer/add/{guid}` | Add peer |
| `PUT /api/ab/peer/update/{guid}` | Update peer |
| `DELETE /api/ab/peer/{guid}` | Delete peer(s) |
| `GET /api/ab/tags/{guid}` | Fetch tags |
| `POST /api/ab/tag/add/{guid}` | Add tag |
| `PUT /api/ab/tag/rename/{guid}` | Rename tag |
| `PUT /api/ab/tag/update/{guid}` | Update tag colour |
| `DELETE /api/ab/tag/{guid}` | Delete tag(s) |
| `POST /api/heartbeat` | Device heartbeat |
| `POST /api/system/sysinfo` | Report device info |
| `POST /api/audit` | Log audit event |

## License

AGPL-3.0 — same as RustDesk itself.
