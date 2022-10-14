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
FROM rust:1.58 AS chef

USER root

# We need to make sure we have the WASM target in our toolchain
# to compile for our Yew app
RUN rustup target add wasm32-unknown-unknown

RUN cargo install --locked \
  cargo-chef \
  cargo-make \
  trunk

# TODO @ jacob - Might not need this
RUN cargo install --locked \
  sqlx-cli \
  --no-default-features \
  --features rustls,postgres

WORKDIR /app

# Step 1 - Cargo chef prepare
# ----------------------------------------------------------------
FROM chef as planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

# Step 2 - Cargo chef build (cook)
# ----------------------------------------------------------------
#
FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json

RUN SQLX_OFFLINE=true cargo chef cook \
  --release \
  --recipe-path recipe.json

# Copy entire project source code
COPY . .

# Build the frontend
RUN cargo make \
  -p prod \
  build-web

# Build the backend binary
RUN cargo build \
  --release \
  --bin backend

# Step 3 - Copy executable to runtime image
# ----------------------------------------------------------------
FROM debian:buster-slim AS runner

# I think that every time we change the image we need to re-define the workdir
#
# Create directories for the following env variables
# APP_VOL_COMMON_TARGET
# APP_VOL_FRONT_TARGET
# APP_VOL_BACK_TARGET
WORKDIR /app
RUN mkdir -p common frontend backend

# Executable will be placed at /app/backend
COPY --from=builder \
  /app/target/release/backend \
  /usr/local/bin/backend

# Move static assets to runner image
RUN mkdir -p static
COPY --from=builder \
  /app/backend/static \
  static
