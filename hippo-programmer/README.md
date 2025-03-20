# Hippo programmer

Simple tool for writing bytes to the Hippomenes programmer.

## Setting up

All that is really needed is the Rust toolchain. For more instructions refer to [install Rust](https://www.rust-lang.org/tools/install).

## Running

The simplest possible example, assuming a Digilent Arty A7 board flashed with latest [Hippo programmer example](https://github.com/onsdagens/hippo-programmer),
is just using 

```
cargo run
```

For now, this writes the bytes [0xDE, 0xAD, 0xBE, 0xEF].

Once the programmer has been integrated with Hippomenes, we will instead include some simple program (or nothing at all, TBD)

## Options

```
Usage: hippo-programmer [OPTIONS]
Options:
  -v, --vid <VID>                VID of the target board [default: 0x0403]
      --pid <PID>                PID of the target board [default: 0x6010]
  -f, --freq <FREQ>              Interface Frequency (kHz) [default: 1000]
  -p, --path <PATH>              Path to file containing bytes to be written
  -e, --elf <ELF>                Path to ELF file
  -i, --imem-size <IMEM_SIZE>    Size of instruction memory (bytes). The passed ELF text section will be padded with zeroes to that size [default: 4096]
      --imem-start <IMEM_START>  Starting address of the instruction memory. Hippomenes defaults to 0x0 [default: 0]
      --dmem-start <DMEM_START>  Starting address of the data memory. Hippomenes defaults to 0x5000_0000 [default: 0x50000000]
  -d, --dmem-size <DMEM_SIZE>    Size of data memory (bytes). The data in the passed ELF file will be padded with zeroes to that size [default: 4096]
  -h, --help                     Print help
```

## TODO

The end goal for this is to provide a cargo subcommand that builds a crate and automatically flashes it to Hippo. For now this is future work.
