use md5::compute;
use std::io::stdin;
use std::time::{Duration, Instant};

const DEFAULT_CHARSET: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*()}{|\'\\?<>.,=-_+~`;:[]\"";

fn main() {
    println!("Written by Crybik");
    println!("GitHub: https://github.com/crybik\n");

    println!("What's the hash you want to crack?");
    let mut hash: String = String::new();
    stdin().read_line(&mut hash).expect("Failed to read input");
    let hash: &str = hash.trim();

    println!("Enter the charset (leave blank for default alphanumeric characters):");
    let mut charset_input: String = String::new();
    stdin()
        .read_line(&mut charset_input)
        .expect("Failed to read input");
    let charset: &str = if charset_input.trim().is_empty() {
        DEFAULT_CHARSET
    } else {
        charset_input.trim()
    };

    println!("How long should the password be at most?");
    let mut max_length_input: String = String::new();
    stdin()
        .read_line(&mut max_length_input)
        .expect("Failed to read input");
    let max_length: u32 = max_length_input.trim().parse().unwrap_or(7);

    let start_time: Instant = Instant::now();
    let found_password: Option<String> = brute_force_md5(charset, max_length, hash);

    let elapsed_time: Duration = start_time.elapsed();
    let total_combinations: u64 = count_combinations(charset, max_length);
    let hashes_per_second: f32 = total_combinations as f32 / elapsed_time.as_secs_f32();

    match found_password {
        Some(password) => println!("Password found: {}", password),
        None => println!("Couldn't find the password. Keep trying!"),
    }

    println!(
        "Cracking completed in {:.2} seconds. Speed: {:.2} hashes/sec",
        elapsed_time.as_secs_f64(),
        hashes_per_second
    );
}

fn brute_force_md5(charset: &str, max_length: u32, hash: &str) -> Option<String> {
    let mut current: Vec<u32> = vec![0; max_length as usize];
    let charset_len: usize = charset.len();

    loop {
        let password: String = current
            .iter()
            .map(|&idx| {
                charset
                    .chars()
                    .nth(idx as usize)
                    .expect("Something went wrong, please report at https://github.com/Crybik/Rust-Md5Forcer/issues")
            })
            .collect();
        let hashed: String = format!("{:x}", compute(password.as_bytes()));

        if hashed == hash {
            return Some(password)
        }

        let mut index: u32 = max_length - 1;
        loop {
            if current[index as usize] < charset_len as u32 - 1 {
                current[index as usize] += 1;
                break;
            } else {
                current[index as usize] = 0;
                if index == 0 {
                    return None;
                }
                index -= 1;
            }
        }
    }
}

fn count_combinations(charset: &str, max_length: u32) -> u64 {
    (charset.len() as u64).pow(max_length)
}
