#! /bin/bash
#
# ------------
# Instructions
# ------------
# source ./scripts/app.local.sh
#
# This file exists to set environment variable in the local shell. When we want
# to run the backend binary locally and not inside of Docker, we use this script
# to populate the necessary variables for the application.
export $(cat ./env/db.dev.env | xargs)
export $(cat ./env/app.dev.env | xargs)
export APP_HOST_NAME="${APP_HOST_NAME}"
export APP_PORT="${APP_PORT_HOST}"
export DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${DB_HOST_NAME}:${DB_PORT_HOST}/${POSTGRES_DB}"
