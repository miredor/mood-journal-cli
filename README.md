# mood-journal-cli

A small random Rust project: a terminal journal app with mood tracking, tags, prompts, and simple stats.

## Features

- Add journal entries with a mood score from 1 to 5
- Attach comma-separated tags
- List recent entries
- Filter entries by tag
- Show average / best / worst mood stats
- Generate a random reflection prompt
- Store data in JSON locally

## Installation

```bash
cargo build --release
```

## Usage

### Add an entry

```bash
cargo run -- add --mood 4 --tags work,rust "Finished a fun CLI project"
```

### List entries

```bash
cargo run -- list
cargo run -- list --limit 5
cargo run -- list --tag rust
```

### View stats

```bash
cargo run -- stats
```

### Get a prompt

```bash
cargo run -- prompt
```

## Custom data file

By default the app stores data in your local application data directory.
You can override it:

```bash
cargo run -- --file ./my-journal.json add --mood 5 --tags ideas "Ship it"
```

## Example session

```bash
cargo run -- --file ./demo.json add --mood 5 --tags rust,cli "Built a tiny journaling tool"
cargo run -- --file ./demo.json add --mood 3 --tags work "Lots of meetings today"
cargo run -- --file ./demo.json list
cargo run -- --file ./demo.json stats
```

## Tests

```bash
cargo test
```
