# Zat

> gimme zat

We have `head` to display the first `n` lines of a file, and we have `tail` to display the last `n` lines of a file, but I haven't heard of a program that does the opposite of those two: display a range of lines *inside* of a file.

This program lets you do exactly that!

```
zat file.txt
```
```
one
two
three
four
five
six
```
```
zat file.txt -s 2 -e 4
```
```
two
three
four
```

Use `-s`/`--start` to specify the first line you want to display, and up to the `-e`/`--end` line. Both the start and the end are *inclusive*.

Lines are 1-indexed, but 0 also means 1.
Specifying `-s 0 -e 0` will just display the first line. The more natural way of doing this is `-e 1`.

Both flags are optional, so you could use this as a `cat` that doesn't concatenate (because cat does it already).

If you want to display a range of lines of stdin, rather than a file, just don't specify the file argument.

```
git --help | zat -s 12 -e 16
```

## Usage

```
Print a range of lines of a file. Both --start and --end are inclusive. Line numbers are 1 indexed, but 0 also means 1.

Usage: zat [FILE] [OPTIONS]

Options:
  -s, --start <NUM>
  -e, --end <NUM>
  -h, --help           Print help (see more with '--help')
  -V, --version        Print version
```

## Install

```
cargo install zat
```

`cargo binstall` and `cargo quickinstall` are also supported.

## Uninstall

```
cargo uninstall zat
```