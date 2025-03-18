# Hippo programmer

Simple tool for writing bytes to the Hippomenes programmer.

## Setting up

All that is really needed is the Rust toolchain. For more instructions refer to (install Rust)[https://www.rust-lang.org/tools/install].

## Running

The simplest possible example, assuming a Digilent Arty A7 board flashed with latest (Hippo programmer example)[https://github.com/onsdagens/hippo-programmer],
is just using 

```
cargo run
```

For now, this writes the bytes [0xDE, 0xAD, 0xBE, 0xEF].

Once the programmer has been integrated with Hippomenes, we will instead include some simple program (or nothing at all, TBD)

## Options

```
Arguments:
  [VID]   VID of the target board [default: 0x0403]
  [PID]   PID of the target board [default: 0x6010]
  [FREQ]  Interface Frequency (kHz) [default: 1000]
  [PATH]  Path to file containing bytes to be written [default: ./example_bytes]
```

