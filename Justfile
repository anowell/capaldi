set dotenv-load := true
set shell := ["bash", "-uc"]

db-setup:
  sqlx database setup
  sqlite3 --bail ${DATABASE_PATH} < db/seed.sql

db-migrate:
  sqlx migrate
  # sql database prepare

db-reset:
  sqlx database reset
  sqlite3 --bail ${DATABASE_PATH} < db/seed.sql

db-run file:
  sqlite3 --bail ${DATABASE_PATH} < db/{{file}}.sql

db-shell:
  sqlite3 --bail --column ${DATABASE_PATH}

db-exec sql:
  sqlite3 --bail --column ${DATABASE_PATH} <<< '{{sql}}'

login:
  curl -c cookies.txt -X POST localhost:8000/session/login

logout:
  curl -b cookies.txt -c cookies.txt -X POST localhost:8000/session/logout

curl PATH *ARGS:
  curl -b cookies.txt -c cookies.txt {{ARGS}} localhost:8000{{PATH}}
