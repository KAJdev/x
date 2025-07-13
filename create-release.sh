#!/bin/bash

# Create Release Script
# This script reads the version from Cargo.toml, creates a git tag, and pushes it to trigger a release

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    print_error "Not in a git repository!"
    exit 1
fi

# Check if Cargo.toml exists
if [ ! -f "Cargo.toml" ]; then
    print_error "Cargo.toml not found in current directory!"
    exit 1
fi

# Extract version from Cargo.toml
VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')

if [ -z "$VERSION" ]; then
    print_error "Could not extract version from Cargo.toml"
    exit 1
fi

TAG_NAME="v$VERSION"

print_status "Current version in Cargo.toml: $VERSION"
print_status "Tag to create: $TAG_NAME"

# Check if tag already exists locally
if git tag -l | grep -q "^$TAG_NAME$"; then
    print_error "Tag $TAG_NAME already exists locally!"
    print_status "Existing tags:"
    git tag -l | grep "^v" | sort -V | tail -5
    exit 1
fi

# Check if tag exists on remote
if git ls-remote --tags origin | grep -q "refs/tags/$TAG_NAME$"; then
    print_error "Tag $TAG_NAME already exists on remote!"
    exit 1
fi

# Check if there are uncommitted changes
if ! git diff-index --quiet HEAD --; then
    print_warning "You have uncommitted changes:"
    git status --porcelain
    echo
    read -p "Do you want to continue anyway? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_status "Aborted by user"
        exit 1
    fi
fi

# Check if we're on main/master branch (optional warning)
CURRENT_BRANCH=$(git branch --show-current)
if [[ "$CURRENT_BRANCH" != "main" && "$CURRENT_BRANCH" != "master" ]]; then
    print_warning "You're not on main/master branch (currently on: $CURRENT_BRANCH)"
    read -p "Do you want to continue? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_status "Aborted by user"
        exit 1
    fi
fi

# Show what will happen
echo
print_status "This will:"
echo "  1. Create git tag: $TAG_NAME"
echo "  2. Push the tag to origin"
echo "  3. Trigger GitHub Actions release workflow"
echo "  4. Build binaries for all platforms"
echo "  5. Create a GitHub release with downloadable assets"
echo

read -p "Do you want to proceed? (y/N): " -n 1 -r
echo

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    print_status "Aborted by user"
    exit 1
fi

# Create the tag
print_status "Creating tag $TAG_NAME..."
if git tag -a "$TAG_NAME" -m "Release $TAG_NAME"; then
    print_success "Tag $TAG_NAME created successfully"
else
    print_error "Failed to create tag"
    exit 1
fi

# Push the tag
print_status "Pushing tag to origin..."
if git push origin "$TAG_NAME"; then
    print_success "Tag pushed successfully"
else
    print_error "Failed to push tag"
    print_status "Removing local tag..."
    git tag -d "$TAG_NAME"
    exit 1
fi

print_success "Release process initiated!"
echo
print_status "You can monitor the release progress at:"
echo "  https://github.com/kajdev/x/actions"
echo
print_status "Once complete, the release will be available at:"
echo "  https://github.com/kajdev/x/releases/tag/$TAG_NAME"
