set export
set dotenv-load := true

# Default command, runs when no arguments are given
_default:
    @just --list

# Run the dev server
dev:
  trunk serve --open

# Build for production
build:
  trunk build --release
