#! /bin/bash
export $(cat ./env/db.dev.env | xargs)
export $(cat ./env/app.dev.env | xargs)
export APP_HOST_NAME="${APP_HOST_NAME}"
export APP_HOST_PORT="${APP_PORT_HOST}"
export DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${DB_HOST_NAME}:${DB_PORT_HOST}/${POSTGRES_DB}"

./target/release/backend
