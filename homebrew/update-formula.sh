#!/bin/bash
# Update Homebrew formula SHA256 checksums for a release
# Usage: ./update-formula.sh v2.0.0

set -e

VERSION="${1:-$(gh release view --json tagName -q .tagName 2>/dev/null)}"
if [ -z "$VERSION" ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 v2.0.0"
    exit 1
fi

VERSION_NUM="${VERSION#v}"
FORMULA="homebrew/git-iris.rb"
REPO="hyperb1iss/git-iris"

echo "Updating formula for ${VERSION}..."

# Download and calculate SHA256 for each artifact
calc_sha() {
    local artifact="$1"
    local url="https://github.com/${REPO}/releases/download/${VERSION}/${artifact}"
    echo "  Fetching ${artifact}..." >&2
    curl -fsSL "$url" | shasum -a 256 | cut -d' ' -f1
}

# Get SHA256 for source tarball
SOURCE_URL="https://github.com/${REPO}/archive/refs/tags/${VERSION}.tar.gz"
echo "  Fetching source tarball..." >&2
SOURCE_SHA=$(curl -fsSL "$SOURCE_URL" | shasum -a 256 | cut -d' ' -f1)

MACOS_ARM64_SHA=$(calc_sha "git-iris-macos-arm64")
LINUX_ARM64_SHA=$(calc_sha "git-iris-linux-arm64")
LINUX_AMD64_SHA=$(calc_sha "git-iris-linux-amd64")

echo ""
echo "SHA256 checksums for ${VERSION}:"
echo "  macos-arm64:  ${MACOS_ARM64_SHA}"
echo "  linux-arm64:  ${LINUX_ARM64_SHA}"
echo "  linux-amd64:  ${LINUX_AMD64_SHA}"
echo "  source:       ${SOURCE_SHA}"
echo ""

# Update formula
sed -i.bak \
    -e "s/version \".*\"/version \"${VERSION_NUM}\"/" \
    -e "s/PLACEHOLDER_MACOS_ARM64_SHA256/${MACOS_ARM64_SHA}/" \
    -e "s/PLACEHOLDER_LINUX_ARM64_SHA256/${LINUX_ARM64_SHA}/" \
    -e "s/PLACEHOLDER_LINUX_AMD64_SHA256/${LINUX_AMD64_SHA}/" \
    -e "s/PLACEHOLDER_SOURCE_SHA256/${SOURCE_SHA}/" \
    "$FORMULA"

rm -f "${FORMULA}.bak"

echo "Updated ${FORMULA}"
echo ""
echo "Next steps:"
echo "  1. Copy to your homebrew-tap repo:"
echo "     cp ${FORMULA} ../homebrew-tap/Formula/"
echo "  2. Commit and push to hyperb1iss/homebrew-tap"
echo ""
