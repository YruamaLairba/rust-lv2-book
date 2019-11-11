cargo build --release
cp target/release/*.so eg-midigate-rs.lv2
mkdir -p ~/.lv2
cp -r *.lv2 ~/.lv2/