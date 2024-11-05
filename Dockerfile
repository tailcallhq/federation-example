FROM debian:trixie-slim
ARG WUNDER_URL="https://github.com/wundergraph/cosmo/releases/download/router%400.136.1/router-router@0.136.1-linux-amd64.tar.gz"
ARG APOLLO_URL="https://github.com/apollographql/router/releases/download/v1.57.1/router-v1.57.1-x86_64-unknown-linux-gnu.tar.gz"
ARG GRAFBASE_URL="https://github.com/grafbase/grafbase/releases/download/gateway-0.17.0/grafbase-gateway-x86_64-unknown-linux-musl"
ARG TAILCALL_URL="https://github.com/tailcallhq/tailcall/releases/download/v0.124.0/tailcall-x86_64-unknown-linux-gnu"

WORKDIR /usr/src/benchmarks

# Update, upgrade, and install dependencies
RUN apt update && apt upgrade -y && \
    apt install -y hey curl gcc musl-dev build-essential nginx=1.26.0-3+b1 wget && \
    rm -rf /var/lib/apt/lists/*

# Setup Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Copy source code for Rust build
COPY source/Cargo.lock source/Cargo.toml source/*.json .
COPY source/src/main.rs src/main.rs

# Build Rust project
RUN cargo build --release

# Configure NGINX
COPY nginx/nginx.conf /etc/nginx/sites-available/default

# Setup Wundergraph
RUN wget -O wunder.tar.gz "$WUNDER_URL" && \
    tar -xvf wunder.tar.gz && rm wunder.tar.gz && \
    mv router wunder && chmod +x wunder

# Setup Apollo
RUN wget -O apollo.tar.gz "$APOLLO_URL" && \
    tar -xvf apollo.tar.gz && rm apollo.tar.gz && \
    mv dist/router apollo && chmod +x apollo

# Setup Grafbase
RUN wget -O grafbase "$GRAFBASE_URL" && chmod +x grafbase

# Setup Tailcall
RUN wget -O tailcall "$TAILCALL_URL" && chmod +x tailcall

# Copy Configurations and Scripts
COPY configurations/* .
COPY scripts/* .
