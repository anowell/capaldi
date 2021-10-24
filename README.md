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

Install sqlite, just, cargo-watch, and sqlx (w/ sqlite feature)

```shell
apt install sqlite3 libsqlite-dev
cargo install just cargo-watch
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

Starting capaldi dev server:

```shell
just dev-server

# in a separate console
just login
just curl /groups
```

Starting capaldi frontend:

```shell
just dev-client

# Open localhost:8000 in browser
```

### Tech Stack

Backend (Rust):

- [Rocket](https://rocket.rs/) - Web Framework
- [SqlX](https://github.com/launchbadge/sqlx) - SQL Toolkit
- [SQLite](https://sqlite.org) - DB

Frontend (TypeScript):
- [Svelte](https://svelte.dev/) - Web Framework
- [Bulma](https://bulma.io/) - CSS Framework
- [Ionicons](https://ionic.io/ionicons/v4) - Icons
- [Svelte-query](https://sveltequery.vercel.app/) - Data Syncronization
- [Axios](https://axios-http.com/) - HTTP Client

Tool:
- [Just](https://github.com/casey/just) - Task runner
- [Rollup.js](https://www.rollupjs.org) - Bundler

### Design Philosophy

Design decisions are guided by a few principles:

- Outcomes before features
- Opinionated before flexible
- Simple before feature-rich
- Fast before shiny
- Delightful before powerful

While the things on the right are desirable, the things on the left take precedence.
