use crate::criteria::matches;
use crate::num_format::NumFormat;
use crate::reporter::Reporter;
use hex;
use rand::prelude::*;
use secp256k1::{rand::rngs::OsRng, scalar::Scalar, PublicKey, Secp256k1, SecretKey};
use std::thread;
use tiny_keccak::{Hasher, Keccak};

fn hash_pubkey(hash: &mut [u8; 32], pub_key: &PublicKey) {
    let mut hasher = Keccak::v256();
    hasher.update(&pub_key.serialize_uncompressed()[1..]);
    hasher.finalize(hash);
}

fn index_to_be_bytes(x: usize) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    let lower = x.to_be_bytes();
    let l = lower.len();
    for i in 0..l {
        bytes[32 - l + i] = lower[i];
    }
    bytes
}

pub fn eoa_find(thread_count: usize, report_interval_seconds: f64, format: NumFormat) {
    let mut handles = Vec::with_capacity(thread_count);

    for ti in 0..thread_count {
        handles.push(Some(thread::spawn(move || {
            let alg = Secp256k1::new();
            let mut reporter = Reporter::new(ti, report_interval_seconds, format);
            let mut hash = [0u8; 32];
            let (mut private_key, mut public_key) = alg.generate_keypair(&mut OsRng);

            loop {
                reporter.record_step();
                public_key = public_key.add_exp_tweak(&alg, &Scalar::ONE).unwrap();

                hash_pubkey(&mut hash, &public_key);
                let addr_bytes = &hash[12..];

                if matches(addr_bytes) {
                    private_key = private_key
                        .add_tweak(
                            &Scalar::from_be_bytes(index_to_be_bytes(reporter.index)).unwrap(),
                        )
                        .unwrap();
                    println!("Found address: 0x{}", hex::encode(addr_bytes));
                    println!("private key: 0x{}", hex::encode(private_key.secret_bytes()));
                    println!(
                        "{:.3} seconds, {:.2}{} iterations",
                        reporter.total_elapsed() / 1000.0,
                        ((reporter.index * thread_count) as f64) / format.factor(),
                        format.letter()
                    );
                    std::process::exit(0);
                }
            }
        })));
    }

    for i in 0..handles.len() {
        handles[i].take().map(std::thread::JoinHandle::join);
    }
}
