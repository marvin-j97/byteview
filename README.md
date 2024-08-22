# thin-slice

[![CI](https://github.com/fjall-rs/thin-slice/actions/workflows/test.yml/badge.svg)](https://github.com/fjall-rs/thin-slice/actions/workflows/test.yml)
[![docs.rs](https://img.shields.io/docsrs/thin-slice?color=green)](https://docs.rs/thin-slice)
[![Crates.io](https://img.shields.io/crates/v/thin-slice?color=blue)](https://crates.io/crates/thin-slice)
![MSRV](https://img.shields.io/badge/MSRV-1.70.0-blue)

An immutable byte slice that may be inlined, and can be partially cloned without heap allocation.

## Memory usage

Allocating 200M "helloworld" (len=10) strings:

|  Struct         | Memory Usage |
|-----------------|--------------|
| `Arc<[u8]>`     | 12.8 GB      |
| `tokio::Bytes`  | 6.4 GB       |
| `ThinSlice`     | 4.8 GB       |

Allocating 100M "helloworldhellow" (len=16) strings:

|  Struct         | Memory Usage |
|-----------------|--------------|
| `Arc<[u8]>`     | 6.4 GB       |
| `tokio::Bytes`  | 6.4 GB       |
| `ThinSlice`     | 5.6 GB       |

Allocating 100M "helloworldhelloworld" (len=20) strings:

|  Struct         | Memory Usage |
|-----------------|--------------|
| `Arc<[u8]>`     | 6.4 GB       |
| `tokio::Bytes`  | 6.4 GB       |
| `ThinSlice`     | 7.2 GB       |

Allocating 5M `"helloworld".repeat(100)` (len=1000) strings:

|  Struct         | Memory Usage |
|-----------------|--------------|
| `Arc<[u8]>`     | 5.2 GB       |
| `tokio::Bytes`  | 5.2 GB       |
| `ThinSlice`     | 5.2 GB       |
