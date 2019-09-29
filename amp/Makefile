DC=../document_compiler.py
INTROS_DIR=../introductions

all: chapter.md eg-amp-rs.lv2/amp.so

clean:
	rm -f chapter.md eg-amp-rs.lv2/amp.ttl.md eg-amp-rs.lv2/manifest.ttl.md Cargo.toml.md src/lib.rs.md eg-amp-rs.lv2/amp.so
	cargo clean

chapter.md: $(INTROS_DIR)/amp.md eg-amp-rs.lv2/amp.ttl.md eg-amp-rs.lv2/manifest.ttl.md Cargo.toml.md src/lib.rs.md
	cat $(INTROS_DIR)/amp.md > chapter.md
	echo "" >> chapter.md
	cat eg-amp-rs.lv2/manifest.ttl.md >> chapter.md
	echo "" >> chapter.md
	cat eg-amp-rs.lv2/amp.ttl.md >> chapter.md
	echo "" >> chapter.md
	cat Cargo.toml.md >> chapter.md
	echo "" >> chapter.md
	cat src/lib.rs.md >> chapter.md

eg-amp-rs.lv2/amp.ttl.md: eg-amp-rs.lv2/amp.ttl
	$(DC) -i eg-amp-rs.lv2/amp.ttl -o eg-amp-rs.lv2/amp.ttl.md

eg-amp-rs.lv2/manifest.ttl.md: eg-amp-rs.lv2/manifest.ttl
	$(DC) -i eg-amp-rs.lv2/manifest.ttl -o eg-amp-rs.lv2/manifest.ttl.md

Cargo.toml.md: Cargo.toml
	$(DC) -i Cargo.toml -o Cargo.toml.md

src/lib.rs.md: src/lib.rs
	$(DC) -i src/lib.rs -o src/lib.rs.md
	
eg-amp-rs.lv2/amp.so: Cargo.toml src/lib.rs
	cargo build --release
	cp target/release/libeg_amp_rs.so eg-amp-rs.lv2/amp.so