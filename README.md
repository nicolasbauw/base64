# base64-lt

[![Current Crates.io Version](https://img.shields.io/crates/v/base64-lt.svg)](https://crates.io/crates/base64-lt)
[![Downloads badge](https://img.shields.io/crates/d/base64-lt.svg)](https://crates.io/crates/base64-lt)

A base64 (with padding) string encoding/decoding utility.

You can use it in two ways:
- Interactive mode : you run it, type your text, and press enter
- Non-interactive mode : you echo your text and pipe it to base64-lt.

The -d commutator is for decoding. Tested on MacOS / Linux / Windows.

Examples:
```
base64-lt
Test
VGVzdA==
````

```text
base64-lt -d
VGVzdA==
Test
````

```text
echo "VGVzdA==" | base64-lt -d
Test
```

License: GPL-3.0
