# ----------------------------------------------------------------
# Step 0 - Install cargo plugins
# ----------------------------------------------------------------
# We only pay the installation cost once,
# it will be cached from the second build onwards
# - trunk:      our WASM build tool
# - cargo-chef: efficiently makes docker cache our build process
# - cargo-make: scrtipt runner through cargo to build and start our apps
#
# 2022-04-26: We choose Rust version 1.58 because at the time of writing,
# the rust wasm / yew frontend app does not compile on 1.6
#
FROM rust:1.64 AS chef

USER root

# We need to make sure we have the WASM target in our toolchain
# to compile for our Yew app
RUN rustup target add wasm32-unknown-unknown

RUN cargo install --locked \
  cargo-chef \
  cargo-make \
  trunk

RUN cargo install --locked \
  sqlx-cli \
  --no-default-features \
  --features rustls,postgres

# Step 1 - Cargo chef prepare
# ----------------------------------------------------------------
FROM chef as planner
WORKDIR /app

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

# ----------------------------------------------------------------
# Step 2 - Cargo chef build (cook)
# ----------------------------------------------------------------
#
FROM chef AS builder
WORKDIR /app

COPY --from=planner /app/recipe.json recipe.json

# We want SQLX_OFFLINE because we cannot depend on the database at this stage
RUN SQLX_OFFLINE=true cargo chef cook \
  --release \
  --recipe-path recipe.json

COPY . .

# Build the backend binary
RUN SQLX_OFFLINE=true cargo build \
  --package backend \
  --release \
  --bin backend

# Build frontend source code. The compiled WASM and HTML file will be served by
# the backend...
# TODO: @ Jacob - Ensure that the backend actually serves the frotnend files
# cargo make `build-web` calls trunk internally
RUN cargo make \
  -p prod \
  build-web

# ----------------------------------------------------------------
# Step 3 - Copy executable to runtime image
# ----------------------------------------------------------------
FROM debian:buster-slim AS runner
WORKDIR /app

# I think that every time we change the image we need to re-define the workdir
#
# Create directories for the following env variables
# APP_VOL_COMMON_TARGET
# APP_VOL_FRONT_TARGET
# APP_VOL_BACK_TARGET
RUN mkdir -p common frontend backend

# Executable will be placed at /app/backend
#
# builder:/app/target/release/backend -> runer:/usr/local/bin/backend
COPY --from=builder \
  /app/target/release/backend \
  /usr/local/bin/backend

# Move static assets from the builder image to the runner image
#
# builder:/app/backend/static/* -> runner:/app/static
RUN mkdir -p static
COPY --from=builder \
  /app/backend/static \
  static
