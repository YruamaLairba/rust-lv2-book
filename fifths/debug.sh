cargo build
cp target/debug/*.so eg-fifths-rs.lv2
mkdir -p ~/.lv2
cp -r *.lv2 ~/.lv2/
jalv urn:rust-lv2-book:eg-fifths-rs
# carla --gdb