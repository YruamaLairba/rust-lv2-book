cargo build --release
cp target/release/*.so eg-metro-rs.lv2
mkdir -p ~/.lv2
cp -r *.lv2 ~/.lv2/
