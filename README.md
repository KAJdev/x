# x

Force a language model type your bash commands for you

- considers your current shell, OS, and environment</sub>
- always shows confirmation before executing commands
- support for multiple LLM providers because its cool like that
- rust is really fast blah blah

## Installation

### Quick Install

```bash
# clone and run the installer
git clone https://github.com/KAJdev/x
cd x
chmod +x install.sh
./install.sh
```

### Manual Install

```bash
# clone and build
git clone https://github.com/KAJdev/x
cd x
cargo build --release

# add to PATH (optional)
cp target/release/x /usr/local/bin/
```

## Setup

Configure your LLM provider before first use:

```bash
# interactive setup (recommended)
x --config

# or specify directly
x --config --provider openai --api-key your-api-key-here
x --config --provider claude --api-key your-api-key-here
```

## Usage

Use natural language to describe what you want to do:

```bash
# file operations
x hey machine slave! create a new directory called my-project and cd into it

# git operations
x create a new git repository and make initial commit because im lazy

# system management
x if you really loved me youd check disk usage for all mounted drives

# package management
x install docker on ubuntu or else
```

## Configuration

The configuration file is stored at:

- macOS/Linux: `~/.config/x/config.toml`
- Windows: `%APPDATA%\x\config.toml`

Example config:

```toml
provider = "OpenAI"  # or "Claude"
api_key = "your-api-key-here"
```

## License

MIT License - feel free to use and modify as needed i guess
