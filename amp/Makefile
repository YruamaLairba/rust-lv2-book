DC=../document_compiler.py

all: chapter.md

clean:
	rm -f chapter.md eg-amp-rs.lv2/amp.ttl.md eg-amp-rs.lv2/manifest.ttl.md Cargo.toml.md src/lib.rs.md

chapter.md: Introduction.md eg-amp-rs.lv2/amp.ttl.md eg-amp-rs.lv2/manifest.ttl.md Cargo.toml.md src/lib.rs.md
	cat Introduction.md > chapter.md
	cat eg-amp-rs.lv2/manifest.ttl.md >> chapter.md
	cat eg-amp-rs.lv2/amp.ttl.md >> chapter.md
	cat Cargo.toml.md >> chapter.md
	cat src/lib.rs.md >> chapter.md

eg-amp-rs.lv2/amp.ttl.md: eg-amp-rs.lv2/amp.ttl
	$(DC) -i eg-amp-rs.lv2/amp.ttl -o eg-amp-rs.lv2/amp.ttl.md

eg-amp-rs.lv2/manifest.ttl.md: eg-amp-rs.lv2/manifest.ttl
	$(DC) -i eg-amp-rs.lv2/manifest.ttl -o eg-amp-rs.lv2/manifest.ttl.md

Cargo.toml.md: Cargo.toml
	$(DC) -i Cargo.toml -o Cargo.toml.md

src/lib.rs.md: src/lib.rs
	$(DC) -i src/lib.rs -o src/lib.rs.md