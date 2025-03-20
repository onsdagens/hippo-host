use clap::Parser;
use clap_num::maybe_hex;
use ftdaye::ftdaye::jtag::FtdiMpsse;
use hippo_programmer::{open_mpsse, write_bytes};
use riscv_elf_parse::Memory;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    /// VID of the target board
    #[arg(short, long)]
    #[clap(value_parser=maybe_hex::<u16>, default_value = "0x0403")]
    vid: u16,

    /// PID of the target board
    #[arg(long)]
    #[clap(value_parser=maybe_hex::<u16>, default_value = "0x6010")]
    pid: u16,

    /// Interface Frequency (kHz)
    #[arg(short, long)]
    #[clap(default_value = "1000")]
    freq: u32,

    /// Path to file containing bytes to be written
    #[arg(short, long)]
    path: Option<PathBuf>,

    /// Path to ELF file
    #[arg(short, long)]
    elf: Option<PathBuf>,

    /// Size of instruction memory (bytes). The passed ELF
    /// text section will be padded with zeroes
    /// to that size.
    #[arg(short, long)]
    #[clap(default_value = "4096")]
    imem_size: usize,

    /// Starting address of the instruction memory.
    /// Hippomenes defaults to 0x0.
    #[arg(long)]
    #[clap(value_parser=maybe_hex::<usize>, default_value = "0")]
    imem_start: usize,

    /// Starting address of the data memory.
    /// Hippomenes defaults to 0x5000_0000
    #[arg(long)]
    #[clap(value_parser=maybe_hex::<usize>, default_value = "0x50000000")]
    dmem_start: usize,

    /// Size of data memory (bytes). The data in the passed
    /// ELF file will be padded with zeroes to
    /// that size.
    #[arg(short, long)]
    #[clap(default_value = "4096")]
    dmem_size: usize,
}

fn main() {
    let args: Args = Args::parse();
    let mut mpsse = open_mpsse(args.vid, args.pid, args.freq).unwrap();

    if let Some(elf) = args.elf {
        write_elf(
            &mut mpsse,
            &elf,
            args.imem_size,
            args.imem_start,
            args.dmem_size,
            args.dmem_start,
        );
    } else if let Some(path) = args.path {
        write_file(&mut mpsse, &path);
    } else {
        write_raw_bytes(&mut mpsse, &[0xDE, 0xAD, 0xBE, 0xEF]);
    }
}
fn write_file(mpsse: &mut FtdiMpsse, path: &PathBuf) {
    let mut bytes = vec![];

    File::open(path).unwrap().read_to_end(&mut bytes).unwrap();

    write_bytes(mpsse, &bytes);
}
fn write_raw_bytes(mpsse: &mut FtdiMpsse, bytes: &[u8]) {
    write_bytes(mpsse, bytes);
}

fn write_elf(
    mpsse: &mut FtdiMpsse,
    path: &PathBuf,
    imem_size: usize,
    imem_start: usize,
    dmem_size: usize,
    dmem_start: usize,
) {
    let mut elf_bytes = vec![];

    File::open(path)
        .unwrap()
        .read_to_end(&mut elf_bytes)
        .unwrap();

    let mem = Memory::new_from_file(&elf_bytes, false);

    let mut imem_vec: Vec<u8> = vec![0; imem_size];

    let mut dmem_vec: Vec<u8> = vec![0; dmem_size];

    for byte in mem.bytes {
        if byte.0 >= imem_start && byte.0 < imem_start + imem_size {
            imem_vec[byte.0 - imem_start] = byte.1;
        } else if byte.0 >= dmem_start && byte.0 < dmem_start + dmem_size {
            dmem_vec[byte.0 - dmem_start] = byte.1;
        } else {
            panic!("Address 0x{:08x} is not within the range of instruction memory (0x{:08x}-0x{:08x}), or data memory (0x{:08x}-0x{:08x})",
                byte.0 ,
                imem_start, 
                imem_start+imem_size,
                dmem_start,
                dmem_start+dmem_size
            );
        }
    }
    imem_vec.append(&mut dmem_vec);
    write_bytes(mpsse, &imem_vec);
}
