Capaldi
------

CAPacity ALlocation & DIagrams

Note: allocation categories are generic, but generally thought of as areas like: product enhancements, reactive work, complexity management, etc)

Capaldi aims to be able to quickly answer these questions:
- Individual
  - How is the individuals time committed across allocation categories?
- Team
  - How are the team's resources distributed across allocation categories?
  - How are the team's resources distributed across projects and releases?
- Org
  - How are the org's resources distributed across allocation categories?
  - How are the org's resources distributed across projects and releases?
  - How are the org's resources distributed over various time intervals (e.g. quarters)?
  - How are the org's resources distributed w.r.t. resource attributes such as role or FTE status?


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

# in a separate console
just login
just curl /groups
```
