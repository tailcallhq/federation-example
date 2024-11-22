FROM debian:trixie-slim

WORKDIR /usr/src/benchmarks

# Update, upgrade, and install dependencies
RUN apt update && apt upgrade -y && \
    apt install -y hey curl gcc musl-dev build-essential nginx=1.26.0-3+b1 wget nodejs npm && \
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

# Install WunderGraph CLI and Router
RUN npm install -g wgc@latest && \
    wgc router download-binary -o . && \
    mv router wunder && \
    chmod +x wunder

# Install Apollo Router
RUN curl -sSL https://router.apollo.dev/download/nix/latest | sh && \
    mv router apollo && chmod +x apollo


# Setup Grafbase
RUN curl -fsSL https://grafbase.com/downloads/gateway | bash && \
    mv ~/.grafbase/bin/grafbase-gateway grafbase && chmod +x grafbase


# Setup Tailcall
RUN npm install -g @tailcallhq/tailcall

# Copy Configurations and Scripts
COPY configurations/* .
COPY scripts/* .
