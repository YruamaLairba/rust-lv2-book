INTROS_DIR=introductions
INSTALL_PREFIX=~/.lv2/

book.md: amp/chapter.md $(INTROS_DIR)/intro.md
	cat $(INTROS_DIR)/intro.md > book.md;
	echo "" >> book.md;
	cat amp/chapter.md >> book.md;

clean:
	make -C amp clean
	rm -f book.md
	cargo clean --manifest-path amp/Cargo.toml

install: amp/eg-amp-rs.lv2/amp.so
	cp -pr amp/eg-amp-rs.lv2/ $(INSTALL_PREFIX)

amp/chapter.md:
	make -C amp chapter.md

amp/eg-amp-rs.lv2/amp.so:
	make -C amp eg-amp-rs.lv2/amp.so