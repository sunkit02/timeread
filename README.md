# TimeRead

A simple cli utility to estimate how long it takes to read a file or directory
and outputs some useful statistics. The buffer size used can be tuned via cli
args.

## Notice

- This utility has not been rigorously tested and it's output should only be
  used as reference.
- Units in output are calculated in the order of 1024 instead of 1000.
  - e.g. 1KB = 1024B, 1MB = 1024KB, etc

## Usage

```
timeread <path> [buffer size (bytes)]
```

Default buffer size: 4KB

```shell
timeread <path>
```

1MB read buffer

```shell
timeread <path> 1048576
```

Sample output

```shell
$ timeread src/main.rs
> Size (in metadata): 2KB. Actually read: 2KB. Took time: 3.878µs. Throughput: 737MB/s
```

## Installation

This project was developed on the 2021 edition of stable rust version 1.84.0
and not tested on any other version.

Toolchain used:

```
stable-x86_64-unknown-linux-gnu (default)
rustc 1.84.0 (9fc6b4312 2025-01-07)
```

To install the package, simply install the above rust toolchain and run

```shell
cargo install --path <path to project root>`.
```
