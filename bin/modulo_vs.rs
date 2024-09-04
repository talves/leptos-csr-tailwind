#!/usr/bin/env -S cargo +nightly -Zscript

//! ```cargo
//! [package]
//! name = "modulo_vs"
//! version = "0.1.0"
//!
//! [dependencies]
//! ```

use std::time::Instant;

fn main() {
    let iterations = 100_000_000;
    let mut odd_count = 0;

    // Using modulus
    let start_modulus = Instant::now();
    for i in 0..iterations {
        if i % 2 != 0 {
            odd_count += 1;
        }
    }
    let duration_modulus = start_modulus.elapsed();
    println!(
        "Modulus: {:?} to count {} odd numbers",
        duration_modulus, odd_count
    );

    // Reset odd_count
    odd_count = 0;

    // Using bitwise AND
    let start_bitwise = Instant::now();
    for i in 0..iterations {
        odd_count += i & 1;
    }
    let duration_bitwise = start_bitwise.elapsed();
    println!(
        "Bitwise AND: {:?} to count {} odd numbers",
        duration_bitwise, odd_count
    );

    let percentage_faster = (duration_modulus.as_secs_f32() - duration_bitwise.as_secs_f32())
        / (duration_bitwise.as_secs_f32())
        * 100.0;
    println!("Result: {:?} % faster/slower", percentage_faster);
}
