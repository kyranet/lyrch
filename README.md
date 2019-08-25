# Lyrch [![Discord](https://discordapp.com/api/guilds/254360814063058944/embed.png)](https://skyra.pw/join)

Skyra VI development repository. Feel free to go to [Skyra V]'s repository for the TypeScript version.

[Skyra V]: https://github.com/kyranet/Skyra

## Development Requirements

- [`Rust`] with [`rustfmt`] and [`rust-clippy`]: To build, run, format, and lint the project.
- [`PostgreSQL`]: To store persistent data.
- [`Redis`]: To store cache data.

[`Rust`]: https://www.rust-lang.org/tools/install
[`rustfmt`]: https://github.com/rust-lang/rustfmt
[`rust-clippy`]: https://github.com/rust-lang/rust-clippy
[`PostgreSQL`]: https://www.postgresql.org/download/
[`Redis`]: https://redis.io/download

## Set-Up

Copy and paste the [`.env.example`] file and rename it to `.env`, then fill it with the precise variables. Once all
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

> **Note**: Before pushing to the repository, please run `cargo fmt` (`rustup component add rustfmt`) and `cargo clippy`
(`rustup component add clippy`) so formatting stays consistent and there are no linter warnings. The first time you run
clippy will be very slow, but afterwards it will run very, very, fast.

[`.env.example`]: /.env.example

## NyProject Network

- [`NeuLink`]: Game Artificial Intelligence Server, used for game commands, **this is not required for the basic set-up**.
Requires [`.NET Core 3.0`].
- [`Ryana`]: Image Rendering Generation Server, used for image commands, **this is not required for the basic set-up**.
Requires [`.NET Core 3.0`].

[`NeuLink`]: https://github.com/kyranet/neulink
[`Ryana`]: https://github.com/kyranet/ryana
[`.NET Core 3.0`]: https://dotnet.microsoft.com/download/dotnet-core/3.0
