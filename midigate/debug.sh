cargo build
cp target/debug/*.so eg-midigate-rs.lv2
mkdir -p ~/.lv2
cp -r *.lv2 ~/.lv2/
# jalv urn:rust-midigate-book:eg-fifths-rs
carla --gdb