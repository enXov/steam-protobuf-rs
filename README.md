# steam-protobuf-rs

[![Crates.io](https://img.shields.io/crates/v/steam-protobuf-rs.svg)](https://crates.io/crates/steam-protobuf-rs)
[![License](https://img.shields.io/crates/l/steam-protobuf-rs.svg)](LICENSE-MIT)

Auto-updated, type-safe Steam Protobuf definitions for Rust.

Maintaining up-to-date Steam Protobuf definitions in Rust is notoriously difficult. **steam-protobuf-rs** automatically extracts Protobuf descriptors from Steam client binaries, converts them into type-safe Rust structs via [prost](https://crates.io/crates/prost), and publishes them to [crates.io](https://crates.io/crates/steam-protobuf-rs).

## Usage

Add the latest version to your project:

```bash
cargo add steam-protobuf-rs
```

Then use the types directly:

```rust
use steam_protobuf_rs::CMsgClientLogon;
use prost::Message;

fn main() {
    let logon = CMsgClientLogon {
        protocol_version: Some(65580),
        cell_id: Some(0),
        client_language: Some("english".into()),
        ..Default::default()
    };

    // Encode
    let bytes = logon.encode_to_vec();

    // Decode
    let decoded = CMsgClientLogon::decode(bytes.as_slice()).unwrap();
    assert_eq!(decoded.protocol_version, Some(65580));
}
```

> No Steam installation required. No `protoc` required. No build-time code generation. Just `cargo add` and start coding.

## How It Works

```text
Valve Steam Update
        ↓
GitHub Actions (scheduled cron)
        ↓
Download Steam binaries from Valve CDN
        ↓
Extract .proto files (steam-protobuf-dumper)
        ↓
Diff check - stop if unchanged
        ↓
prost-build → Rust types
        ↓
Version bump → cargo publish
```

The extraction is powered by [steam-protobuf-dumper](https://github.com/enXov/steam-protobuf-dumper).

## Ecosystem Vision

**steam-protobuf-rs** is the foundational layer for building Steam tools in Rust:

```text
                  steam-protobuf-rs
                 /                 \
                /                   \
           steamkit               depot-downloader
```

This crate provides **only** the message types. It does not include CM networking, authentication, content downloading, or any high-level Steam API logic - those belong in separate crates.

### Why this exists

I needed a depot downloader written in Rust for an internal tool. After doing a lot of research, I found that crates.io had become a graveyard for Steam protobuf packages - unmaintained, incomplete, or with protobuf files scattered in random places.

The [Steamworks](https://crates.io/crates/steamworks) crate is great, but there was no equivalent for Steam's raw Protobufs. If you wanted type-safe CM messages, you had to start from scratch: figure out what Protobuf is, learn how to dump them from Steam binaries, wrangle `prost-build`, and repeat the whole thing every time Valve pushes an update.

So I built this crate. It's not a plug-and-play Steam client - it's a developer-experience-first foundation. You get always-current protobuf types with zero setup, and you can build whatever you want on top: a SteamKit-style client, a depot downloader, or anything else.

Yes, the `-rs` suffix is a bit cliché (uwu)

## Credits

- [steam-protobuf-dumper](https://github.com/enXov/steam-protobuf-dumper) - Extracts Protobuf definitions from Steam binaries

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
