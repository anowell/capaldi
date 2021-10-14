set dotenv-load := true

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

db-exec sql:
  sqlite3 --bail --column ${DATABASE_PATH} <<< {{sql}}


