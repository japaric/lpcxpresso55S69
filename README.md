# `lpcxpresso55s69`

> [Prototype] Real Time for The Masses on the homogeneous dual core LPC55S69 (2x M33)

## Running the examples

The onboard debugger comes flashed with CMSIS-DAP firmware but I didn't have
much luck with it so I replaced it with [J-LINK firmware][jlink].

[jlink]: https://www.segger.com/products/debug-probes/j-link/models/other-j-links/lpcxpresso-on-board/

``` console
$ # on a terminal
$ JLinkGDBServer -device LPC55S69 -if SWD
```

``` console
$ # on another terminal
$ rustup default beta # or Rust 1.36 when that's out

$ rustup target add thumbv8m.main-none-eabi

$ cd lpc55s6x

$ cargo run --example xschedule --release

(gdb) continue
```

(If you get a linker error when you build an example without optimizations then
re-compile with `--release` -- I don't have time to fix the linker error right
now)

This example should blink the RGB led on the board.

Note that flashing sometimes fails so you may need to re-rerun the `cargo run`
command.

## License

All source code (including code snippets) is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  [https://www.apache.org/licenses/LICENSE-2.0][L1])

- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  [https://opensource.org/licenses/MIT][L2])

[L1]: https://www.apache.org/licenses/LICENSE-2.0
[L2]: https://opensource.org/licenses/MIT

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.
