# Changelog

All notable changes to the Pidgin Compiler project will be documented in this file.

## [0.1.15] - 2025-01-19

### Added
- **Array Support**: Added fixed-size arrays `[1, 2, 3]` and dynamic arrays `{1, 2, 3}`
- **Array Methods**: Added `length()`, `push()`, `pop()`, and `clear()` methods for dynamic arrays
- **String Replacement**: Added `replaceChar()` method with transform syntax
- **Enhanced Module System**: Improved import system with support for multiple imports
- **Installation Scripts**: Added `install.sh` and `update.sh` for Unix-like systems
- **Windows Support**: Added `install.bat` and `update.bat` for Windows
- **Distribution Scripts**: Automated distribution creation for multiple platforms
- **GitHub Actions**: Automated CI/CD pipeline for builds and releases

### Changed
- **Enhanced Error Reporting**: Improved line and column information in error messages
- **Better Type System**: Improved type checking and error messages
- **Updated Documentation**: Comprehensive README with examples and troubleshooting

### Fixed
- **Error Line Reporting**: Fixed incorrect line numbers in error messages
- **Return Value Issues**: Fixed issues with return value reporting
- **Cross-platform Compatibility**: Improved support for different architectures

## [0.1.10] - 2025-01-18

### Added
- **Basic Language Features**: Variables, arithmetic, comparisons, conditionals, loops
- **Function Support**: Function declarations and calls
- **Print Statements**: Basic print functionality with format strings
- **Module Imports**: Basic import system with GET/FROM syntax
- **Interactive REPL**: Command-line interface for testing code
- **Error Handling**: Basic error reporting and handling

### Changed
- **Initial Release**: First public release of the Pidgin compiler

## [Unreleased]

### Planned Features
- **Standard Library**: Built-in functions for common operations
- **File I/O**: Reading and writing files
- **Error Recovery**: Better error recovery in parser
- **Optimizations**: Performance improvements
- **IDE Support**: Language server protocol support
- **Package Manager**: Dependency management system

---

## Version Format

This project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html):
- **MAJOR** version for incompatible API changes
- **MINOR** version for added functionality in a backwards compatible manner
- **PATCH** version for backwards compatible bug fixes

## Release Process

1. Update version in `Cargo.toml`
2. Update this changelog
3. Create a git tag: `git tag v0.1.15`
4. Push the tag: `git push origin v0.1.15`
5. GitHub Actions will automatically build and release 