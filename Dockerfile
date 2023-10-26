FROM rust:latest as builder
WORKDIR /Github/rust_kube_learning
COPY . .
RUN cargo install --path .
FROM debian:bookworm
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/rust_kube_learning /usr/local/bin/rust_kube_learning
CMD ["rust_kube_learning"]
