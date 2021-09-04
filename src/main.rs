use passgen::{az, nums, symbs, AZ};
use rand::Rng;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let PASSWORD_LEN: usize = if args.len() < 2 {
        16
    } else {
        args[1].parse().unwrap()
    };
    let CHARSET: &[u8] = &[AZ, az, nums, symbs].concat();
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    println!("{:?}", password);
    println!("{:?}", password);
}
