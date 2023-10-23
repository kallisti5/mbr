# mbr

If you use or enjoy this, [![buy me a coffee at ko-fi](https://storage.ko-fi.com/cdn/kofi4.png?v=3)](https://ko-fi.com/Q5Q2QF1PE)!

A rust crate to read legacy MBR disk partitions

> Warning: This crate moves slowly since the MBR specification is pretty simple! Don't take
> lack of updates as abandonment of this project!  It currently handles most basic usage
> cases, and bugs/pr's are addressed via the issue tracker :-)

## Example Usage

```rust
extern crate mbr;
let partitions = mbr::partition::read_partitions("/dev/sda");
```

## What about GPT partition tables?

There is a [great GPT parsing library](https://crates.io/crates/gpt) created by another author.
I'm tracking their API somewhat to ensure applications parsing MBR and GPT tables generally follows the same process.

## License

Copyright, 2017-2021 Alexander von Gluck IV
Released under the terms of the MIT license.

