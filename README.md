# mdns_responder - Rust mDNS responder

mdns_responder is a pure rust implementation of the mDNS ([RFC 6762]) and DNS-SD ([RFC 6763]) protocols.

## Usage

To use it, first add this to your `Cargo.toml`:

```toml
[dependencies.mdns-responder]
git = "https://github.com/plietar/rust-mdns-responder"
```

Then, add this to your crate root:

```rust
extern crate mdns_responder;
```

[RFC 6762]: https://tools.ietf.org/html/rfc6762
[RFC 6763]: https://tools.ietf.org/html/rfc6763
