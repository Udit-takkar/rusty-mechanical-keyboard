# 🎹 Rusty Mechanical Keyboard Sound Simulator

A high-performance Rust application that plays realistic mechanical keyboard sounds when you type, system-wide. Experience the satisfying click and clack of mechanical switches on any keyboard!

## ✨ Features

- **Realistic Sound Effects** - Uses authentic NK Cream mechanical keyboard sounds
- **Low Latency** - Optimized for instant audio response
- **Concurrent Playback** - Multiple audio streams prevent sound queuing
- **System-Wide** - Works across all applications, not just the terminal
- **Cross-Platform** - Supports macOS, Windows, and Linux
- **Auto-Releases** - Automatic builds and releases on every push to main
- **Customizable** - Easy to swap sound packs

## 🚀 Quick Start

### Download Latest Release

1. **Go to [Releases](https://github.com/Udit-takkar/rusty-mechanical-keyboard/releases)**
2. **Download** the appropriate file for your system:
   - **macOS Intel**: `rusty-mechanical-keyboard-intel.tar.gz`
   - **macOS Apple Silicon**: `rusty-mechanical-keyboard-arm64.tar.gz`
   - **macOS Universal**: `rusty-mechanical-keyboard-universal.tar.gz` (works on both)
3. **Extract** the archive: `tar -xzf rusty-mechanical-keyboard-*.tar.gz`
4. **Run**: `./rusty-mechanical-keyboard`
5. **Grant permissions** when prompted

### Build from Source

```bash
# Prerequisites
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/yourusername/rusty-mechanical-keyboard.git
cd rusty-mechanical-keyboard
cargo build --release

# Run
cargo run
```

## 🔧 Installation

### macOS

1. **Download** the latest release from GitHub
2. **Extract** the archive
3. **Grant Accessibility Permission**:
   - Go to **System Preferences** → **Security & Privacy** → **Privacy** → **Accessibility**
   - Add the application to the list
   - Check the box to enable it
4. **Run** the application

## 🎵 Sound Packs

### Included Sound Pack

- **NK Cream** - Authentic mechanical keyboard sounds with individual key variations

### Adding Custom Sound Packs

1. **Create** a folder with your sound pack
2. **Add** a `config.json` file (see example below)
3. **Update** the code to use your sound pack path

#### Example config.json

```json
{
  "id": "my-sound-pack",
  "name": "My Custom Sound Pack",
  "key_define_type": "multiple",
  "includes_numpad": false,
  "sound": "default.wav",
  "defines": {
    "1": "key1.wav",
    "2": "key2.wav",
    "3": "key3.wav"
  }
}
```

## 🛠️ Development

### Prerequisites

- **Rust** 1.70+ ([Install Rust](https://rustup.rs/))
- **Audio system** (ALSA on Linux, Core Audio on macOS, DirectSound on Windows)

### Building

```bash
# Clone repository
git clone https://github.com/yourusername/rusty-mechanical-keyboard.git
cd rusty-mechanical-keyboard

# Build in debug mode
cargo build

# Build in release mode
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

### Project Structure

```
rusty-mechanical-keyboard/
├── src/
│   └── main.rs              # Main application code
├── nk-cream/                # Sound pack directory
│   ├── config.json          # Sound pack configuration
│   └── *.wav               # Audio files
├── .github/
│   └── workflows/
│       └── macos-release.yml # CI/CD pipeline
├── Cargo.toml               # Rust dependencies
└── README.md               # This file
```

## 🚀 Performance

### Optimizations

- **Sound Pool** - 8 concurrent audio streams prevent lag
- **Memory Loading** - All sounds pre-loaded for instant playback
- **Threading** - Separate threads for keyboard detection and audio
- **Efficient Data Structures** - Fast HashMap lookups for sound mapping

### System Requirements

- **RAM**: ~50MB (with sound pack loaded)
- **CPU**: Minimal impact (optimized for low latency)
- **Storage**: ~10MB (including sound pack)

## 🐛 Troubleshooting

### Common Issues

#### "Permission denied" Error

- **macOS**: Grant Accessibility permission in System Preferences
- **Linux**: Add user to audio group: `sudo usermod -a -G audio $USER`
- **Windows**: Run as Administrator if needed

#### No Sound Playing

- **Check audio output** - Ensure speakers/headphones are working
- **Check volume** - Make sure system volume is up
- **Check permissions** - Verify audio permissions are granted

#### High CPU Usage

- **Update Rust** - Ensure you're using the latest stable version
- **Check sound files** - Corrupted audio files can cause issues
- **Restart application** - Sometimes a restart helps

### Debug Mode

```bash
# Run with debug logging to see what's happening
RUST_LOG=debug cargo run
```

## 🤝 Contributing

We welcome contributions! Here's how you can help:

### Ways to Contribute

- 🐛 **Report bugs** - Open an issue with details
- 💡 **Suggest features** - Share your ideas
- 🔧 **Submit pull requests** - Fix bugs or add features
- 📚 **Improve documentation** - Help others understand the project
- 🎵 **Add sound packs** - Create new keyboard sound variations

## 🔄 Changelog

### v1.0.0

- Initial release
- NK Cream sound pack integration
- System-wide keyboard listening
- Multi-platform support
- Auto-release CI/CD pipeline

---

**Made with ❤️ and Rust** 🦀

_Enjoy the satisfying sound of mechanical keyboards!_ ⌨️✨
