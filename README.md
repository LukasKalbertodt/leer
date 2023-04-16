# Types that have a notion of “being empty”

[<img alt="CI status of main" src="https://img.shields.io/github/actions/workflow/status/LukasKalbertodt/leer/ci.yml?branch=main&label=CI&logo=github&logoColor=white&style=for-the-badge" height="23">](https://github.com/LukasKalbertodt/leer/actions/workflows/ci.yml)
[<img alt="Crates.io Version" src="https://img.shields.io/crates/v/leer?logo=rust&style=for-the-badge" height="23">](https://crates.io/crates/leer)
[<img alt="docs.rs" src="https://img.shields.io/crates/v/leer?color=blue&label=docs&style=for-the-badge" height="23">](https://docs.rs/leer)

A tiny trait to abstract over types that have a notion of “being empty” and can create such an empty instance.
Intended to be a foundational crate.
See [the documentation](https://docs.rs/leer) for more information.

```rust
pub trait Empty {
    fn empty() -> Self;
}
```

<br />

---

## License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
