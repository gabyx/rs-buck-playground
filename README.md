# Building Rust with Buck

This is simple demo/playground project to play with `buck2`.

Buck [`buck`](https://github.com/facebook/buck2) is a build tool designed for
multi-language mono-repositories.

In this repository we are going to provide the toolchain with `flake.nix` over
`nix develop .` which provides the `buck2` executable too. We build a simple
Rust executable in [`src/main.rs`](src/main.rs) with dependencies in a
`Cargo.toml` file. The build tool `buck` needs these dependencies converted to
`buck` build files, which is done with
[`reindeer`](https://github.com/facebookincubator/reindeer).

## Requirements

- Nix Environment [see here](https://nixos.org/download/).
- [optional] `direnv` installed.

## Build the Project

If you have `direnv` installed it will load the development shell from the
`flake.nix`. when you enter the repository, otherwise do the following:

```shell
nix develop .
```

### Build with Buck2

**Convert the external dependencies to `buck`**:

```shell
just buckify
```

**See all targets** with

```shell
buck targets //...
```

**Build and run a target** with

```shell
buck run //src:main
```

# Inspirations

- [buck-nix](https://github.com/thoughtpolice/buck2-nix)
- [buck2-example](https://github.com/cormacrelf/buck2-example)
