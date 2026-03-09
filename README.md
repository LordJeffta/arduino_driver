# Arduino driver

This repository is used to understand how to create drivers for the Arduino Uno.

The main branch is used to start coding with the Arduino, the drivers are held in the other branches.

Template from <https://github.com/Rahix/avr-hal-template>.

## Requirements

- Rust toolchain nightly-2025-04-27
- ravedude 0.2.2

## Build Instructions

1. Install prerequisites as described in the `avr-hal` [README](https://github.com/Rahix/avr-hal#quickstart).

2. Run `cargo build --release` to build the firmware.

3. Run `cargo run --release` to flash the firmware to a connected board.  If `ravedude`
   fails to detect your board, check its documentation at
   <https://crates.io/crates/ravedude>.

4. `ravedude` will open a console session after flashing where you can interact
   with the UART console of your board.

`ravedude`: https://github.com/Rahix/avr-hal/tree/main/ravedude

## License

MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)