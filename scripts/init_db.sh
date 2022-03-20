#!/usr/bin/env bash

set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
    echo >&2 "Error: psql is not installed."
    exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx is not installed."
    echo >&2 "Use:"
    echo >&2 "    cargo install --version=0.5.5 sqlx-cli --no-default-features --features postgres"
    echo >&2 "to install it."
    exit 1
fi

if [ ! -f ".env" ]; then
    echo >&2 "File .env not found"
    echo >&2 "Using defaults"
    DB_USER="postgres"
    DB_PASSWORD="password"
    DB_NAME="blog_db"
    DB_PORT="5432"
    DB_HOST="localhost";
else
    source .env
fi

if [[ -z "${SKIP_DOCKER}" ]]
then
    RUNNING_POSTGRES_CONTAINER=$(docker ps --filter 'name=postgres' --format '{{.ID}}')
    if [[ -n $RUNNING_POSTGRES_CONTAINER ]]; then
        docker kill ${RUNNING_POSTGRES_CONTAINER}
    fi

    docker run \
        -e POSTGRES_USER=${DB_USER} \
        -e POSTGRES_PASSWORD=${DB_PASSWORD} \
        -e POSTGRES_DB=${DB_NAME} \
        -p "${DB_PORT}":5432 \
        -d \
        --name "postgres_$(date '+%s')" \
        postgres -N 1000
fi

until PGPASSWORD="${DB_PASSWORD}" psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT} - running migrations now!"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run 

>&2 echo "Postgres has been migrated, ready to go!"
