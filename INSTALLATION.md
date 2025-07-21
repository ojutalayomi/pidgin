# Pidgin Compiler Installation Guide

This guide shows you how to install the Pidgin compiler so you can use the `pidgin` command from anywhere on your system.

## Quick Installation

### Option 1: Automatic Installation (Recommended)

Run the installation script:

```bash
./install.sh
```

The script will:
1. Build the release version if needed
2. Offer system-wide or local installation
3. Guide you through the process

### Option 2: Manual Installation

#### System-wide Installation (requires sudo)

```bash
# Build the release version
cargo build --release

# Install system-wide (requires sudo)
sudo cp target/release/pidgin /usr/local/bin/
sudo chmod +x /usr/local/bin/pidgin
```

#### Local Installation (no sudo required)

```bash
# Build the release version
cargo build --release

# Create local bin directory
mkdir -p ~/.local/bin

# Copy the executable
cp target/release/pidgin ~/.local/bin/

# Make it executable
chmod +x ~/.local/bin/pidgin
```

## Adding to PATH

After installation, you need to add the installation directory to your PATH.

### For macOS/Linux:

Add this line to your shell configuration file:

**For bash users (~/.bashrc):**
```bash
export PATH="$HOME/.local/bin:$PATH"
```

**For zsh users (~/.zshrc):**
```bash
export PATH="$HOME/.local/bin:$PATH"
```

**For system-wide installation:**
```bash
# Usually not needed as /usr/local/bin is already in PATH
```

### For Windows:

1. **Using Environment Variables:**
   - Right-click "This PC" → Properties → Advanced system settings
   - Click "Environment Variables"
   - Under "User variables", find "Path" and click "Edit"
   - Add the directory containing `pidgin.exe`

2. **Using PowerShell:**
   ```powershell
   $env:PATH += ";C:\path\to\pidgin"
   ```

## Verifying Installation

After installation and adding to PATH, restart your terminal and test:

```bash
# Test the command
pidgin --help

# Run an example
pidgin examples/hello.pg
```

## Usage Examples

Once installed, you can use the compiler from anywhere:

```bash
# Run a program
pidgin my-program.pg

# Run from any directory
pidgin /path/to/program.pg

# Start interactive mode
pidgin

# Show tokens (debug mode)
pidgin --tokens my-program.pg

# Show AST (debug mode)
pidgin --ast my-program.pg
```

## Installation Methods

### Method 1: Using the Install Script

```bash
# Interactive installation
./install.sh

# System-wide installation (requires sudo)
sudo ./install.sh
```

### Method 2: Using the Distribution

1. **Download the distribution:**
   ```bash
   # Extract the distribution
   unzip pidgin-macos-x86_64.zip
   cd pidgin-macos-x86_64
   ```

2. **Install using the included script:**
   ```bash
   ./install.sh
   ```

### Method 3: Manual Installation

```bash
# Build from source
cargo build --release

# Choose installation location
# Option A: System-wide (requires sudo)
sudo cp target/release/pidgin /usr/local/bin/
sudo chmod +x /usr/local/bin/pidgin

# Option B: Local installation
mkdir -p ~/.local/bin
cp target/release/pidgin ~/.local/bin/
chmod +x ~/.local/bin/pidgin
```

## Platform-Specific Instructions

### macOS

```bash
# Using Homebrew (if available)
brew install --build-from-source .

# Manual installation
./install.sh
```

### Linux

```bash
# Ubuntu/Debian
sudo apt-get install build-essential
./install.sh

# CentOS/RHEL/Fedora
sudo yum groupinstall "Development Tools"
./install.sh
```

### Windows

```bash
# Using PowerShell
.\install.sh

# Manual installation
copy target\release\pidgin.exe C:\Windows\System32\
```

## Troubleshooting

### "Command not found" error

1. **Check if the executable exists:**
   ```bash
   ls -la ~/.local/bin/pidgin
   # or
   ls -la /usr/local/bin/pidgin
   ```

2. **Check if the directory is in PATH:**
   ```bash
   echo $PATH
   ```

3. **Add to PATH manually:**
   ```bash
   export PATH="$HOME/.local/bin:$PATH"
   ```

### Permission denied error

```bash
# Make the executable executable
chmod +x ~/.local/bin/pidgin
# or
sudo chmod +x /usr/local/bin/pidgin
```

### "No such file or directory" error

This usually means the executable was built for a different architecture. Make sure you're using the correct distribution for your system.

## Uninstalling

### Remove from PATH

Remove the PATH line from your shell configuration file (~/.bashrc, ~/.zshrc, etc.).

### Remove the executable

```bash
# For local installation
rm ~/.local/bin/pidgin

# For system-wide installation
sudo rm /usr/local/bin/pidgin
```

## Advanced Installation

### Custom Installation Directory

```bash
# Install to a custom directory
mkdir -p /opt/pidgin
cp target/release/pidgin /opt/pidgin/
chmod +x /opt/pidgin/pidgin

# Add to PATH
export PATH="/opt/pidgin:$PATH"
```

### Multiple Versions

```bash
# Install different versions
cp target/release/pidgin ~/.local/bin/pidgin-v1.0
cp target/release/pidgin ~/.local/bin/pidgin-v1.1

# Use specific version
pidgin-v1.0 my-program.pg
```

## Integration with IDEs

### VS Code

Add to your VS Code settings.json:
```json
{
    "terminal.integrated.env.osx": {
        "PATH": "/usr/local/bin:${env:PATH}"
    }
}
```

### IntelliJ IDEA

1. Go to Settings → Tools → Terminal
2. Add the PATH environment variable

## Package Manager Integration

### Creating a Package

For distribution, you can create packages:

**For macOS (using Homebrew):**
```ruby
class PidginCompiler < Formula
  desc "A simple programming language compiler"
  homepage "https://github.com/your-repo/pidgin"
  url "https://github.com/your-repo/pidgin/releases/download/v1.0.0/pidgin-macos-x86_64.zip"
  sha256 "..."

  def install
    bin.install "pidgin"
  end
end
```

**For Linux (using apt):**
Create a .deb package with proper installation scripts.

---

## Summary

After following this guide, you'll be able to use the `pidgin` command from anywhere on your system, just like any other command-line tool. The compiler will be available in your PATH and ready to run any `.pg` file. 