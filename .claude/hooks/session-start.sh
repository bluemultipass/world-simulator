#!/bin/bash
set -euo pipefail

if [ "${CLAUDE_CODE_REMOTE:-}" != "true" ]; then
  exit 0
fi

REPO_ROOT="$CLAUDE_PROJECT_DIR"

# Install git pre-commit hook from the versioned script
install -m 755 "$REPO_ROOT/.claude/hooks/pre-commit.sh" "$REPO_ROOT/.git/hooks/pre-commit"

echo "Pre-commit hook installed."
