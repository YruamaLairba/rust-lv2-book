cargo build
cp target/debug/*.so eg-metro-rs.lv2
mkdir -p ~/.lv2
cp -r *.lv2 ~/.lv2/
# jalv urn:rust-lv2-book:eg-metro-rs
carla --gdb
