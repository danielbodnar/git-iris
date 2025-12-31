#!/bin/sh
# Git-Iris Installer
# Usage: curl -fsSL https://raw.githubusercontent.com/hyperb1iss/git-iris/main/install.sh | sh
#
# Environment variables:
#   IRIS_INSTALL_DIR  - Installation directory (default: ~/.local/bin)
#   IRIS_VERSION      - Specific version to install (default: latest)

set -e

# SilkCircuit Neon colors
CYAN='\033[38;2;128;255;234m'
PURPLE='\033[38;2;225;53;255m'
CORAL='\033[38;2;255;106;193m'
GREEN='\033[38;2;80;250;123m'
YELLOW='\033[38;2;241;250;140m'
RED='\033[38;2;255;99;99m'
RESET='\033[0m'
BOLD='\033[1m'

REPO="hyperb1iss/git-iris"
BINARY_NAME="git-iris"
INSTALL_DIR="${IRIS_INSTALL_DIR:-$HOME/.local/bin}"

info() {
	printf '%b>%b %s\n' "$CYAN" "$RESET" "$1"
}

success() {
	printf '%b%b%b %s\n' "$GREEN" "✓" "$RESET" "$1"
}

warn() {
	printf '%b!%b %s\n' "$YELLOW" "$RESET" "$1"
}

error() {
	printf '%b%b%b %s\n' "$RED" "✗" "$RESET" "$1" >&2
	exit 1
}

# Detect OS
detect_os() {
	case "$(uname -s)" in
	Linux*) echo "linux" ;;
	Darwin*) echo "macos" ;;
	MINGW* | MSYS* | CYGWIN*) echo "windows" ;;
	*) error "Unsupported operating system: $(uname -s)" ;;
	esac
}

# Detect architecture
detect_arch() {
	case "$(uname -m)" in
	x86_64 | amd64) echo "amd64" ;;
	aarch64 | arm64) echo "arm64" ;;
	*) error "Unsupported architecture: $(uname -m)" ;;
	esac
}

# Get latest version from GitHub
get_latest_version() {
	if command -v curl >/dev/null 2>&1; then
		curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/'
	elif command -v wget >/dev/null 2>&1; then
		wget -qO- "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/'
	else
		error "curl or wget is required"
	fi
}

# Download file
download() {
	url="$1"
	output="$2"
	if command -v curl >/dev/null 2>&1; then
		curl -fsSL "$url" -o "$output"
	elif command -v wget >/dev/null 2>&1; then
		wget -q "$url" -O "$output"
	else
		error "curl or wget is required"
	fi
}

# Main installation
main() {
	printf '\n'
	printf '%b%b  ◆ Git-Iris Installer%b\n' "$PURPLE" "$BOLD" "$RESET"
	printf '%b  ─────────────────────%b\n\n' "$CORAL" "$RESET"

	# Detect platform
	OS=$(detect_os)
	ARCH=$(detect_arch)
	info "Detected platform: ${BOLD}${OS}-${ARCH}${RESET}"

	# Map to artifact names
	case "${OS}-${ARCH}" in
	linux-amd64) ARTIFACT="git-iris-linux-amd64" ;;
	linux-arm64) ARTIFACT="git-iris-linux-arm64" ;;
	macos-arm64) ARTIFACT="git-iris-macos-arm64" ;;
	macos-amd64)
		warn "macOS x86_64 not pre-built, try: cargo install git-iris"
		exit 1
		;;
	windows-*)
		warn "For Windows, download from GitHub releases or use: cargo install git-iris"
		exit 1
		;;
	*) error "No pre-built binary for ${OS}-${ARCH}" ;;
	esac

	# Get version
	VERSION="${IRIS_VERSION:-$(get_latest_version)}"
	if [ -z "$VERSION" ]; then
		error "Failed to determine version"
	fi
	info "Installing version: ${BOLD}${VERSION}${RESET}"

	# Create install directory
	mkdir -p "$INSTALL_DIR"

	# Download binary
	DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/${ARTIFACT}"
	TMP_FILE=$(mktemp)

	info "Downloading from GitHub releases..."
	download "$DOWNLOAD_URL" "$TMP_FILE" || error "Download failed. Check if ${VERSION} exists."

	# Install binary
	mv "$TMP_FILE" "${INSTALL_DIR}/${BINARY_NAME}"
	chmod +x "${INSTALL_DIR}/${BINARY_NAME}"
	success "Installed to ${CYAN}${INSTALL_DIR}/${BINARY_NAME}${RESET}"

	# Check PATH
	case ":$PATH:" in
	*":$INSTALL_DIR:"*) ;;
	*)
		printf '\n'
		warn "Add ${CYAN}${INSTALL_DIR}${RESET} to your PATH:"
		printf '\n'
		printf '%b  # Add to ~/.bashrc or ~/.zshrc:%b\n' "$CORAL" "$RESET"
		printf '  export PATH="$HOME/.local/bin:$PATH"\n'
		printf '\n'
		;;
	esac

	# Verify installation
	if command -v git-iris >/dev/null 2>&1 || [ -x "${INSTALL_DIR}/${BINARY_NAME}" ]; then
		printf '\n'
		success "${GREEN}${BOLD}Installation complete!${RESET}"
		printf '\n'
		printf '%b  Get started:%b\n' "$PURPLE" "$RESET"
		printf '    git-iris config --provider <openai|anthropic|google>\n'
		printf '    git-iris gen        %b# Generate commit message%b\n' "$CORAL" "$RESET"
		printf '    git-iris studio     %b# Launch interactive TUI%b\n' "$CORAL" "$RESET"
		printf '\n'
		printf '%b  Docs: https://hyperb1iss.github.io/git-iris%b\n' "$CYAN" "$RESET"
		printf '\n'
	else
		error "Installation verification failed"
	fi
}

main "$@"
