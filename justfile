# List available commands
_default:
    @just --list

# Packge binary, and misc files into tar.gz
package:
    @just compile
    tar -cvzf log-viewer.tar.gz \
        log-viewer.desktop \
        install.sh \
        -C "$PWD"/assets/ log-128x128.png \
        -C "$PWD"/target/aarch64-unknown-linux-gnu/release/ log-viewer

# Compile for ProSpectral
compile:
    cross build --release --target aarch64-unknown-linux-gnu

# Debug build
build:
    cross build --target aarch64-unknown-linux-gnu