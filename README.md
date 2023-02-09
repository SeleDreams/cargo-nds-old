# cargo-nds

Cargo command to work with Nintendo DS project binaries. Based on cargo-3ds.

## Usage

Use the nightly toolchain to build DS apps (either by using `rustup override nightly` for the project directory or by adding `+nightly` in the `cargo` invocation).

```txt
Commands:
  build
          Builds an executable suitable to run on a 3DS (3dsx)
  run
          Builds an executable and sends it to a device with `3dslink`
  test
          Builds a test executable and sends it to a device with `3dslink`
  help
          Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help information (use `-h` for a summary)

  -V, --version
          Print version information
```

Additional arguments will be passed through to the given subcommand.
See [passthrough arguments](#passthrough-arguments) for more details.

It is also possible to pass any other `cargo` command (e.g. `doc`, `check`),
and all its arguments will be passed through directly to `cargo` unmodified with the proper `RUSTFLAGS`
You will however need to specify the target json as the nds target isn't yet supported by rust officially

### Basic Examples

* `cargo nds build`
* `cargo nds check --verbose`
* `cargo nds test --no-run`

### Passthrough Arguments

Due to the way `cargo-nds` and `cargo` parse arguments, there is
unfortunately some complexity required when invoking an executable with arguments.

All unrecognized arguments beginning with a dash (e.g. `--release`, `--features`,
etc.) will be passed through to `cargo` directly.
