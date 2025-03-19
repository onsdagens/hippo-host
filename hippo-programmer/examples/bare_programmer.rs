use clap::Parser;
use clap_num::maybe_hex;
use hippo_programmer::{open_mpsse, write_bytes};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    /// VID of the target board
    #[clap(value_parser=maybe_hex::<u16>, default_value = "0x0403")]
    vid: u16,

    /// PID of the target board
    #[clap(value_parser=maybe_hex::<u16>, default_value = "0x6010")]
    pid: u16,

    /// Interface Frequency (kHz)
    #[clap(default_value = "1000")]
    freq: u32,

    /// Path to file containing bytes to be written
    #[clap(default_value = "./example_bytes")]
    path: PathBuf,
}

fn main() {
    let args: Args = Args::parse();

    let mut mpsse = open_mpsse(args.vid, args.pid, args.freq).unwrap();

    let mut bytes = vec![];

    File::open(args.path)
        .unwrap()
        .read_to_end(&mut bytes)
        .unwrap();

    write_bytes(&mut mpsse, &bytes);
}
