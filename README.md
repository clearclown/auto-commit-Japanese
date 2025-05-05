# acommit

## Automagically-generated commit messages

A CLI tool that generates commit messages from your staged changes, built in Rust and using [gpt-4o-mini](https://platform.openai.com/overview).

## Installation

You can install `acommit` by running the following command in your terminal.

```
git clone https://github.com/clearclown/auto-commit-Japanese.git

cd auto-commit-Japanese

cargo build --release

ln -sf "$(pwd)/target/release/auto-commit" .bin/acommit
```

## Usage

`auto-commit` uses GPT-3.5. To use it, grab an API key from [your dashboard](https://beta.openai.com/), and save it to `OPENAI_API_KEY` as follows (you can also save it in your bash/zsh profile for persistance between sessions).

```bash
export OPENAI_API_KEY='sk-XXXXXXXX'
```

Once you have configured your environment, stage some changes by running, for example, `git add .`, and then run `auto-commit`.

Of course, `auto-commit` also includes some options, for editing the message before commiting, or just printing the message to the terminal.

```sh
$ acommit --help
Automagically generate commit messages.

Usage: auto-commit [OPTIONS]

Options:
  -v, --verbose...  More output per occurrence
  -q, --quiet...    Less output per occurrence
      --dry-run     Output the generated message, but don't create a commit.
  -r, --review      Edit the generated commit message before committing.
  -h, --help        Print help information
  -V, --version     Print version information
```

## Develop

Make sure you have the latest version of rust installed (use [rustup](https://rustup.rs/)). Then, you can build the project by running `cargo build`, and run it with `cargo run`.

## License

This project is open-sourced under the MIT license. See [the License file](LICENSE) for more information.
