use std::fs;
use clap::Parser;
use rand::{distr::Alphanumeric, Rng, SeedableRng, seq::SliceRandom, rngs::StdRng};

/// Hashes a file
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File to hash
    filename: String,
}

fn hash(input: Vec<u8>) -> String {
    let mut hash: u64 = 0;
    let prime: u64 = 67_280_421_310_721;

    let mut rng = StdRng::seed_from_u64(prime);

    let numbers = "0123456789".chars().collect::<Vec<char>>();
    let mut characters =
        "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ".chars().collect::<Vec<char>>();
    characters.shuffle(&mut rng);

    for (i, c) in input.iter().enumerate() {
        hash = hash.wrapping_mul(prime).wrapping_add(*c as u64 + i as u64);
    }

    let mut hash_items: Vec<u8> = vec![];
    for i in hash.to_string().chars() {
        hash_items.push(i as u8 - 48);
    }

    let mut hash_vector: Vec<char> = vec![];
    while hash_vector.len() < 64 {
        for (idx, i) in hash_items.iter().enumerate() {
            hash_vector.push(numbers[(idx+*i as usize) / 3_usize]);
            hash_vector.push(characters[*i as usize]);
        }
    }

    rng = StdRng::seed_from_u64(hash);
    hash_vector.shuffle(&mut rng);

    hash_vector[0..64].iter().collect()
}

fn testing() {
    let mut hashes: Vec<String> = vec![];
    for _ in 1..1_000_000 {
        let s: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();

        let current_hash = hash(s.into_bytes().to_vec());
        if hashes.contains(&current_hash) {
            println!("collision");
        }
        hashes.push(current_hash.clone());
    }
}

fn main() {
    let args = Args::parse();

    let data = fs::read(args.filename).expect("File not found");

    println!("{}", hash(data));

    // testing();
}
