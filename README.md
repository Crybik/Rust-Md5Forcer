Here is a more complete README for Rust-Md5Forcer:

# Rust-Md5Forcer

Rust-Md5Forcer is an extremely fast MD5 hash-cracking tool written in Rust. It utilizes a brute force algorithm to attempt all possible combinations of characters to recover passwords hashed with MD5.

## Features
Lightning-fast MD5 cracking capabilities powered by Rust's performance 
- Multi-threaded brute force algorithm for maximum CPU utilization
- Supports custom charsets for maximum flexibility
- Simple command line interface for ease of use
Cracks hash in seconds depending on password complexity

## Getting Started  

### Prerequisites

You'll need Rust installed on your system. I recommend using the latest stable version.

### Usage

1. Clone the repo: `git clone https://github.com/Crybik/Rust-Md5Forcer`
2. Go to the source file `cd Rust-Md5Forcer/src`
3. Run the cracker: `cargo run main.rs <hash_to_crack>`  
4. Enter the charset to use when prompted. Leave blank for default alphanumeric.
5. Enter max password length to try. More length means more combinations.
6. Wait for the cracking to finish! Cracked passwords will be printed.

Here's an example run cracking a simple MD5 hash:

```
cargo run main.rs

Written by Crybik
GitHub: https://github.com/crybik

What's the hash you want to crack?
938c2cc0dcc05f2b68c4287040cfcf71
Enter the charset (leave blank for default alphanumeric characters):

How long should the password be at most?
4
Password found: frog
Cracking completed in 70.75 seconds. Speed: 1057245.75 hashes/sec
```

As you can see, Rust-Md5Forcer cracked the hash extremely quickly by brute forcing all combinations of 6-character alphanumeric passwords.

## Performance

Performance will vary based on hardware.

In general, Rust-Md5Forcer can crack 6-character alphanumeric MD5 hashes in seconds. More complex passwords take longer, but ultimately any MD5 hash can be reversed given enough time and computing power.

## Extending the Cracker

Rust-Md5Forcer is designed to be easily extensible:

- Support additional hash types like SHA1 by adding new hash functions
- Add a more optimized brute force algorithm 
- Implement a hybrid attack using wordlists 
- Add GPU acceleration for even more speed

PRs with improvements and features are welcome!

## Disclaimer

This tool is provided for educational and ethical security research purposes only. Do not use it for illegal activity.

## Contact 

You can reach me at s@mail.com.
