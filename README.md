# bevy_chess_tutorial

:round_pushpin: Init

- [ ] Create project

```
cargo new bevy_chess_tutorial && cd bevy_chess_tutorial
```
>    Created binary (application) `bevy_chess_tutorial` package

- [ ] Add `bevy` Library

```
cargo add bevy
```

- [ ] Enable `nightly` Rust

```
rustup toolchain install nightly 
```
> Returns :
```yaml
info: syncing channel updates for 'nightly-x86_64-apple-darwin'
info: latest update on 2023-05-29, rust version 1.72.0-nightly (1c53407e8 2023-05-28)
info: downloading component 'rust-std' for 'aarch64-apple-ios'
info: downloading component 'rust-src'
info: downloading component 'rust-std' for 'x86_64-apple-ios'
info: downloading component 'cargo'
info: downloading component 'clippy'
info: downloading component 'rust-docs'
info: downloading component 'rust-std'
info: downloading component 'rustc'
info: downloading component 'rustfmt'
info: removing previous version of component 'rust-std' for 'aarch64-apple-ios'
info: removing previous version of component 'rust-src'
info: removing previous version of component 'rust-std' for 'x86_64-apple-ios'
info: removing previous version of component 'cargo'
info: removing previous version of component 'clippy'
info: removing previous version of component 'rust-docs'
info: removing previous version of component 'rust-std'
info: removing previous version of component 'rustc'
info: removing previous version of component 'rustfmt'
info: installing component 'rust-std' for 'aarch64-apple-ios'
 24.1 MiB /  24.1 MiB (100 %)  12.6 MiB/s in  2s ETA:  0s
info: installing component 'rust-src'
info: installing component 'rust-std' for 'x86_64-apple-ios'
 24.2 MiB /  24.2 MiB (100 %)  12.1 MiB/s in  2s ETA:  0s
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'rust-docs'
 13.6 MiB /  13.6 MiB (100 %)   3.3 MiB/s in  3s ETA:  0s
info: installing component 'rust-std'
 24.5 MiB /  24.5 MiB (100 %)  11.8 MiB/s in  2s ETA:  0s
info: installing component 'rustc'
 55.3 MiB /  55.3 MiB (100 %)  14.3 MiB/s in  3s ETA:  0s
info: installing component 'rustfmt'

  nightly-x86_64-apple-darwin updated - rustc 1.72.0-nightly (1c53407e8 2023-05-28) (from rustc 1.71.0-nightly (f9a6b7158 2023-05-05))

info: checking for self-update
```

```
rustup toolchain list      
```
> Returns :
```yaml
stable-x86_64-apple-darwin (default)
nightly-x86_64-apple-darwin
```


# References

- [ ] [Chess game in Rust using Bevy](https://caballerocoll.com/blog/bevy-chess-tutorial)
