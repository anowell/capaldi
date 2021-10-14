Capaldi
------

CAPacity ALlocation & DIagrams


Config:
- Jira epic query



### Dev Setup

Install sqlite, just, and sqlx (w/ sqlite feature)

```shell
apt install sqlite3 libsqlite-dev
cargo install just
cargo install sqlx --no-default-features --features sqlite
```

```shell
# First time:
just db-setup

# To reset database:
just db-reset
```

Build

```shell
# source .env to set DATABASE_URL (used by compile time schema type checking)
source .env
cargo c
```

Starting capaldi backend:

```shell
source .env
cargo run
curl -X POST -b ~/cookies.txt localhost:8000/session/login
curl -b ~/cookies.txt localhost:8000/groups
```
