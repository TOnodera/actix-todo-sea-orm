FROM rust:latest

RUN useradd -m -u 1000 rust -s /bin/bash
RUN rustup component add rustfmt
RUN cargo install cargo-watch diesel_cli
RUN chown -R rust:rust /usr/local/cargo
USER rust