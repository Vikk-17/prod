#!/usr/bin/env bash

# enable the debugging mode
set -x

# -e exit if one command fail
# -o if one command fail in the pipeline then the whole pipeline considered as fail
set -eo pipefail

# check if psql is installed in the system
if ! [ -x "$(command -v psql)" ]; then
    echo >&2 "Error: psql is not installed"
    exit 1
fi

# check if sqlx is installed in the system
if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx is not installed"
    echo >&2 "Use: "
    echo >&2 "      cargo install sqlx"
    echo >&2 "to install it."
    exit 1
fi

# check if a custom user has been set otherwise default to 'postgres'
DB_USER="${POSTGRES_USER:=postgres}"
# check if a custom password has been set otherwise default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
# check if a custom database name has been set otherwise default to 'newsletter'
DB_NAME="${POSTGRES_DB:=newsletter}"
# check if a custom port has been set otherwise default to '5432'
DB_PORT="${POSTGRES_PORT:=5432}"
# check if a custom host has been set otherwise default to 'localhost'
DB_HOST="${POSTGRES_HOST:=localhost}"

# Allow to skip docker if dockerized postgres database is already running
if [[ -z '${SKIP_DOCKER}' ]]
then
    # Launch postgres using docker
    docker run \
        -e POSTGRES_USER=${DB_USER} \
        -e POSTGRES_PASSWORD=${DB_PASSWORD} \
        -e POSTGRES_DB=${DB_NAME} \
        -p "${DB_PORT}":5432 \
        -d postgres \
        postgres -N 1000
    # increase maximum number of connections for testing purpose by default N 100
fi

# keep pinging until and unless postgres is healthy and ready to take connections
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres"
    -c '\q'; do
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1
done
>&2 echo "Postgres is up and running on port ${DB_PORT} - running
migrations now!"

DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
export DATABASE_URL
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated"
