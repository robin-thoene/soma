# soma

## Summary

A fast, easy-to-use, privacy-respecting, and fully open-source tracker for body weight and body
composition.

## Local development

Run the application

```shell
cargo run
```

Run the application with hot reloading (using [watchexec](https://github.com/watchexec/watchexec))

```shell
watchexec -r -- cargo run
```

Run the tests

```shell
cargo test
```

Run the tests with hot reloading (using [watchexec](https://github.com/watchexec/watchexec))

```shell
watchexec -r -- cargo test
```

Check if you violated the hexagonal architecture dependency rules

```shell
./scripts/lint_architecture.sh
```

Check the workspace rules (using [cargo-deny](https://github.com/EmbarkStudios/cargo-deny))

```shell
cargo deny check
```
