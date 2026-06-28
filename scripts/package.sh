cargo build --release

VERSION=$(grep -m 1 "version" Cargo.toml | cut -d"=" -f2 | cut -d" " -f2)
TARGET="Linux-$(uname -m)"
ARCHIVE="noema-${VERSION}-${TARGET}.tar.gz"

mkdir -p "dist/noema-${VERSION}"

cp target/release/noema "dist/noema-${VERSION}/"
cp README.md "dist/noema-${VERSION}/"

cp LICENSE "dist/noema-${VERSION}/"

tar -czvf "$ARCHIVE" -C dist "noema-${VERSION}"
