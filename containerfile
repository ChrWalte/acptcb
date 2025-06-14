
FROM rust:1-bookworm AS builder

WORKDIR /src/acptcb

# install dependencies only
COPY Cargo.toml ./
RUN rustup target add x86_64-unknown-linux-musl
RUN mkdir -p ./src && \
    echo "fn main() {}" > ./src/main.rs && \
    cargo build --release

# build source code
COPY ./src ./src
RUN cargo build --target x86_64-unknown-linux-musl --release

# ---

# FROM docker:dind-rootless AS runtime
FROM docker:dind AS runtime

# copy the binary from the builder stage
COPY --from=builder /src/acptcb/target/x86_64-unknown-linux-musl/release/acptcb /usr/local/bin/acptcb
RUN chmod +x /usr/local/bin/acptcb

# copy the entrypoint script
COPY ./entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

ENTRYPOINT ["sh", "/entrypoint.sh"]
