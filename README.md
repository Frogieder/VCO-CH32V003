# VCO on CH32V003
An implementation of a voltage-controlled oscillator running on all versions of CH32V003 written in Rust.
The main purpose of the VCO is to give an acoustic representation of changing voltage, thus the voltage-to-frequency mapping has been optimized for that.
If you require a different function, you can modify the line starting with `let mut f = ` (line 86 as of writing) in `src/main.rs`.

## Pinout
Pin numbers are valid for the SOP-8 CH32V003J4M6. For other models, use the corresponding pins based on the pin names below.
* Input: PC4 / Pin 7
* Output: PA1 / Pin 1
* Complementary output: PA2 / Pin 3
* Vdd: Pin 4
* Vss: Pin 2
* SWIO: PD1 / Pin 8

## Compile and run
### Prerequesites
This projects nequires an instalation of Rust.

You also need to install `wlink` using one of the following methods:

Using cargo (universal method):
```bash
cargo install --git https://github.com/ch32-rs/wlink
```
Arch users can install `wlink` from AUR
```bash
yay -S wlink
```
### Compile and upload

To run this, connect a WCH-linkE to your computer and the CH32V003. Then launch the following command in the project root:
```bash
cargo run --release
```
