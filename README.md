# 🕺 Dancing Grandpa - Baby Entertainment System

A delightful baby entertainment system built with the Bevy game engine in Rust. Features animated dancing characters synchronized to music, designed to captivate and entertain babies and toddlers.

## Features

- **Multiple animated dancing grandpa characters** - Configurable number of dancers
- **Tempo-synchronized animations** - Characters dance in time with the music
- **Configurable system** - Adjust number of dancers, music tempo, and more via JSON config
- **Automatic restart loop** - Seamless entertainment with fade effects between cycles
- **Cross-platform** - Available as both desktop and web applications
- **Built with Rust & Bevy** - High performance and reliability

## Quick Start

### Desktop Version

```bash
# Clone the repository
git clone https://github.com/ianwalkeruk/dancing-grandpa.git
cd dancing-grandpa

# Run the desktop application
cargo run --bin desktop
```

### Web Version

```bash
# Build the web version
cd website
wasm-pack build --target web --out-dir pkg

# Serve the web application
python3 -m http.server 8000

# Open http://localhost:8000 in your browser
```

## Project Structure

This is a Cargo workspace containing three packages:

- **`common/`** - Shared library code for both desktop and web versions
- **`desktop/`** - Native desktop application using Bevy
- **`website/`** - WebAssembly web application using Bevy + wasm-bindgen

## Configuration

The system uses a JSON configuration file to customize the experience:

```json
{
  "images": [
    "assets/grandpa1.png",
    "assets/grandpa2.png", 
    "assets/grandpa3.png"
  ],
  "audio_file": "assets/dance_music.wav",
  "tempo_bpm": 120,
  "num_dancers": 5,
  "fade_duration_secs": 2.0,
  "silence_duration_secs": 3.0
}
```

## Assets

The system includes placeholder assets:
- 3 dancing grandpa character images
- Upbeat dance music in WAV format
- All assets are located in the `assets/` directory

## Development

### Prerequisites

- Rust 1.70+ with Cargo
- For web builds: `wasm-pack` and a modern web browser
- For desktop builds: System audio libraries (ALSA on Linux, etc.)

### Building

```bash
# Desktop version
cargo build --bin desktop

# Web version  
cd website
wasm-pack build --target web --out-dir pkg
```

### Running Tests

```bash
cargo test
```

## Technical Details

- **Engine**: Bevy 0.14 game engine
- **Audio**: Bevy's audio system with WAV support
- **Graphics**: 2D sprite rendering with WebGL2 support for web
- **Animation**: Custom tempo-based animation system
- **WASM**: Full WebAssembly support with proper getrandom configuration

## Browser Compatibility

The web version supports modern browsers with WebAssembly and WebGL2:
- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

## License

This project is open source. See LICENSE file for details.

## Contributing

Contributions welcome! Please feel free to submit issues and pull requests.

---

Built with ❤️ using Rust and Bevy for entertaining babies everywhere! 🍼✨
