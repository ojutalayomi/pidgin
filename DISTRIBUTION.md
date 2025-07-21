# Pidgin Compiler Distribution Guide

This guide explains how to create portable distributions of the Pidgin compiler that can run on any computer without requiring Rust or any development tools.

## Quick Start

### For Immediate Use (Current Platform)

1. **Build the release version:**
   ```bash
   cargo build --release
   ```

2. **Create a distribution:**
   ```bash
   ./distribute.sh
   ```

3. **Test the distribution:**
   ```bash
   cd pidgin-dist
   ./run.sh examples/hello.pg
   ```

4. **Distribute the zip file:**
   - Copy `pidgin-Darwin-x86_64.zip` (or similar) to any computer
   - Extract it
   - Run: `./run.sh examples/hello.pg`

### For Multiple Platforms

1. **Build for all platforms:**
   ```bash
   ./build-all-platforms.sh
   ```

2. **Distribute the appropriate zip file for each platform:**
   - `pidgin-linux-x86_64.zip` - Linux (Intel/AMD)
   - `pidgin-linux-aarch64.zip` - Linux (ARM)
   - `pidgin-macos-x86_64.zip` - macOS (Intel)
   - `pidgin-macos-aarch64.zip` - macOS (Apple Silicon)
   - `pidgin-windows-x86_64.zip` - Windows (Intel/AMD)

## Distribution Methods

### Method 1: Simple Distribution (Recommended)

Use the `distribute.sh` script to create a distribution for the current platform:

```bash
./distribute.sh
```

This creates:
- `pidgin-dist/` - Directory with everything needed
- `pidgin-<platform>-<arch>.zip` - Portable archive

### Method 2: Multi-Platform Distribution

Use the `build-all-platforms.sh` script to create distributions for all major platforms:

```bash
./build-all-platforms.sh
```

This requires:
- Rust with cross-compilation targets installed
- May take longer to build all platforms

### Method 3: Manual Distribution

1. **Build the release version:**
   ```bash
   cargo build --release
   ```

2. **Copy the executable and examples:**
   ```bash
   mkdir my-distribution
   cp target/release/pidgin my-distribution/
   cp -r examples my-distribution/
   ```

3. **Create a simple runner script:**
   ```bash
   # For Unix-like systems
   echo '#!/bin/bash
   ./pidgin "$@"' > my-distribution/run.sh
   chmod +x my-distribution/run.sh
   ```

## Running on Target Computers

### Unix-like Systems (Linux, macOS)

1. **Extract the distribution:**
   ```bash
   unzip pidgin-linux-x86_64.zip
   cd pidgin-linux-x86_64
   ```

2. **Make executable:**
   ```bash
   chmod +x pidgin
   chmod +x run.sh
   ```

3. **Run programs:**
   ```bash
   ./run.sh examples/hello.pg
   ./pidgin examples/hello.pg
   ./run.sh  # Start interactive REPL
   ```

### Windows

1. **Extract the distribution:**
   ```cmd
   # Use Windows Explorer or:
   powershell Expand-Archive pidgin-windows-x86_64.zip
   cd pidgin-windows-x86_64
   ```

2. **Run programs:**
   ```cmd
   run.bat examples\hello.pg
   pidgin.exe examples\hello.pg
   run.bat  # Start interactive REPL
   ```

## Installation Options

### System-wide Installation

```bash
# Unix-like systems
sudo ./install.sh

# Or manually:
sudo cp target/release/pidgin /usr/local/bin/
sudo chmod +x /usr/local/bin/pidgin
```

### Local Installation

```bash
# Install to user's local bin directory
./install.sh
# Choose 'y' when prompted for local installation
```

### Portable Installation

Just extract the distribution and run from the directory - no installation needed!

## File Structure

A typical distribution contains:

```
pidgin-dist/
├── pidgin          # Main executable
├── run.sh                   # Unix/Linux/macOS runner script
├── run.bat                  # Windows runner script
├── install.sh               # Installation script
├── README.md                # Usage instructions
└── examples/                # Example programs
    ├── hello.pg
    ├── fibonacci.pg
    ├── simple.pg
    └── ...
```

## Troubleshooting

### Permission Denied
```bash
chmod +x pidgin
chmod +x run.sh
```

### File Not Found
- Make sure you're in the correct directory
- Check that the file path is correct
- Use relative paths: `./run.sh examples/hello.pg`

### Executable Not Found
- Verify the executable exists in the distribution
- Check that it's the correct architecture for your system
- Try running it directly: `./pidgin examples/hello.pg`

### Cross-Platform Issues
- Use the appropriate distribution for your platform
- For Windows, use the `.exe` version
- For Apple Silicon Macs, use the `aarch64` version

## Advanced Usage

### Creating Custom Distributions

1. **Modify the distribution script:**
   ```bash
   # Edit distribute.sh to include additional files
   cp my-custom-examples/*.pg "$DIST_DIR/examples/"
   ```

2. **Add configuration files:**
   ```bash
   # Add any configuration files your users might need
   cp config.json "$DIST_DIR/"
   ```

### Automated Distribution

Create a CI/CD pipeline to automatically build and distribute:

```yaml
# Example GitHub Actions workflow
name: Build and Distribute
on: [push, release]
jobs:
  build:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo build --release
      - run: ./distribute.sh
      - uses: actions/upload-artifact@v2
        with:
          name: pidgin-${{ matrix.os }}
          path: pidgin-*.zip
```

## Security Considerations

- The distributed executable is statically linked and self-contained
- No external dependencies are required
- The executable only reads `.pg` files and writes to stdout/stderr
- Consider code signing for production distributions

## Performance

- Release builds are optimized for performance
- The executable is typically 600KB-1MB in size
- Startup time is minimal
- Memory usage is low

## Support

For issues with distribution or running the compiler:

1. Check the platform-specific README in the distribution
2. Verify you're using the correct architecture
3. Test with the included examples first
4. Check file permissions and paths 