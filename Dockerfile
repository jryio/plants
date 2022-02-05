# Step 0 - Install cargo plugins
# ----------------------------------------------------------------
# We only pay the installation cost once,
# it will be cached from the second build onwards
# - trunk:      our WASM build tool
# - cargo-chef: efficiently makes docker cache our build process
FROM rust:slim as chef

RUN cargo install --locked \
  cargo-chef \
  trunk

# Workdir refers to the directory path within the docker container
# The path comes from .env/common
# Example: "/app"
WORKDIR "/${WORKDIR}"

# Step 0 - Install and build tailwindcss
# ----------------------------------------------------------------
FROM node:slim as tailwind
# Only copy over the frontend code
COPY frontend frontend
RUN mkdir -p output
RUN echo "tailwind -> Copied over frontend code"

# Install dependencies
RUN echo "tailwind -> Installing dependencies"
RUN npm install -g \
  tailwindcss@latest \
  tailwindcss-cli@latest \
  postcss@latest \
  autoprefixer@latest \
  clean-css-cli@latest \
  npm-run@latest
RUN ls -la

RUN echo "tailwind -> Running npx tailwindcss-cli@latest"
RUN npx tailwindcss-cli@latest build \
  frontend/styles/index.css \
  --config frontend/tailwind.config.js \
  --minify --output output/tailwind.big.css

RUN echo " tailwind -> running cleancss on output"
RUN rm output/tailwind.css && \
  npx clean-css-cli@latest output/tailwind.big.css  -o output/tailwind.css && \
  rm output/tailwind.big.css

# Step 1 - Cargo chef prepare
# ----------------------------------------------------------------
FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Step 2 - Cargo chef build (cook)
# ----------------------------------------------------------------
FROM chef as builder
COPY --from=planner "/${WORKDIR}/recipe.json" recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Copy source code
COPY . .
# Build the frontend bundle

# Build the backend binary
RUN cargo build --release backend  # Adding the --release flag to cargo will cutdown on size
