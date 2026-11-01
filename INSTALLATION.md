# Installation

## System Requirements

| Component | Requirement |
|-----------|-------------|
| Rust | 1.78+ |
| Linux | libwebkit2gtk-4.0-dev, libgtk-3-dev |
| Windows | MSVC toolchain |
| Android | Android SDK, NDK r26b, JDK 17 |

## Linux

### Dependencies

```bash
# Debian/Ubuntu
sudo apt-get update
sudo apt-get install libwebkit2gtk-4.0-dev libgtk-3-dev

# Fedora
sudo dnf install webkit2gtk3-devel gtk3-devel

# Arch
sudo pacman -S webkit2gtk gtk3
```

### Build and Install

```bash
git clone https://github.com/aghorbani84/Fazora.git
cd Fazora
cargo build --release
```

The binary is at `target/release/fazora`.

### System Integration

FAZORA installs as a standalone binary. Desktop integration (file associations,
MIME types, desktop entry) is platform-dependent and should be configured
manually or via a package manager.

## Windows

### Prerequisites

- Rust with MSVC toolchain: `rustup default stable`
- Windows SDK

### Build and Install

```bash
git clone https://github.com/aghorbani84/Fazora.git
cd Fazora
cargo build --release
```

The binary is at `target/release/fazora.exe`.

## Android

### Prerequisites

- Android SDK (command-line tools)
- Android NDK r26b
- JDK 17
- Rust Android targets:

```bash
rustup target add aarch64-linux-android x86_64-linux-android
```

### Build

```bash
git clone https://github.com/aghorbani84/Fazora.git
cd Fazora
cargo build --release --target aarch64-linux-android
```

The native library is at `target/aarch64-linux-android/release/libfazora.so`.

APK packaging requires Dioxus mobile toolchain configuration.

## Verification

After building, verify the application starts:

```bash
# Linux
./target/release/fazora

# Windows
.\target\release\fazora.exe
```

The application window should open showing the FAZORA home dashboard.

## Troubleshooting

| Issue | Solution |
|-------|----------|
| `libwebkit2gtk-4.0 not found` | Install `libwebkit2gtk-4.0-dev` |
| `linker `cc` not found` | Install build-essential / gcc |
| Android target not found | Run `rustup target add aarch64-linux-android` |
| `cargo: command not found` | Install Rust from https://rustup.rs/ |
