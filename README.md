# Programming LV2 Plugins - Rust Edition

This repository contains the sample plugins of the "Programming LV2 Plugins - Rust edition" book, as well as means to build both the plugins and the book.

## Building the book

The book is generated from the source files of the samples. In order to build the book, you need to have Python 3 installed. Simply type

```bash
python3 make_book.py
```

and the book will be written to `export/README.md`.

## Building the samples

Every sample is a self-contained Rust crate; You can simply build it with cargo. If you want to install the samples on your machine, you can run `./install.sh` in every crate's directory. This will build the crates and copy the bundles to `~/.lv2`.

## Licensing

Just like the original, the book and the code is published under the `ISC` license. See the [LICENSE file](LICENSE.md) for more info.