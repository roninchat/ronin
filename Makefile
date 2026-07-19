# Ronin packaging helpers (Linux)
#
# Installs the release binary, freedesktop .desktop entry, and hicolor icons.

ROOT := $(abspath $(dir $(lastword $(MAKEFILE_LIST))))
PACKAGING_DIR := $(ROOT)/packaging
BINARY ?= $(ROOT)/target/release/ronin
PREFIX ?= $(HOME)/.local
INSTALL_SCRIPT := $(ROOT)/scripts/install.sh

.PHONY: help install uninstall install-system uninstall-system install-dry-run clean-install

help:
	@echo "Packaging targets:"
	@echo "  make install            Install to \$$PREFIX (default: ~/.local)"
	@echo "  make uninstall          Remove files from \$$PREFIX"
	@echo "  make install-system     Install to /usr/local (may need sudo)"
	@echo "  make uninstall-system   Remove from /usr/local"
	@echo "  make install-dry-run    Print planned install without writing"
	@echo "  make clean-install      Alias for uninstall"
	@echo "Override: make install PREFIX=/opt/ronin BINARY=./ronin"

install:
	$(INSTALL_SCRIPT) --prefix "$(PREFIX)" --binary "$(BINARY)"

uninstall:
	$(INSTALL_SCRIPT) --uninstall --prefix "$(PREFIX)"

install-system:
	$(INSTALL_SCRIPT) --system --binary "$(BINARY)"

uninstall-system:
	$(INSTALL_SCRIPT) --uninstall --system

install-dry-run:
	$(INSTALL_SCRIPT) --prefix "$(PREFIX)" --binary "$(BINARY)" --dry-run

clean-install: uninstall
