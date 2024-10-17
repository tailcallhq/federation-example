FROM rust:latest
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release
EXPOSE 8081
CMD ["cargo", "run", "--release"]