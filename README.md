# Requirements

- Rust (see [rustup.rs](https://rustup.rs/))
- ASDF (see [asdf-vm.com](https://asdf-vm.com/#/core-manage-asdf-vm?id=install))

# Setup

## Install nodejs with asdf

```bash
asdf install
```

## Install cargo-leptos

```bash
cargo install cargo-leptos
```

or if you have binstall installed (avoid building from source)

```bash
cargo binstall cargo-leptos
```

## Install diesel_cli

```bash
cargo install diesel_cli --no-default-features --features [postgres|sqlite|mysql]
```

or if you have binstall installed (avoid building from source) [not recommended]

```bash
cargo binstall diesel_cli --no-default-features
```

> diesel_cli can require lib like libpq-dev, libmysqlclient-dev, libsqlite3-dev
> or in fedora libpq-devel, community-mysql-devel, sqlite-devel

# Usefull links

- https://diesel.rs/guides/getting-started