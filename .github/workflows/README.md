# GitHub Actions Workflows

This directory contains the optimized GitHub Actions workflows for the Pidgin compiler project.

## Workflow Overview

### 1. `test.yml` - Fast Feedback
**Purpose**: Quick validation for rapid feedback on PRs and pushes
**Triggers**: Push to main/master, Pull requests
**Duration**: ~2-3 minutes

**What it does**:
- ✅ Builds the project
- ✅ Runs unit tests (`cargo test`)
- ✅ Tests basic examples (hello.pg, simple.pg, fibonacci.pg)
- ✅ Tests debug modes (--tokens, --ast)
- ✅ Tests REPL functionality
- ✅ Quick syntax check of key examples

**Why it exists**: Provides fast feedback to developers without running comprehensive tests that take longer.

### 2. `ci.yml` - Comprehensive CI
**Purpose**: Thorough testing and quality assurance
**Triggers**: Push to main/master, Pull requests
**Duration**: ~8-10 minutes

**What it does**:
- ✅ **Multi-Rust Testing**: Tests with Rust stable, 1.70, and 1.75
- ✅ **Code Quality**: Clippy linting and code formatting checks
- ✅ **Security Audit**: Cargo audit for security vulnerabilities
- ✅ **Basic Multi-Platform Build**: Builds on Linux, macOS, Windows
- ✅ **Comprehensive Testing**: All feature tests and error handling
- ✅ **Performance Benchmarking**: Performance and memory usage analysis
- ✅ **Distribution Testing**: Tests the distribution script

**Why it exists**: Ensures code quality, security, and compatibility across different environments.

### 3. `release.yml` - Release Management
**Purpose**: Automated release creation and distribution
**Triggers**: Version tags (v*), Manual dispatch
**Duration**: ~5-7 minutes

**What it does**:
- ✅ **Production Builds**: Cross-platform builds with specific targets
- ✅ **Distribution Creation**: Creates platform-specific distribution packages
- ✅ **Artifact Generation**: Produces zip archives for each platform
- ✅ **GitHub Release**: Automatically creates releases with artifacts

**Why it exists**: Handles the complete release process from build to distribution.

## Workflow Optimization

### Eliminated Redundancies:
1. **Removed duplicate comprehensive testing** from `test.yml` (now in `ci.yml`)
2. **Removed duplicate performance testing** from `test.yml` (now in `ci.yml`)
3. **Removed duplicate multi-platform builds** from `ci.yml` (simplified to basic builds)
4. **Removed duplicate release job** from `ci.yml` (handled by `release.yml`)
5. **Removed duplicate error handling tests** from `test.yml` (now in `ci.yml`)

### Benefits:
- ⚡ **Faster PR feedback**: `test.yml` completes in ~2-3 minutes
- 🔍 **Comprehensive quality**: `ci.yml` ensures thorough testing
- 🚀 **Focused releases**: `release.yml` handles only release tasks
- 💰 **Reduced CI costs**: Eliminated duplicate work
- 🎯 **Clear separation of concerns**: Each workflow has a specific purpose

## Usage

### For Developers:
- **PRs**: `test.yml` runs first for quick feedback, then `ci.yml` for comprehensive validation
- **Main branch**: Both `test.yml` and `ci.yml` run to ensure quality

### For Releases:
- **Create a tag**: `git tag v1.0.0 && git push origin v1.0.0`
- **Manual trigger**: Use the "workflow_dispatch" trigger in GitHub Actions UI

### For Performance Monitoring:
- **Performance benchmarks**: Run automatically on main branch pushes via `ci.yml`

## Workflow Dependencies

```
test.yml (Fast Feedback)
    ↓
ci.yml (Comprehensive CI)
    ↓
release.yml (Release Management) ← Only on tags
```

## Configuration

### Rust Versions:
- **test.yml**: stable only
- **ci.yml**: stable, 1.70, 1.75
- **release.yml**: stable only

### Platforms:
- **test.yml**: ubuntu-latest only
- **ci.yml**: ubuntu-latest, macos-latest, windows-latest
- **release.yml**: ubuntu-latest, macos-latest, windows-latest (with specific targets)

### Caching:
All workflows use GitHub Actions caching for:
- Cargo registry
- Cargo git dependencies
- Build artifacts

This reduces build times and CI costs significantly. 