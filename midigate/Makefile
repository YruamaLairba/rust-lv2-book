DC=../document_compiler.py
INTROS_DIR=../introductions

all: eg-midigate-rs.lv2/midigate.so

clean:
	rm -f chapter.md eg-midigate-rs.lv2/midigate.ttl.md eg-midigate-rs.lv2/manifest.ttl.md Cargo.toml.md src/lib.rs.md 
	rm -f eg-midigate-rs.lv2/midigate.so
	cargo clean

chapter.md: $(INTROS_DIR)/midigate.md eg-midigate-rs.lv2/midigate.ttl.md eg-midigate-rs.lv2/manifest.ttl.md Cargo.toml.md src/lib.rs.md
	cat $(INTROS_DIR)/midigate.md > chapter.md
	echo "" >> chapter.md
	cat eg-midigate-rs.lv2/manifest.ttl.md >> chapter.md
	echo "" >> chapter.md
	cat eg-midigate-rs.lv2/midigate.ttl.md >> chapter.md
	echo "" >> chapter.md
	cat Cargo.toml.md >> chapter.md
	echo "" >> chapter.md
	cat src/lib.rs.md >> chapter.md

eg-midigate-rs.lv2/midigate.ttl.md: eg-midigate-rs.lv2/midigate.ttl
	$(DC) -i eg-midigate-rs.lv2/midigate.ttl -o eg-midigate-rs.lv2/midigate.ttl.md

eg-midigate-rs.lv2/manifest.ttl.md: eg-midigate-rs.lv2/manifest.ttl
	$(DC) -i eg-midigate-rs.lv2/manifest.ttl -o eg-midigate-rs.lv2/manifest.ttl.md

Cargo.toml.md: Cargo.toml
	$(DC) -i Cargo.toml -o Cargo.toml.md

src/lib.rs.md: src/lib.rs
	$(DC) -i src/lib.rs -o src/lib.rs.md
	
eg-midigate-rs.lv2/midigate.so: Cargo.toml src/lib.rs
	cargo build --release
	cp target/release/libeg_midigate_rs.so eg-midigate-rs.lv2/midigate.so