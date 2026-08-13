# Dev Ops Learnings

I wanted to learn some basic dev ops, so I made this repo as an example.

## Goals

- [x] Create a simple testable project
- [ ] Create releases with tags
- [ ] Setup CI pipeline for testing
- [ ] Use the badges everybody has in their READMEs

## Simple project

The project I chose was to create a CLI to convert units.

To run it use:

```bash
$ cargo run -- ft-m 3.281
1
```

But more relevant would be to test it with:

```bash
cargo test
```
