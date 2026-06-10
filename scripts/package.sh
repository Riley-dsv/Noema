cargo build --release

VERSION="v0.0.5-alpha"
TARGET="x86_64-linux"
ARCHIVE="noema-${VERSION}-${TARGET}.tar.gz"

mkdir -p "dist/noema-${VERSION}"

cp target/release/noema "dist/noema-${VERSION}/"
cp README.md "dist/noema-${VERSION}/"

cp LICENSE "dist/noema-${VERSION}/"

tar -czvf "$ARCHIVE" -C dist "noema-${VERSION}"
