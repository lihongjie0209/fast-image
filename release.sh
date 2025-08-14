#!/bin/bash

# Manual release trigger script for Fast Image
# This script helps create releases with proper tagging

set -e

echo "🚀 Fast Image Release Script"

# Check if we're in a git repository
if [ ! -d ".git" ]; then
    echo "❌ Not a git repository. Please run this script from the project root."
    exit 1
fi

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
echo "📦 Current version: $CURRENT_VERSION"

# Ask for new version
read -p "🔢 Enter new version (or press Enter to use current): " NEW_VERSION
NEW_VERSION=${NEW_VERSION:-$CURRENT_VERSION}

echo "🏷️  Preparing release for version: $NEW_VERSION"

# Confirm
read -p "❓ Continue with release $NEW_VERSION? (y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "❌ Release cancelled"
    exit 1
fi

# Update version in Cargo.toml if different
if [ "$NEW_VERSION" != "$CURRENT_VERSION" ]; then
    echo "📝 Updating version in Cargo.toml..."
    sed -i.bak "s/^version = .*/version = \"$NEW_VERSION\"/" Cargo.toml
    rm Cargo.toml.bak 2>/dev/null || true
fi

# Update Cargo.lock
echo "🔒 Updating Cargo.lock..."
cargo update

# Create release commit
echo "💾 Creating release commit..."
git add Cargo.toml Cargo.lock
git commit -m "Release v$NEW_VERSION" || echo "No changes to commit"

# Create and push tag
echo "🏷️  Creating and pushing tag..."
git tag -a "v$NEW_VERSION" -m "Release v$NEW_VERSION"
git push origin "v$NEW_VERSION"
git push origin master

echo "🎉 Release v$NEW_VERSION has been triggered!"
echo "📋 Check GitHub Actions for build progress:"
echo "   https://github.com/lihongjie0209/fast-image/actions"
echo ""
echo "📦 Release will be available at:"
echo "   https://github.com/lihongjie0209/fast-image/releases/tag/v$NEW_VERSION"
