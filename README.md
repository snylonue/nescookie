# nescookie

A netscape cookie file parser using pest.

**[docs.rs](https://docs.rs/nescookie/0.2.0)** | **[crates.io](https://crates.io/crates/nescookie)**

# Usage

```rust
let jar = nescookie::open("/path/to/cookie/file").unwrap();
```