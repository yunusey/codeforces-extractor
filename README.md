# codeforces-extractor
Blazingly fast codeforces problem extractor! *Rust btw!*

> [!WARNING] Old Version of the Project
> Before, the tool was written in Python. If you want to stick to the old version, you can check out the [`python` branch](https://github.com/yunusey/codeforces-extractor/tree/python).

## Installation 📦
You can install this tool using:
```bash
cargo install --git https://github.com/yunusey/codeforces-extractor
```

## Usage 📝
Using this tool is pretty simple. The command takes two arguments:
```bash
codeforces-extractor <contest_id> --save-path <path>
```
You need to provide both the arguments (they do not have default values).

## Features 🚀
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

## Use with Neovim 🛠️
The main reason for this tool is to be used with Neovim. You can check the plugin [here](https://github.com/yunusey/codeforces-nvim).
