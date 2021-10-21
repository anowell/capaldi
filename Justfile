set dotenv-load := true
set shell := ["bash", "-uc"]

# Initialize DB schema and seed data
db-setup:
  sqlx database setup
  sqlite3 --bail ${DATABASE_PATH} < db/seed.sql

# Run DB migrations
db-migrate:
  sqlx migrate

# Destroy and reinitialize DB schema and seed data
db-reset:
  sqlx database reset
  sqlite3 --bail ${DATABASE_PATH} < db/seed.sql

# Run a specific SQL file
db-run file:
  sqlite3 --bail ${DATABASE_PATH} < {{file}}

# Start a sqlite shell to the DB
db-shell:
  sqlite3 --bail --column ${DATABASE_PATH}

# Execute a specific SQL command
db-exec sql:
  sqlite3 --bail --column ${DATABASE_PATH} <<< '{{sql}}'

# Login to the dev server with curl, saving session cookie for subsequent requests
login:
  curl -c cookies.txt -X POST localhost:8000/session/login

# Logout of dev server with curl
logout:
  curl -b cookies.txt -c cookies.txt -X POST localhost:8000/session/logout

# Curl a path with session cookies (e.g. from the login command)
curl PATH *ARGS:
  curl -b cookies.txt -c cookies.txt {{ARGS}} localhost:8000{{PATH}}

# Start the dev server (watch, rebuild, and restart server)
dev-server:
  cargo watch --why --ignore frontend -x run

# Start the frontend build (watch and rebuild)
dev-client:
  cd frontend && npm run dev
