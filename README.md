# Miqat

A Rust native library for computing prayer times based on astronomical calculations

## Building for Android

The Android library (AAR) is built from the Rust core via [cargo-ndk](https://github.com/bbqsrc/cargo-ndk) and [UniFFI](https://mozilla.github.io/uniffi-rs/).

### Prerequisites

- The Rust Android targets (installed automatically by `rust-toolchain.toml`).
- The `cargo-ndk` CLI: `cargo install cargo-ndk`
- Android SDK, plus NDK `27.3.13750724` (see `android/buildSrc/src/main/kotlin/GradleConfig.kt`).
- `android/local.properties` pointing at your SDK — this file is machine-specific and gitignored, so a fresh clone must create it:

  ```properties
  sdk.dir=/path/to/Android/sdk
  ```

- For **release** builds only, the nightly toolchain and `rust-src` (used for `-Zbuild-std`):

  ```sh
  rustup toolchain install nightly
  rustup component add rust-src --toolchain nightly
  ```

### Build

```sh
just android        # debug AAR
just android -r     # release AAR (optimized, panic=abort std)
```

The AAR is produced at `android/miqat/build/outputs/aar/`.

## Contributing

Please see the `CONTRIBUTING.md` file for more information.

## Code of Conduct

Our contributor code of conduct can be found in the `code-of-conduct.md` file.

## Acknowledgement

This library is a fork of [salah](https://github.com/insha/salah), which is based on the [Adhan](https://github.com/batoulapps/Adhan) library by Batoul Apps. All astronomical calculations are high precision equations directly from the book [Astronomical Algorithms](http://www.willbell.com/math/mc1.htm) by Jean Meeus.
