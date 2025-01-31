# TimeRead

A simple cli utility to estimate how long it takes to read a file. The buffer size used can be tuned via cli args.

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
