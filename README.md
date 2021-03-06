# nub
Simple command line tool to delete all duplicate files in a given directory

## Installation

0. If you haven't installed *Rust* / *rustup* yet, go to [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) and follow the instructions for your operating system. 
1. `rustup update`
2. `cargo install --git https://github.com/k-gruenberg/nub`

## Usage

```
nub 1.0.0
Kendrick Grünberg
Simple command line tool to delete all duplicate files in a given directory

USAGE:
    nub <DIRECTORY>

ARGS:
    <DIRECTORY>    Path to a directory containing files

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```

## Why the name 'nub'?!

`nub` (meaning "essence") is the Haskell function that removes duplicate elements from a list:

```
Input: nub [0,1,2,3,2,1,0]
Output: [0,1,2,3]
```

## ToDo

* Add a `--recursive` flag/feature to also apply `nub` to all subfolders and to their subfolders etc...
* Add a `--strictRecursive` (or `--totalRecursive`) flag/feature to also delete duplicates **across** subfolders!
* Add a `--images` flag/feature to also delete all duplicate images / image duplicates, **even** when their hashes are different. Possibly with a `--tolerance [TOLERANCE]` argument to specify how tolerant to be when comparing images for equality, `0.0` meaning all images are regarded as equal and `1.0` meaning that only the metadata of the images may be different.