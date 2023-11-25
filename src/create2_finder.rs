use crate::criteria::matches;
use crate::num_format::NumFormat;
use crate::reporter::Reporter;
use hex;
use rand::prelude::*;
use rand::rngs::OsRng;
use std::thread;
use tiny_keccak::{Hasher, Keccak};

#[derive(Debug)]
pub enum Create2FindError {
    InvalidDeployerLength(usize),
    InvalidInitCodeHashLength(usize),
    TooManyThreads(usize),
}

pub fn create2_find(
    thread_count: usize,
    report_interval_seconds: f64,
    format: NumFormat,
    deployer_addr: &[u8],
    init_code_hash: &[u8],
) -> Result<(), Create2FindError> {
    if deployer_addr.len() != 20 {
        return Err(Create2FindError::InvalidDeployerLength(deployer_addr.len()));
    }

    if init_code_hash.len() != 32 {
        return Err(Create2FindError::InvalidInitCodeHashLength(
            init_code_hash.len(),
        ));
    }

    if thread_count > 0xffff {
        return Err(Create2FindError::TooManyThreads(thread_count));
    }

    let mut handles = Vec::with_capacity(thread_count);

    let mut start_create2_buffer = [0u8; 85];
    start_create2_buffer[0] = 0xff;

    for (i, byte) in deployer_addr.iter().enumerate() {
        start_create2_buffer[1 + i] = *byte;
    }
    let mut rng = OsRng;
    rng.fill_bytes(&mut start_create2_buffer[1 + 20 + 32 - 12..1 + 20 + 32 - 6]);
    for (i, byte) in init_code_hash.iter().enumerate() {
        start_create2_buffer[1 + 20 + 32 + i] = *byte;
    }

    for ti in 0..thread_count {
        handles.push(Some(thread::spawn(move || {
            let mut create2_buffer = start_create2_buffer.clone();
            create2_buffer[1 + 20 + 32 - 13] = (ti % 256) as u8;
            create2_buffer[1 + 20 + 32 - 14] = (ti / 256) as u8;
            let mut reporter = Reporter::new(ti, report_interval_seconds, format);
            let mut hash = [0u8; 32];
            for b1 in 0..=255 {
                create2_buffer[1 + 20 + 0x1f - 5] = b1;
                for b2 in 0..=255 {
                    create2_buffer[1 + 20 + 0x1f - 4] = b2;
                    for b3 in 0..=255 {
                        create2_buffer[1 + 20 + 0x1f - 3] = b3;
                        for b4 in 0..=255 {
                            create2_buffer[1 + 20 + 0x1f - 2] = b4;
                            for b5 in 0..=255 {
                                create2_buffer[1 + 20 + 0x1f - 1] = b5;
                                for b6 in 0..=255 {
                                    create2_buffer[1 + 20 + 0x1f - 0] = b6;

                                    reporter.record_step();

                                    let mut keccak256 = Keccak::v256();
                                    keccak256.update(&create2_buffer);
                                    keccak256.finalize(&mut hash);
                                    let addr_bytes = &hash[12..];

                                    if matches(addr_bytes) {
                                        println!("Found address: 0x{}", hex::encode(addr_bytes));
                                        println!(
                                            "salt: 0x{}",
                                            hex::encode(&create2_buffer[21..21 + 0x20])
                                        );
                                        println!(
                                            "{:.3} seconds, {:.2}{} iterations",
                                            reporter.total_elapsed() / 1000.0,
                                            ((reporter.index * thread_count) as f64)
                                                / format.factor(),
                                            format.letter()
                                        );
                                        std::process::exit(0);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })));
    }

    for i in 0..handles.len() {
        handles[i].take().map(std::thread::JoinHandle::join);
    }

    Ok(())
}
