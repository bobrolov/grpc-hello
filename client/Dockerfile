FROM rust:latest

RUN rustup component add rustfmt

WORKDIR /usr/src/client

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN cargo build --release

RUN rm -f target/release/deps/client*

COPY . .

RUN cargo build --release

RUN cargo install --path .

CMD ["/usr/local/cargo/bin/client"]