# Zsh Completion for Web Browser History

## Overview

This project provides a tab-completion of web history in Zsh.
It is designed for use with a history file of a Chromium-based browser (defaults to Google Chrome history).
The querying of the browser database file is provided via a command-line utility (`web-hist`).

### Features:
- Searches your browser history and suggests previously visited URLs.
- Supports globbing with (`*`),
- Incremental completion for domain, path, and parameters

## Installation

1. **Build the project** using `make`:
    ```shell
    make
    ```

2. **Install** with:
    ```shell
    make install
    ```

    This installs:
    - `web-hist` into `~/bin/`.
    - Zsh completion script `_web` into `~/.zfunc/`.

You need to have `cargo` (Rust project manager) to run the build.

Ensure `~/bin` is in your `PATH` and `~/.zfunc` is sourced in your `.zshrc`:
```shell
export PATH=$HOME/bin:$PATH
fpath=($HOME/.zfunc $fpath)
autoload -Uz compinit && compinit
```

## Usage Examples

### Basic Usage:
```shell
web www.goog[TAB]  # Suggest visited URLs like 'www.google.com', 'www.google.fr'
```

### Using Globbing Characters:
```shell
web *math[TAB]       # Search history for any URL containing 'math'
web */*.ipynb[TAB]   # Find previously visited Jupyter Notebooks
web */*?*.ipynb[TAB]
```

### Using with Other Executables:
```shell
compdef google-chrome=web
```

## Environment Variables
- **WEB_HISTORY_FILE**: The location of the history file. Defaults to
`/.config/google-chrome/Default/History`.

Example:
```shell
export WEB_HISTORY_FILE=~/.config/microsoft-edge/Default/History
```

## Implementation
In large part generated by an LLM.
