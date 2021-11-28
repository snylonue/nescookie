# nescookie

A netscape cookie file parser.

**[docs.rs](https://docs.rs/nescookie/0.3.0)** | **[crates.io](https://crates.io/crates/nescookie)**

# Usage

```rust
// open a file directly
let jar = nescookie::open("/path/to/cookie/file").unwrap();

// parse a string
let content = ".pixiv.net	TRUE	/	TRUE	1784339332	p_ab_id	7\n";
let jar = nescookie::parse(content).unwrap();

// parse to an exist `CookieJar`
let builder = nescookie::CookieJarBuilder::with_jar(existed_jar);
// res is a `CookieJar`
let res = builder.open("/path/to/cookie/file").unwrap().finish(); // or builder.parse(content)...
```
