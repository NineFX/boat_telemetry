rust_rpi_target := "armv7-unknown-linux-gnueabihf"
version := env_var_or_default("GITHUB_REF_NAME", "0.0.1-dev")

default:
  @just --list

build-rust-release: 
    cargo build --release

build-rust-release-rpi:
    cargo build --release --target {{rust_rpi_target}}

install_package: package
    @echo 'Generating a Debian package'
    sudo apt-get remove boat-telemetry || true
    sudo apt-get install -y -f ./target/debian/boat-telemetry_{{version}}_arm64.deb

package: build-rust-release
    @echo 'Generating a Debian package'
    cargo deb --strip --deb-version {{version}}
    dpkg-deb -c target/debian/boat-telemetry_{{version}}_arm64.deb

package-rpi:
    @echo 'Generating a cross-compiled Debian package for Raspberry Pi'
    CARGO_DEB_DPKG_SHLIBDEPS=false cargo deb --strip --deb-version {{version}} --target {{rust_rpi_target}}

clean:
    cargo clean

apt-repo:
    rm -rf apt-repo
    aptly repo create -config=.aptly.conf -distribution=bullseye -component=main boat_telemetry
    aptly repo add -config=.aptly.conf boat_telemetry target/debian/*.deb
    aptly publish repo -config=.aptly.conf --skip-signing boat_telemetry

# Install prerequisites on a Debian Linux distro
bootstrap:
    apt-get update
    apt-get install -y clang-16 clang-tools-16 gdb valgrind curl wget \
                       gcc-arm-linux-gnueabihf g++-arm-linux-gnueabihf \
                       libc6-dev-armhf-cross libc6-armhf-cross \
                       libstdc++6-armhf-cross libgcc-s1-armhf-cross \
                       libc++1-16 libc++abi1-16 libclang-rt-16-dev libstdc++6 \
                       checksec aptly
    cargo install cargo-deb
    rustup target add {{rust_rpi_target}}

ci-build: bootstrap build-rust-release-rpi

ci-release: ci-build package-rpi apt-repo
