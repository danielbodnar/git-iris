#!/bin/bash
set -euo pipefail

# If the first argument starts with a dash or is a known subcommand, prepend git-iris.
first_arg="${1:-}"
case "$first_arg" in
    ""|-*|gen|commit|review|pr|changelog|release-notes|studio|config|project-config|list-presets|themes|completions|hook|help)
        set -- git-iris "$@"
        ;;
esac

# Setup git config if environment variables are provided
if [ -n "${GIT_USER_NAME:-}" ]; then
    git config --global user.name "$GIT_USER_NAME"
fi

if [ -n "${GIT_USER_EMAIL:-}" ]; then
    git config --global user.email "$GIT_USER_EMAIL"
fi

# Respect XDG config locations when present instead of assuming ~/.config.
CONFIG_HOME="${XDG_CONFIG_HOME:-$HOME/.config}"
CONFIG_DIR="${CONFIG_HOME}/git-iris"
CONFIG_FILE="${CONFIG_DIR}/config.toml"

# Create Git-Iris config directory
mkdir -p "$CONFIG_DIR"

# Initialize config settings
CONFIG_ARGS=()
DEFAULT_PROVIDER="${GITIRIS_DEFAULT_PROVIDER:-}"

# Process provider config
if [ -n "${GITIRIS_PROVIDER:-}" ]; then
    CONFIG_ARGS+=(--provider "$GITIRIS_PROVIDER")

    # API key for the provider (if provided)
    if [ -n "${GITIRIS_API_KEY:-}" ]; then
        CONFIG_ARGS+=(--api-key "$GITIRIS_API_KEY")
    fi

    # Model for the provider (if provided)
    if [ -n "${GITIRIS_MODEL:-}" ]; then
        CONFIG_ARGS+=(--model "$GITIRIS_MODEL")
    fi

    # Fast model for the provider (if provided)
    if [ -n "${GITIRIS_FAST_MODEL:-}" ]; then
        CONFIG_ARGS+=(--fast-model "$GITIRIS_FAST_MODEL")
    fi

    # Token limit for the provider (if provided)
    if [ -n "${GITIRIS_TOKEN_LIMIT:-}" ]; then
        CONFIG_ARGS+=(--token-limit "$GITIRIS_TOKEN_LIMIT")
    fi
fi

# Custom instructions
if [ -n "${GITIRIS_INSTRUCTIONS:-}" ]; then
    CONFIG_ARGS+=(--instructions "$GITIRIS_INSTRUCTIONS")
fi

# Preset
if [ -n "${GITIRIS_PRESET:-}" ]; then
    CONFIG_ARGS+=(--preset "$GITIRIS_PRESET")
fi

# Gitmoji setting (boolean flag, not value)
if [ "${GITIRIS_GITMOJI:-}" = "true" ]; then
    CONFIG_ARGS+=(--gitmoji)
elif [ "${GITIRIS_GITMOJI:-}" = "false" ]; then
    CONFIG_ARGS+=(--no-gitmoji)
fi

# Additional parameters (comma-separated key=value pairs)
if [ -n "${GITIRIS_PARAMS:-}" ]; then
    # Split the comma-separated list and add each parameter
    IFS=',' read -ra PARAM_ARRAY <<<"$GITIRIS_PARAMS"
    for param in "${PARAM_ARRAY[@]}"; do
        CONFIG_ARGS+=(--param "$param")
    done
fi

# Apply configuration if parameters were provided
if [ "${#CONFIG_ARGS[@]}" -gt 0 ] || [ -n "$DEFAULT_PROVIDER" ]; then
    # Only configure if it's not already done or if forced
    if [ ! -f "$CONFIG_FILE" ] || [ "${GITIRIS_FORCE_CONFIG:-}" = "true" ]; then
        if [ "${#CONFIG_ARGS[@]}" -gt 0 ]; then
            git-iris config "${CONFIG_ARGS[@]}"
        fi

        # Allow callers to set a different default provider after configuring
        # a specific provider's API key/model.
        if [ -n "$DEFAULT_PROVIDER" ] && [ "$DEFAULT_PROVIDER" != "${GITIRIS_PROVIDER:-}" ]; then
            git-iris config --provider "$DEFAULT_PROVIDER"
        fi
    fi
fi

# If no command is provided, print help
if [ "$1" = "git-iris" ] && [ $# -eq 1 ]; then
    exec "$@" --help
else
    exec "$@"
fi
