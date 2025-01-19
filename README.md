# byteview

[![CI](https://github.com/marvin-j97/byteview/actions/workflows/test.yml/badge.svg)](https://github.com/marvin-j97/byteview/actions/workflows/test.yml)
[![CI](https://github.com/marvin-j97/byteview/actions/workflows/miri.yml/badge.svg)](https://github.com/marvin-j97/byteview/actions/workflows/miri.yml)
[![docs.rs](https://img.shields.io/docsrs/byteview?color=green)](https://docs.rs/byteview)
[![Crates.io](https://img.shields.io/crates/v/byteview?color=blue)](https://crates.io/crates/byteview)
![MSRV](https://img.shields.io/badge/MSRV-1.65-blue)

An immutable byte slice that may be inlined, and can be partially cloned without heap allocation.

![Memory layout](./byteview.png)

`byteview` was designed to speed up deserialization in `lsm-tree`, allow inlining of small values and reduce memory usage compared to Arc'd slices.
Values with a known length can be constructed 2-2.5x faster than using Arcs:

![Constructor benchmark](ctor_bench.png)

## Memory usage

Allocating 200M "" (len=0) strings:

|  Struct         | Memory Usage |
|-----------------|--------------|
| `Arc<[u8]>`     | 9.6 GB      |
| `tokio::Bytes`  | 6.4 GB       |
| `ByteView`     | 4.8 GB       |

Allocating 200M "helloworld" (len=10) strings:

|  Struct         | Memory Usage |
|-----------------|--------------|
| `Arc<[u8]>`     | 12.8 GB      |
| `tokio::Bytes`  | 12.8 GB       |
| `ByteView`     | 4.8 GB       |

Allocating 100M "helloworldhelloworld" (len=20) strings:

|  Struct         | Memory Usage |
|-----------------|--------------|
| `Arc<[u8]>`     | 6.4 GB       |
| `tokio::Bytes`  | 6.4 GB       |
| `ByteView`     | 2.4 GB       |

Allocating 50M "helloworldhelloworldhelloworldhelloworld" (len=30) strings:

|  Struct         | Memory Usage |
|-----------------|--------------|
| `Arc<[u8]>`     | 4.0 GB       |
| `tokio::Bytes`  | 4.0 GB       |
| `ByteView`     | 3.6 GB       |

Allocating 500k `"helloworld".repeat(1000)` (len=10'000) strings:

|  Struct         | Memory Usage |
|-----------------|--------------|
| `Arc<[u8]>`     | 5 GB       |
| `tokio::Bytes`  | 5 GB       |
| `ByteView`     | 5 GB       |

## Run fuzz tests

```bash
cargo +nightly fuzz run fuzz_target_1
```