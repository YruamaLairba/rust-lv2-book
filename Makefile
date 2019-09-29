INSTALL_PREFIX = ~/.lv2
CARGO = cargo

all: book.md

clean:
	make -C amp clean
	rm -f book.md
	cargo clean --manifest-path amp/Cargo.toml

book.md: amp/chapter.md Introduction.md
	cat Introduction.md > book.md
	cat amp/chapter.md >> book.md

amp/chapter.md:
	make -C amp chapter.md