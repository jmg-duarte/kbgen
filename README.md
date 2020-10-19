# kbgen

`kbgen` or *K*nowledge *B*ased *G*enerator is a tool to generate Markdown for a [Foam](https://github.com/foambubble/foam) knowledge base.

This tool is highly opinionated and may not fit your personal workflow,
however, I welcome any suggestions!

## Usage

`kbgen` has two commands, `article` and `note`.

Both commands are preceded by a destination file, if the file exists `kbgen` will throw an error.

### Article

The `article` command generates a file suited for article summaries.
This command takes the following fields:

| Flag           | Name    | Description         |
| -------------- | ------- | ------------------- |
| `-t`           | Title   | The article title   |
| `-a`           | Authors | The article authors |
| `-d` / `--doi` | DOI     | The DOI reference   |

### Note

The `note` command generates a file for notes.
As of now, the file is a simple H1 header.

| Flag | Name  | Description    |
| ---- | ----- | -------------- |
| `-t` | Title | The note title |

## Help

To print the help text you can either run `kbgen` without any arguments or `kbgen -h`.

## Installation

Currently, the only way to install `kbgen` is by running the following command:

```
cargo install --git https://github.com/jmg-duarte/kbgen --branch main
```

