#! /bin/bash
#
# ------------
# Instructions
# ------------
# source ./scripts/database-url.sh

export $(cat ./env/db.dev.env | xargs)
export DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${DB_HOST_NAME}:${DB_PORT_HOST}/${POSTGRES_DB}"
