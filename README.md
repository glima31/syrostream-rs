# syrostream-rs

[![CI](https://github.com/glima31/syrostream-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/glima31/syrostream-rs/actions/workflows/ci.yml)
[![MIT license](https://img.shields.io/badge/License-MIT-blue.svg)](https://lbesson.mit-license.org/)

Simple crate for encoding audio into [Syrostream](https://github.com/korginc/volcasample) format, used to load samples onto the Korg Volca Sample.

This crate was initially developed as part of an ongoing project (repo currently private) but I eventually found myself needing it somewhere else, so I decided to extract it into its own repository and make it public.

Please note that this library does **not** aim to expose a complete Rust API for the Syro SDK in its entirety. Instead, i designed it with only the Syrostream encoding functionality in mind.

## Workspace

This is a Cargo workspace with two crates:

- **syro-sys** — Raw FFI bindings to the [Korg Syro SDK](https://github.com/korginc/volcasample)
- **syrostream** (root package) — Safe Rust API for encoding audio into Syrostream format

## Usage

First, make sure you add the crate as dependency in your `Cargo.toml`:
```toml
[dependencies]
syrostream = { git = "https://github.com/glima31/syrostream-rs.git" }
```

Then, to encode your audio:
```rust
use std::num::NonZeroU32;

let src_audio: &[i16] = &[/* ...audio samples... */];

// The sample rate of `src_audio`. The syrostream output sample rate is always 44.1kHz,
// but your audio source can be different.
let src_rate = NonZeroU32::new(22_050).unwrap();

// Load sample into slot 0. Only values from 0 to 99 are accepted.
let slot = 0; 

let syro_output = syrostream::encode(src_audio, src_rate, slot).unwrap();
```

The output is stereo interleaved `i16` samples at 44,100 Hz, ready to be written to a WAV file and played back into the Volca Sample's SYNC input.

## Building

When cloning this repo, make sure you include `--recurse-submodules` in your git command:
```sh
git clone --recurse-submodules https://github.com/glima31/syrostream-rs.git
```

Once the repository is cloned, just run the usual `cargo build` and you're ready to roll. 

## License

This project is licensed under the MIT license. The Korg Syro SDK (included as a submodule) is licensed under the BSD-3-Clause license. See `syro-sys/volcasample/COPYING` for details.
