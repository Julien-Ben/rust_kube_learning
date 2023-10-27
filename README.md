# rust_kube_learning

## Goal

For learning purpose.
Deploying a toy web server written in Rust, working with MongoDB, on a Kubernetes cluster.

## Features

* Automated Deployment
* Build with Docker
* Auto-scaling
* Load balancing
* Automatic deployment and configuration of the database

## Description

To be completed

## Getting Started

### Environment variables

Create you own `.env` file following the `.env.example` template

### Prerequisites

* Rust
* If running without Kubernetes : a MongoDB database, with its `MONGOURI` specified in `.env` file

For deployment:

* Docker CLI
* `kubectl`
* Kubernetes cluster with `$KUBECONFIG` environment variable configured

Most prerequisites can be installed with the Makefile `prerequisites` rule

### Running

```
cargo run
```

API available on port `8080`
