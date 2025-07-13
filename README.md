# x

Force a language model type your bash commands for you

- 🎯 **Context-Aware**: considers your current shell, OS, and environment because i write good software
- 🔒 **Safe**: always shows confirmation before executing commands, dont you even worry
- ⚙️ **Configurable**: support for multiple LLM providers because its cool like that
- 🚀 **Blazingly Fast**: i just like the crab tbh

## Installation

### Quick Install

```bash
# Clone and run the installer
git clone https://github.com/KAJdev/x
cd x
chmod +x install.sh
./install.sh
```

### Manual Install

```bash
# Clone and build
git clone https://github.com/KAJdev/x
cd x
cargo build --release

# Add to PATH (optional)
cp target/release/x /usr/local/bin/
```

## Setup

Configure your LLM provider before first use:

```bash
# Interactive setup (recommended)
x --config

# Or specify directly
x --config --provider openai --api-key your-api-key-here
x --config --provider claude --api-key your-api-key-here
```

## Usage

Use natural language to describe what you want to do:

```bash
# SSH key generation
x generate a new ssh key for me, dog

# File operations
x create a new directory called my-project and cd into it, slave

# Git operations
x create a new git repository and make initial commit, dont make me wait...

# System management
x HEY YOU! check disk usage for all mounted drives!!

# Package management
x install docker on ubuntu or else

# And much more!
```

## How it Works

do you really care? you are here because you are lazy.

## Configuration

The configuration file is stored at:

- macOS/Linux: `~/.config/x/config.toml`
- Windows: `%APPDATA%\x\config.toml`

Example config:

```toml
provider = "OpenAI"  # or "Claude"
api_key = "your-api-key-here"
```

## Safety

The tool will always:

- Show you the generated command before execution
- Ask for your confirmation
- Display the command output
- Never execute commands without your explicit approval

## License

MIT License - feel free to use and modify as needed i guess
