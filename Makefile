# Variables
BINARY_PATH = target/release/web-hist
INSTALL_BIN_DIR = $(HOME)/bin
ZFUNC_DIR = $(HOME)/.zfunc
ZSH_COMPLETION_FILE = _web

# Default build target
all: build

# Build the Rust project in release mode
build:
	cargo build --release

# Install the binary and completion script
install: $(BINARY_PATH) $(ZSH_COMPLETION_FILE)
	# Ensure the installation directories exist
	mkdir -p $(INSTALL_BIN_DIR)
	mkdir -p $(ZFUNC_DIR)

	# Install the binary with 755 permissions
	install -m 755 $(BINARY_PATH) $(INSTALL_BIN_DIR)/web-hist

	# Install the Zsh completion file
	install -m 644 $(ZSH_COMPLETION_FILE) $(ZFUNC_DIR)/

# Clean up the build files
clean:
	cargo clean

# PHONY targets
.PHONY: all build install clean
