# Programming LV2 Plugins - Rust Edition

This repository contains the sample plugins of the "Programming LV2 Plugins - Rust edition" book, as well as means to build both the plugins and the book.

## Building the book

The book is generated from the source files of the samples. In order to build the book, you need to have `make` and Python 3 installed. Simply type

```bash
make book.md
```

and the book will be written to `book.md`.

## Building the samples

Every sample is a self-contained Rust crate; You can simply build it with cargo. If you want to install the samples on your machine, you can run

```bash
make install
```

This will build the crates and copy the bundles to a installation prefix. The standard prefix is `~/.lv2`. If you want to install the plugins in another directory, you can set in the top of the `Makefile`.

## Licensing

Just like the original, the book and the code is published under the `ISC` license. See the [LICENSE file](LICENSE.md) for more info.