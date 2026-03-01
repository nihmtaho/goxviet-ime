#!/bin/bash
# Extract changelog section for a specific version from CHANGELOG.md
# Usage: ./extract-changelog.sh <version>
# Example: ./extract-changelog.sh 2.0.9
#
# This script can be run from:
# 1. Project root (where CHANGELOG.md exists)
# 2. Skill directory (will search for CHANGELOG.md in parent directories)

set -e

VERSION=$1

if [ -z "$VERSION" ]; then
  echo "Usage: $0 <version>"
  echo "Example: $0 2.0.9"
  exit 1
fi

# Try to find CHANGELOG.md
# First check current directory
CHANGELOG_FILE="CHANGELOG.md"

if [ ! -f "$CHANGELOG_FILE" ]; then
  # Try parent directory
  if [ -f "../CHANGELOG.md" ]; then
    CHANGELOG_FILE="../CHANGELOG.md"
  elif [ -f "../../CHANGELOG.md" ]; then
    CHANGELOG_FILE="../../CHANGELOG.md"
  elif [ -f "../../../CHANGELOG.md" ]; then
    CHANGELOG_FILE="../../../CHANGELOG.md"
  elif [ -f "../../../../CHANGELOG.md" ]; then
    CHANGELOG_FILE="../../../../CHANGELOG.md"
  else
    echo "Error: CHANGELOG.md not found in current or parent directories" >&2
    exit 1
  fi
fi

# Find the section for this version
# Pattern: ## [VERSION] - DATE
# Extract from this line until the next ## or ---

# Use awk to extract the section
awk -v version="$VERSION" '
  BEGIN { printing=0; found=0 }
  
  # Match version header: ## [2.0.9] - 2026-02-10
  /^## \['"$VERSION"'\]/ {
    printing=1
    found=1
    next
  }
  
  # Stop at next version header or separator
  /^## \[/ && printing {
    printing=0
  }
  
  /^---$/ && printing {
    printing=0
  }
  
  # Print lines in the section (skip the version header itself)
  printing {
    print
  }
  
  END {
    if (!found) {
      print "Error: Version " version " not found in CHANGELOG.md" > "/dev/stderr"
      exit 1
    }
  }
' "$CHANGELOG_FILE"
