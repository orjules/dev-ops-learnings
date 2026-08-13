# Dev Ops Learnings

![Rust CI](https://github.com/orjules/dev-ops-learnings/actions/workflows/rust.yml/badge.svg)

I wanted to learn some basic dev ops, so I made this repo as an example.

## Goals

- [x] Create a simple testable project
- [x] Create releases with tags
- [x] Add license
- [x] Setup CI pipeline for testing
- [x] Use the badges everybody has in their READMEs
- [ ] Setup CD to automatically create releases

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

## License

The website [choosealicense](https://choosealicense.com/) is pretty helpful.

## CI

The workflows (on GitHub at least) live in `.github/workflows`.
The template for rust looks like this, but I added the comments:

```yaml
name: Rust # Simply the name

# When this will be run
on:
  push:
    branches: [ "main" ] # Any push directly to main
  pull_request:
    branches: [ "main" ] # A change in the PR which is pointed at main

env:
  CARGO_TERM_COLOR: always # Cosmetic to have color in the cargo output

jobs: # Each job checks its own run on a VM
  build-and-test: # Template calls it only build

    runs-on: ubuntu-latest # Which VM it uses

    steps: # Then sequential steps
    - uses: actions/checkout@v4 # An official action by GitHub: https://github.com/actions/checkout
    - name: Build # Could be skipped but making this step explicit seems good
      run: cargo build --verbose
    - name: Run tests # The actually interesting part
      run: cargo test --verbose
```

When created the actions run and can be seen in the `Actions` tab.
The commit also gets a small check mark on the main page.

## Badges

GitHub provides the passing badge already at `https://github.com/<user>/<repo>/actions/workflows/<name>.yml/badge.svg`.
