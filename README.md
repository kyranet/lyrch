# Lyrch

This repository holds the private code for Experimental Skyra VI. Using, sharing, duplicating, or any other forms of
redistrubition of this code without permission is strictly forbidden.

## Development Requirements

- [`Rust`] with [`rustfmt`] and [`rust-clippy`]
- [`PostgreSQL`]
- [`Redis`]

[`Rust`]: https://www.rust-lang.org/tools/install
[`rustfmt`]: https://github.com/rust-lang/rustfmt
[`rust-clippy`]: https://github.com/rust-lang/rust-clippy
[`PostgreSQL`]: https://www.postgresql.org/download/
[`Redis`]: https://redis.io/download

## Set-Up

Copy and paste the [.env.example] file and rename it to `.env`, then fill it with the precise variables. Once all
development requirements are set up:

```bash
# Builds the project as debug
$ cargo build

# Or optionally, run can be used to update dependencies,
# build the project, and run the project.
$ cargo run

# And for production:
$ cargo run --release
```

Also, before pushing to the repository, please run `cargo fmt` (`rustup component add rustfmt`) and `cargo clippy`
(`rustup component add clippy`) so formatting stays consistent and there are no linter warnings. The first time you run
clippy will be very slow, but afterwards it will run very, very, fast.

[`.env.example`]: /.env.example

## NyProject Network

- [`NeuLink`]: https://github.com/kyranet/neulink
- [`Ryana`]: https://github.com/kyranet/ryana
