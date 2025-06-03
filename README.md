# group-runner

Group Rust executable output in GitHub logs

`group-runner` is useful, e.g., when you have lots of integration tests and seeing their output concatenated can be overwhelming.

**Note:** When designing your testsuite, be sure to consider [@matklad]'s excellent blogpost, [Delete Cargo Integration Tests].

## Example `group-runner` output

<img src="etc/screenshot.png">

## Recommended usage with one Cargo command

1. In your GitHub workflow, install `group-runner`:

   ```yaml
   steps:
     - name: Install group-runner
       run: cargo install group-runner
   ```

2. Pass the following option to `cargo run`, `cargo test`, or `cargo bench`:
   ```sh
   --config "target.'cfg(all())'.runner = 'group-runner'"
   ```
   Example:
   ```yaml
   steps:
     - name: Test
       run: cargo test --config "target.'cfg(all())'.runner = 'group-runner'"
   ```
   See [The Cargo Book] for more information.

## Recommended usage with multiple Cargo commands

Like [above], however, we recommend storing the configuration in an environment variable. Example:

```yaml
env:
  GROUP_RUNNER: target.'cfg(all())'.runner = 'group-runner'
steps:
  - name: Test foo
    run: cargo test --package foo --config "$GROUP_RUNNER"
  - name: Test bar
    run: cargo test --package bar --config "$GROUP_RUNNER"
```

## Notes

To avoid mixing build output with test output, we recommend building tests in a separate step prior to running them. Example:

```yaml
steps:
  - name: Build
    run: cargo test --no-run
  - name: Test
    run: cargo test --config "target.'cfg(all())'.runner = 'group-runner'"
```

[@matklad]: https://github.com/matklad
[Delete Cargo Integration Tests]: https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html
[The Cargo Book]: https://doc.rust-lang.org/cargo/reference/config.html#targettriplerunner
[above]: #recommended-usage-with-one-cargo-command
