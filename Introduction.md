# Programming LV2 Plugins - Rust Edition

## Foreword

This book is an effort to translate the [LV2 Book by David Robillard](http://lv2plug.in/book/) for the [`lv2rs`](https://github.com/Janonard/lv2rs.git) crate. As such, the examples in this book as well as the README's and comments are copied from the original, but the book itself has been altered to adapt for the differences between C and Rust. Since the `lv2rs` crate has been discontinued in favor of [rust-lv2](https://github.com/rust-dsp/rust-lv2), this book is not and will never be complete. According to current planning, there is going to be a new translation for rust-lv2 as soon it's developed enough.

## Introduction

This is a series of well-documented example plugins that demonstrate the various features of LV2. Starting with the most basic plugin possible, each adds new functionality and explains the features used from a high level perspective.

API and vocabulary reference documentation explains details, but not the ``big picture''. This book is intended to complement the reference documentation by providing good reference implementations of plugins, while also conveying a higher-level understanding of LV2.

The chapters/plugins are arranged so that each builds incrementally on its predecessor. Reading this book front to back is a good way to become familiar with modern LV2 programming. The reader is expected to be familiar with Rust, but otherwise no special knowledge is required; the first plugin describes the basics in detail.

Each chapter corresponds to executable plugin code which can be found in the `samples` directory of the book's [Github Repository](https://github.com/Janonard/lv2rs-book). If you prefer to read actual source code, all the content here is also available in the source code as comments.
