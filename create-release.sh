#!/bin/bash

# Pidgin Compiler Release Creation Script
# This script helps you create a new release

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Pidgin Compiler Release Creation${NC}"
echo "====================================="

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo -e "${RED}Error: Not in a git repository${NC}"
    echo "Please run this script from your project directory"
    exit 1
fi

# Check if we have uncommitted changes
if ! git diff-index --quiet HEAD --; then
    echo -e "${YELLOW}Warning: You have uncommitted changes${NC}"
    echo "Please commit or stash your changes before creating a release"
    echo ""
    echo "To commit changes:"
    echo "  git add ."
    echo "  git commit -m \"Your commit message\""
    echo ""
    echo "To stash changes:"
    echo "  git stash"
    echo ""
    read -p "Continue anyway? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Get the current version from Cargo.toml
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | cut -d'"' -f2)
echo -e "${GREEN}Current version: ${CURRENT_VERSION}${NC}"

# Ask for the new version
echo ""
echo "Enter the new version (e.g., 1.0.0, 1.1.0, 2.0.0):"
read -p "New version: " NEW_VERSION

# Validate version format
if [[ ! $NEW_VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo -e "${RED}Error: Invalid version format${NC}"
    echo "Please use semantic versioning (e.g., 1.0.0)"
    exit 1
fi

# Check if version already exists
if git tag | grep -q "v$NEW_VERSION"; then
    echo -e "${RED}Error: Version v$NEW_VERSION already exists${NC}"
    exit 1
fi

echo ""
echo -e "${BLUE}Preparing release v$NEW_VERSION...${NC}"

# Update version in Cargo.toml
echo "Updating Cargo.toml..."
sed -i.bak "s/^version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" Cargo.toml
rm Cargo.toml.bak

# Build and test
echo "Building and testing..."
cargo build --release
cargo test

# Test distribution
echo "Testing distribution..."
./distribute.sh

# Commit version change
echo "Committing version change..."
git add Cargo.toml
git commit -m "Bump version to $NEW_VERSION"

# Create tag
echo "Creating tag v$NEW_VERSION..."
git tag v$NEW_VERSION

# Push changes
echo "Pushing changes to remote..."
git push origin main
git push origin v$NEW_VERSION

echo ""
echo -e "${GREEN}✓ Release v$NEW_VERSION created successfully!${NC}"
echo ""
echo "What happens next:"
echo "1. GitHub Actions will automatically build for all platforms"
echo "2. A release will be created with all distributions"
echo "3. You can monitor progress at: https://github.com/$(git config --get remote.origin.url | sed 's/.*github.com[:/]\([^/]*\/[^/]*\).*/\1/')/actions"
echo ""
echo "To check the release status:"
echo "  # Check Actions tab on GitHub"
echo "  # Or check locally:"
echo "  git log --oneline -5"
echo "  git tag -l | tail -5"
echo ""
echo -e "${BLUE}Release creation complete!${NC}" 