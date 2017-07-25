# mbr

A rust crate to read legacy MBR disk partitions

> Warning: This crate is early in development and may encounter API changes as it grows.
> When the version hits 1.x.x the API will be stable

## What about GPT partition tables?

There is a [great GPT parsing library](https://crates.io/crates/gpt) created by another author.
I'm tracking their API somewhat to ensure applications parsing MBR and GPT tables generally follows the same process.

## License

Copyright, 2017 Alexander von Gluck IV
Released under the terms of the MIT license.

