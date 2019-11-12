cargo build
cp target/debug/*.so eg-amp-rs.lv2
mkdir -p ~/.lv2
cp -r *.lv2 ~/.lv2/
# jalv urn:rust-lv2-book:eg-amp-rs
carla --gdb