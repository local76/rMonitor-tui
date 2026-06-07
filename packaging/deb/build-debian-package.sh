#!/bin/sh
# Resolve script directory and change to it
cd "$(dirname "$0")"

echo "Building Debian package..."

# Create staging directory structure
mkdir -p debian/usr/bin
mkdir -p ../../dist/packages

# Locate and copy binary
if [ -f "../../dist/binaries/rmonitor" ]; then
    cp ../../dist/binaries/rmonitor debian/usr/bin/rmonitor
elif [ -f "../../target/x86_64-unknown-linux-musl/release/rmonitor" ]; then
    cp ../../target/x86_64-unknown-linux-musl/release/rmonitor debian/usr/bin/rmonitor
elif [ -f "../../target/release/rmonitor" ]; then
    cp ../../target/release/rmonitor debian/usr/bin/rmonitor
else
    echo "Error: compiled rmonitor binary not found in target/ or dist/binaries/."
    exit 1
fi

chmod 755 debian/usr/bin/rmonitor

# Run dpkg-deb to build the package
dpkg-deb --build debian ../../dist/packages/rmonitor.deb

# Clean up staging binary
rm -f debian/usr/bin/rmonitor
