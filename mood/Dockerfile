FROM rust:latest
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release
EXPOSE 8071
CMD ["cargo", "run", "--release"]