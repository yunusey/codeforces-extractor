# codeforces-extractor
Blazingly fast Codeforces test extractor! *Rust btw!*

> [!CAUTION]
> Before, the project was written in Python. Due to several reasons, it is now rewritten in Rust and is more performant and convenient to use.

## Installation
You can install this tool using:
```bash
cargo install --git https://github.com/yunusey/codeforces-extractor
```

### Requirements
Because Codeforces is behind a bot challenge, this tool launches a real Chromium-based browser via [`chromiumoxide`](https://crates.io/crates/chromiumoxide). You need one of the following available:

- `chromium`, `google-chrome`, or another Chromium-based browser on your `PATH`, **or**
- the `CHROME` environment variable pointing at the browser executable

Firefox, Zen, and other non-Chromium browsers are **not** supported.

If you see `Failed to get content for contest: ...`, the underlying cause is often that no Chrome/Chromium binary could be detected (`Could not auto detect a chrome executable`). Fix it by installing Chromium/Chrome, or by setting:

```bash
export CHROME=/path/to/chromium
codeforces-extractor 1790
```

## Usage
Using this tool is pretty simple. You can run `codeforces-extractor --help` to see the usage:
```bash
Usage: codeforces-extractor [OPTIONS] <CONTEST_NAME>

Arguments:
  <CONTEST_NAME>  Contest Name

Options:
  -s, --save-path <SAVE_PATH>  Save Path [default: ./problems]
  -n, --native-display         Use native display (no Xvfb)
  -h, --help                   Print help
```

Passing `--native-display` as an argument will launch `chromium` (or `google-chrome`) in your native display which will pop up a window. Though it works more reliably, most users would not, probably, prefer it. That's why, by default, `codeforces-extractor` will open up a virtual display instead using `Xvfb`. Again, my recommendation is, if it doesn't bother you, to use `--native-display` as it is a lot more reliable.

## Features
The tool will save the test cases like this:
```
 ./test
├──  A
│  ├──  0.in
│  └──  0.out
├──  B
│  ├──  0.in
│  └──  0.out
├──  C
│  ├──  0.in
│  └──  0.out
├──  D
│  ├──  0.in
│  └──  0.out
├──  E
│  ├──  0.in
│  └──  0.out
├──  F
│  ├──  0.in
│  └──  0.out
└──  G
   ├──  0.in
   └──  0.out
```

## Use with Neovim
This tool acts as the backend for my Neovim plugin, [codeforces-nvim](https://github.com/yunusey/codeforces-nvim), but can be used directly through the CLI, as described above.

## Tests

If you would like to run the tests yourself, please do not just run `cargo test` because there are, as of now, two tests fetching and parsing, and they cannot run simultaneously--I don't really know why though :D--that's why, you need to run them one by one using `cargo test -- --test-threads=1`.
