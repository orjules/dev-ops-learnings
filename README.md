# Dev Ops Learnings

I wanted to learn some basic dev ops, so I made this repo as an example.

## Goals

- [x] Create a simple testable project
- [x] Create releases with tags
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

## Tags and releases

To tag a commit it is apparently best practice to use the annotated tags like so:

```bash
git tag -a v0.1.0 -m"Release v0.1.0"
```

These of course need to be pushed to GitHub like so:

```bash
git push origin v0.1.0
```

Alternatively, GitHub allows to create a new tag on the dialogue where a new release is created.

To create the release, simply use the GitHub feature for it.
It's pretty self explanatory.
