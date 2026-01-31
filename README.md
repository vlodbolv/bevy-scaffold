# bevy-scaffold

A minimal **Bevy 0.18** project scaffold designed for reproducible development using **Distrobox**. This repository provides a deterministic environment for high-quality video rendering and general Bevy development.

---

## 🌟 Features

* **Bevy 0.18 + Rust:** Pre-configured for the latest ECS standards (Required Components, `Mesh3d`, `MeshMaterial3d`).
* **Deterministic Rendering:** Built-in logic to export 30 FPS video frames without simulation "jitter," regardless of hardware lag.
* **Distrobox Integration:** Host-container workflow keeps your main OS clean while providing all necessary Vulkan, X11, and Wayland headers.
* **FFmpeg Ready:** Pre-configured with **RPM Fusion** to support H.264/MP4 encoding directly from your container.

---

## 📋 Prerequisites

On your **host machine**, you need:

* **Distrobox** installed.
* **Podman** (preferred) or **Docker**.
* **NVIDIA/AMD/Intel Drivers** (Vulkan support is required for Bevy).

---

## 🚀 Getting Started

### 1. Initialize the Environment

Clone the repository and spin up the container:

```bash
git clone https://github.com/vlodbolv/bevy-scaffold.git
cd bevy-scaffold

# Create the container (bevy-dev)
./init_distrobox.sh

# Enter the container
distrobox enter bevy-dev

```

### 2. Run Internal Setup

Once inside the container, install Rust and the system dependencies (including FFmpeg with H.264 support):

```bash
./setup_inside_distrobox.sh

```

### 3. Run and Render

Navigate to the project and run the deterministic render:

```bash
cd hello
cargo run

```

The application will render exactly **5 seconds (150 frames)** of animation and automatically close.

---

## 🎬 Video Export Workflow

This scaffold is designed to output a PNG sequence into the `hello/out/` directory. To convert these frames into a high-quality MP4, run the following inside your Distrobox:

```bash
ffmpeg -r 30 -i out/%05d.png -vcodec libx264 -crf 18 -pix_fmt yuv420p output.mp4

```

---

## 📂 Project Structure

```text
bevy-scaffold/
├── hello/
│   ├── Cargo.toml         # Configured for Bevy 0.18 & bevy_image_export 0.16
│   ├── src/
│   │   └── main.rs        # 30 FPS deterministic logic + Writer<AppExit>
│   └── out/               # Generated PNG frames (ignored by git)
├── init_distrobox.sh      # Host-side: creates the Fedora-based container
├── setup_inside_distrobox.sh # Container-side: installs Rust, Vulkan, & FFmpeg
└── README.md

```

---

## ⚙️ Technical Notes

### Deterministic Time

The scaffold utilizes `Time<Fixed>` and the `FixedUpdate` schedule. This ensures that every exported frame represents exactly  of a second of simulation time, even if the CPU takes several seconds to write a single PNG to disk.

### Bevy 0.18 Standards

The included `main.rs` uses the modern 0.18 API:

* **`Mesh3d` / `MeshMaterial3d**` instead of deprecated `PbrBundle`.
* **`Writer<AppExit>`** for clean system termination.
* **`delta_secs()` / `elapsed_secs()**` for standardized time access.

---

## 🛠 Troubleshooting

**Container won't stop?**
If Distrobox refuses to stop due to "active sessions," force it from your host:

```bash
podman stop -f bevy-dev

```

**Missing Codecs?**
If FFmpeg fails to recognize `libx264`, ensure `setup_inside_distrobox.sh` successfully added the **RPM Fusion Nonfree** repositories.

