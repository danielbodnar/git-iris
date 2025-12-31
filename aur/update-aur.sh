#!/bin/bash
# Update AUR package for a new release
# Usage: ./update-aur.sh v2.0.0

set -e

VERSION="${1:-$(gh release view --repo hyperb1iss/git-iris --json tagName -q .tagName 2>/dev/null)}"
if [ -z "$VERSION" ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 v2.0.0"
    exit 1
fi

VERSION_NUM="${VERSION#v}"
REPO="hyperb1iss/git-iris"

echo "Updating AUR package to ${VERSION}..."

# Calculate SHA256 checksums
echo "Calculating checksums..."
SHA_X86_64=$(curl -fsSL "https://github.com/${REPO}/releases/download/${VERSION}/git-iris-linux-amd64" | sha256sum | cut -d' ' -f1)
SHA_AARCH64=$(curl -fsSL "https://github.com/${REPO}/releases/download/${VERSION}/git-iris-linux-arm64" | sha256sum | cut -d' ' -f1)

echo "  x86_64:  ${SHA_X86_64}"
echo "  aarch64: ${SHA_AARCH64}"

# Update PKGBUILD
cat > PKGBUILD << EOF
# Maintainer: Stefanie Jane <stef@hyperbliss.tech>
pkgname=git-iris-bin
pkgver=${VERSION_NUM}
pkgrel=1
pkgdesc="An intelligent agent that understands your code and crafts perfect Git artifacts"
arch=('x86_64' 'aarch64')
url="https://github.com/hyperb1iss/git-iris"
license=('Apache-2.0')
provides=('git-iris')
conflicts=('git-iris')
depends=('gcc-libs' 'openssl')

source_x86_64=("\${pkgname}-\${pkgver}-x86_64::https://github.com/hyperb1iss/git-iris/releases/download/v\${pkgver}/git-iris-linux-amd64")
source_aarch64=("\${pkgname}-\${pkgver}-aarch64::https://github.com/hyperb1iss/git-iris/releases/download/v\${pkgver}/git-iris-linux-arm64")

sha256sums_x86_64=('${SHA_X86_64}')
sha256sums_aarch64=('${SHA_AARCH64}')

package() {
    install -Dm755 "\${srcdir}/\${pkgname}-\${pkgver}-\${CARCH}" "\${pkgdir}/usr/bin/git-iris"
}
EOF

# Update .SRCINFO
cat > .SRCINFO << EOF
pkgbase = git-iris-bin
	pkgdesc = An intelligent agent that understands your code and crafts perfect Git artifacts
	pkgver = ${VERSION_NUM}
	pkgrel = 1
	url = https://github.com/hyperb1iss/git-iris
	arch = x86_64
	arch = aarch64
	license = Apache-2.0
	provides = git-iris
	conflicts = git-iris
	depends = gcc-libs
	depends = openssl
	source_x86_64 = git-iris-bin-${VERSION_NUM}-x86_64::https://github.com/hyperb1iss/git-iris/releases/download/v${VERSION_NUM}/git-iris-linux-amd64
	source_aarch64 = git-iris-bin-${VERSION_NUM}-aarch64::https://github.com/hyperb1iss/git-iris/releases/download/v${VERSION_NUM}/git-iris-linux-arm64
	sha256sums_x86_64 = ${SHA_X86_64}
	sha256sums_aarch64 = ${SHA_AARCH64}

pkgname = git-iris-bin
EOF

echo ""
echo "Updated PKGBUILD and .SRCINFO for ${VERSION}"
echo ""
echo "Next steps:"
echo "  1. Clone the AUR repo (first time only):"
echo "     git clone ssh://aur@aur.archlinux.org/git-iris-bin.git ~/dev/aur-git-iris"
echo ""
echo "  2. Copy files and push:"
echo "     cp PKGBUILD .SRCINFO ~/dev/aur-git-iris/"
echo "     cd ~/dev/aur-git-iris"
echo "     git add PKGBUILD .SRCINFO"
echo "     git commit -m \"Update to ${VERSION_NUM}\""
echo "     git push"
echo ""
