# TimeRead

A simple cli utility to estimate how long it takes to read a file. The buffer size used can be tuned via cli args.

## Usage

Default buffer size: 4KB

```shell
timeread <file>
```

1MB read buffer

```shell
timeread <file> 1048576
```

Sample output

```shell
$ timeread src/main.rs
> Metadata size: 2KB. Actually read: 2KB. Took time: 3.878µs. Throughput: 737MB/s
```
