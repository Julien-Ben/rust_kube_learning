FROM rust:latest
WORKDIR /Github/rust_kube_learning
COPY . .
RUN cargo install --path .
CMD ["rust_kube_learning"]
