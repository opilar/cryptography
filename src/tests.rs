use super::md5::md5_rounds;
use super::sha1::sha1_rounds;

const MAX_ROUNDS: usize = 32;
const MAX_DATA: u16 = 0xFFFF;

pub fn run_md5_test_diff() {
    let mut result: Vec<u32> = Vec::with_capacity(MAX_ROUNDS+1);
    result.resize_with(MAX_ROUNDS+1, Default::default);
    for input in 0..=MAX_DATA {
        let input = input.to_le_bytes();
        let mut previous_round_hash = md5_rounds(&input, 1);
        for rounds in 2..=MAX_ROUNDS {
            let hash = md5_rounds(&input, rounds);
            result[rounds] += previous_round_hash.diff_with(&hash);
            previous_round_hash = hash;
        }
    }

    println!("run_md5_test_diff");
    for (round, diff) in result.iter().enumerate() {
        println!("{}: {}", round, (*diff as f64) / (MAX_DATA as f64));
    }
}

pub fn run_sha1_test_diff() {
    let mut result: Vec<u32> = Vec::with_capacity(MAX_ROUNDS+1);
    result.resize_with(MAX_ROUNDS+1, Default::default);
    for input in 0..=MAX_DATA {
        let input = input.to_le_bytes();
        let mut previous_round_hash = sha1_rounds(&input, 1);
        for rounds in 2..=MAX_ROUNDS {
            let hash = sha1_rounds(&input, rounds);
            result[rounds] += previous_round_hash.diff_with(&hash);
            previous_round_hash = hash;
        }
    }

    println!("run_sha1_test_diff");
    for (round, diff) in result.iter().enumerate() {
        println!("{}: {}", round, (*diff as f64) / (MAX_DATA as f64));
    }
}
