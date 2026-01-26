# bevy-scaffold

A minimal **Bevy** project scaffold with shell scripts to spin up a reproducible development container using Distrobox. 

This repository is meant as a starting point for new Bevy projects, giving you:

- A clean, opinionated layout for a Bevy app.
- Host–container integration via Distrobox for a stable Rust and Bevy toolchain.
- Shell scripts to bootstrap and configure the dev environment (`init_distrobox.sh`, `setup_inside_distrobox.sh`).
---

## Features

- Bevy-based Rust application scaffold (main crate plus space for plugins/systems/assets).  
- Distrobox-powered development environment, so you can keep your host system clean.   
- Scripted initialization to install Rust, system dependencies, and Bevy build requirements in the container. 

---

## Prerequisites

On the host (your main OS), you should have:

- A compatible Linux distribution. [  
- `distrobox` installed and working (`distrobox-create` / `distrobox-enter` available). 
- `podman` or `docker`, depending on your Distrobox backend. 
- `git` to clone this repository. 

---

## Getting started

Clone the repository:

### git clone https://github.com/vlodbolv/bevy-scaffold.git
### cd bevy-scaffold

Initialize the Distrobox container:
### ./init_distrobox.sh

This script is expected to:

Create (or update) a dedicated dev container for this project.

Mount the repo into the container for tight host–container integration. 

Mount the repo into the container for tight host–container integration. 
e.g.
### distrobox enter bevy-dev

Then, inside the container, run:

### ./setup_inside_distrobox.sh

This script is expected to:

Install Rust (via rustup) and required toolchain components. 

Install system dependencies needed for Bevy (graphics, audio, windowing libraries, etc. for your distro).

Run cargo build once so subsequent builds are faster.

Finally, to run the Bevy app from inside the container:

### cargo run

Project structure
The repo is intended to follow a conventional Bevy layout. 
bevy-scaffold/
├─ hello/
│  ├─ Cargo.toml        # Rust crate and Bevy dependency configuration
│  └─ src/
│     └─ main.rs        # Application entry point, Bevy App setup
├─ init_distrobox.sh             # Host-side script to create/enter container
├─ setup_inside_distrobox.sh     # In-container setup script
└─ README.md

You can replace the hello/src/ directory with additional modules (for example: systems.rs, plugins.rs) as your Bevy project grows. 

Common workflows
Rebuild and run after code changes:
# cd hello
# cargo run

Format and lint:
# cargo fmt
# cargo clippy

Customizing the scaffold
Once the environment works, you can:

Add new Bevy plugins / systems under hello/src/. 

Extend setup_inside_distrobox.sh to install extra tooling (for example: cargo-watch, just, additional libraries).

Adjust init_distrobox.sh to change base image, container name, or mounted paths.

Turn the repository into a workspace later by adding a root Cargo.toml with a [workspace] section if you introduce additional crates.
