#!/bin/bash
# Bevy-Rust Development Environment Setup
# Run this script INSIDE the distrobox container

set -e

echo "=== Bevy-Rust Development Environment Setup ==="
echo "Installing dependencies for Bevy development..."
echo ""

# Update system
echo "Updating system packages..."
sudo dnf update -y

# Install Rust development tools and Bevy dependencies
echo "Installing development tools and libraries..."
sudo dnf install -y \
  gcc \
  gcc-c++ \
  make \
  cmake \
  pkg-config \
  openssl-devel \
  alsa-lib-devel \
  libudev-devel \
  libX11-devel \
  libXcursor-devel \
  libXrandr-devel \
  libXi-devel \
  mesa-libGL-devel \
  vulkan-loader \
  vulkan-headers \
  vulkan-validation-layers \
  wayland-devel \
  libxkbcommon-devel \
  libxkbcommon-x11-devel \
  libxkbcommon-x11

echo ""
echo "Installing Rust toolchain..."

# Install Rust using rustup
if ! command -v rustup &> /dev/null; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source "$HOME/.cargo/env"
  echo "Rust installed successfully!"
else
  echo "Rust already installed, updating..."
  rustup update stable
fi

# Ensure stable toolchain is default
rustup default stable

echo ""
echo "=== Setup Complete ==="
echo ""
echo "Rust version: $(rustc --version)"
echo "Cargo version: $(cargo --version)"
echo ""
echo "Your Bevy development environment is ready!"
echo ""
echo "Next steps:"
echo "  1. Navigate to your project directory: cd ~/Code/bevy-rust"
echo "  2. Create a new project: cargo new my-bevy-game"
echo "  3. Add Bevy dependency and start coding!"
echo ""
echo "To test the environment, you can create the minimal test app"
echo "provided in the setup documentation."