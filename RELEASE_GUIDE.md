# GitHub Release Guide

This guide explains how to create GitHub releases with automated builds for the Pidgin compiler.

## Overview

The automated release system will:
1. Build the compiler for all major platforms (Linux, macOS, Windows)
2. Create portable distributions with examples and documentation
3. Automatically create a GitHub release when you push a version tag
4. Upload all platform-specific distributions to the release

## Prerequisites

1. **GitHub Repository**: Your code must be in a GitHub repository
2. **GitHub Actions**: Enabled for your repository
3. **Write Permissions**: You need write access to create releases

## Creating a Release

### Step 1: Prepare Your Code

Make sure your code is ready for release:

```bash
# Test everything locally
cargo test
cargo build --release
./distribute.sh  # Test local distribution
```

### Step 2: Commit and Push Changes

```bash
# Commit your changes
git add .
git commit -m "Prepare for release v1.0.0"
git push origin main
```

### Step 3: Create and Push a Version Tag

```bash
# Create a version tag (use semantic versioning)
git tag v1.0.0

# Push the tag to GitHub
git push origin v1.0.0
```

**Important**: The tag must start with `v` (e.g., `v1.0.0`, `v2.1.3`) to trigger the release workflow.

### Step 4: Monitor the Build

1. Go to your GitHub repository
2. Click on the "Actions" tab
3. You'll see the "Build and Release" workflow running
4. Wait for all platforms to build successfully

### Step 5: Release is Created Automatically

Once all builds complete:
- A GitHub release is created automatically
- All platform distributions are uploaded
- Release notes are generated from commits

## Manual Release Creation

If you want to create a release manually:

1. Go to your GitHub repository
2. Click "Releases" in the right sidebar
3. Click "Create a new release"
4. Choose a tag or create a new one
5. Add release notes
6. Upload the distribution files manually

## Version Tagging Guidelines

### Semantic Versioning

Use semantic versioning: `MAJOR.MINOR.PATCH`

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Examples

```bash
# First release
git tag v1.0.0

# Bug fix release
git tag v1.0.1

# New feature release
git tag v1.1.0

# Breaking change release
git tag v2.0.0
```

## Platform Support

The automated build creates distributions for:

- **Linux x86_64**: `pidgin-linux-x86_64.zip`
- **Linux ARM64**: `pidgin-linux-aarch64.zip`
- **macOS Intel**: `pidgin-macos-x86_64.zip`
- **macOS Apple Silicon**: `pidgin-macos-aarch64.zip`
- **Windows x86_64**: `pidgin-windows-x86_64.zip`

## Distribution Contents

Each distribution contains:

```
pidgin-<platform>/
├── pidgin          # Main executable
├── run.sh                   # Unix/Linux/macOS runner script
├── run.bat                  # Windows runner script
├── README.md                # Platform-specific instructions
└── examples/                # Example programs
    ├── hello.pg
    ├── fibonacci.pg
    ├── simple.pg
    └── ...
```

## Troubleshooting

### Build Failures

If builds fail:

1. **Check the Actions tab** for error details
2. **Test locally** with the same Rust version
3. **Check dependencies** in Cargo.toml
4. **Verify cross-compilation targets** are installed

### Common Issues

**"Target not found" error:**
```bash
# Install missing targets locally
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-apple-darwin
# etc.
```

**"Permission denied" error:**
```bash
# Make scripts executable
chmod +x scripts/create-distribution.sh
```

**"File not found" error:**
- Check that all required files exist
- Verify file paths in the distribution script

### Manual Build Testing

Test the build process locally:

```bash
# Test the distribution script
./scripts/create-distribution.sh test-platform ubuntu-latest

# Check the output
ls -la pidgin-test-platform/
```

## Release Notes

### Automatic Generation

GitHub Actions automatically generates release notes from:
- Commit messages since the last release
- Pull request titles
- Issue references

### Custom Release Notes

You can customize release notes by:
1. Editing the release after creation
2. Adding a `RELEASE_NOTES.md` file
3. Using GitHub's release editor

### Example Release Notes

```markdown
# Pidgin Compiler v1.0.0

## What's New
- Initial release of the Pidgin programming language compiler
- Support for basic programming constructs
- Cross-platform compatibility

## Features
- Variables and assignments
- Arithmetic operations
- Control flow (if/else, while loops)
- String manipulation
- Array support
- Function definitions and calls

## Installation
Download the appropriate distribution for your platform:
- **Linux**: `pidgin-linux-x86_64.zip`
- **macOS**: `pidgin-macos-x86_64.zip`
- **Windows**: `pidgin-windows-x86_64.zip`

## Usage
```bash
# Extract and run
unzip pidgin-linux-x86_64.zip
cd pidgin-linux-x86_64
./run.sh examples/hello.pg
```

## Breaking Changes
None (initial release)

## Bug Fixes
- Fixed error reporting line numbers
- Improved array bounds checking
```

## Advanced Configuration

### Customizing the Workflow

Edit `.github/workflows/release.yml` to:
- Add more platforms
- Change build configurations
- Modify distribution contents
- Add custom build steps

### Adding New Platforms

To add a new platform:

1. Add to the matrix in the workflow:
```yaml
- os: new-os
  target: new-target
  platform: new-platform
  artifact_name: pidgin-new-platform
```

2. Update the distribution script if needed
3. Test the build locally

### Conditional Releases

You can make releases conditional:

```yaml
# Only release on main branch
if: github.ref == 'refs/heads/main' && startsWith(github.ref, 'refs/tags/v')

# Only release on specific tags
if: startsWith(github.ref, 'refs/tags/v') && !contains(github.ref, '-alpha')
```

## Security Considerations

### Code Signing

For production releases, consider code signing:

1. **macOS**: Use Apple Developer certificates
2. **Windows**: Use Authenticode certificates
3. **Linux**: Use GPG signatures

### Security Scanning

Add security scanning to your workflow:

```yaml
- name: Security scan
  uses: actions/security-scan@v1
  with:
    path: .
```

## Monitoring and Analytics

### Release Metrics

Track release success:
- Download counts
- Platform usage
- Error reports
- User feedback

### Automated Testing

Add automated testing to releases:
- Unit tests
- Integration tests
- Example program validation
- Performance benchmarks

---

## Summary

The automated release system makes it easy to:
1. Create consistent releases across all platforms
2. Automate the distribution process
3. Provide users with ready-to-use packages
4. Maintain release quality and consistency

Follow this guide to create professional, automated releases for your Pidgin compiler! 