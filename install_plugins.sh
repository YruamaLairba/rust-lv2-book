# Build the crates.
cargo build --release --manifest-path amp/Cargo.toml;

# Copy the the libraries into the bundles.
cp amp/target/release/libeg_amp_rs.so amp/eg-amp-rs.lv2/amp.so;

# Install the bundles.
cp -r amp/eg-amp-rs.lv2 $INSTALL_PREFIX/eg-amp-rs.lv2;