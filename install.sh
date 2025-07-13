#!/bin/bash

set -e

echo "Let's install this pos"

# is rust is installed?
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo is not installed. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# build it
echo "📦 Building x..."
cargo build --release

if [ ! -f "target/release/x" ]; then
    echo "❌ Build failed!" # LMAO
    exit 1
fi

echo "✅ Build successful!"

# where we droppin boys
echo ""
echo "📍 Choose installation location:"
echo "1) /usr/local/bin (system-wide, requires sudo)"
echo "2) ~/.local/bin (user-only)"
echo "3) Don't install, just build"

read -p "Enter your choice (1-3): " choice

case $choice in
    1)
        echo "🔐 Installing to /usr/local/bin (requires sudo)..."
        sudo cp target/release/x /usr/local/bin/
        echo "✅ Installed to /usr/local/bin/x"
        ;;
    2)
        echo "📁 Installing to ~/.local/bin..."
        mkdir -p ~/.local/bin
        cp target/release/x ~/.local/bin/
        echo "✅ Installed to ~/.local/bin/x"
        echo "⚠️  Make sure ~/.local/bin is in your PATH"
        echo "   Add this to your shell profile: export PATH=\"\$HOME/.local/bin:\$PATH\""
        ;;
    3)
        echo "✅ Build complete! Binary available at: ./target/release/x"
        ;;
    *)
        echo "❌ Invalid choice"
        exit 1
        ;;
esac

echo ""
echo "🎉 Installation complete!"
echo ""
echo "Next steps:"
echo "1. Configure your LLM provider: x --config"
echo "2. Start using: x your command here"
echo ""
echo "Example: x ssh key named id_rsa"
