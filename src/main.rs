mod create2_finder;
mod criteria;
mod eoa_finder;
mod num_format;
mod reporter;

use create2_finder::create2_find;
use hex;
use num_format::NumFormat;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Args {
    #[structopt(help = "Number of threads")]
    thread_count: usize,

    #[structopt(help = "Iteration count with format (G/M/K/U)")]
    iter_format: String,

    #[structopt(help = "Iteration interval in seconds at which stats will be reported")]
    report_interval: f64,
}

#[derive(Debug)]
enum FindoorError {
    InvalidFormat(String),
}

fn main() -> Result<(), FindoorError> {
    let args = Args::from_args();

    let format = match args.iter_format.as_str() {
        "G" => Ok(NumFormat::Giga),
        "M" => Ok(NumFormat::Mega),
        "K" => Ok(NumFormat::Kilo),
        "U" => Ok(NumFormat::Uni),
        _ => Err(FindoorError::InvalidFormat(args.iter_format)),
    }?;

    // Change here

    let deployer_address = "4e59b44847b379578588920cA78FbF26c0B4956C";
    let init_code_hash = "adeec2e1b9fd10ba10a631c44352e132442d4de05bc170f69a7a0ecdfce34a53";

    create2_find(
        args.thread_count,
        args.report_interval,
        format,
        hex::decode(deployer_address)
            .expect("invalid hex?")
            .as_slice(),
        hex::decode(init_code_hash)
            .expect("invalid hex?")
            .as_slice(),
    )
    .expect("no create2 error");

    Ok(())
}
