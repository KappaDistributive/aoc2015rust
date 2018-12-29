extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

const INPUT: &str = &"bgvyzdsv";

fn solve_part_1() -> u64 {
    let mut hasher = Md5::new();

    for i in 0..std::u64::MAX {
        hasher.input(INPUT.as_bytes());
        hasher.input(i.to_string().as_bytes());
        let mut output = [0; 16];
        hasher.result(&mut output);

        if output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32 == 0 {
            return i;
        }
        hasher.reset();
    }
    unreachable!();
}

fn solve_part_2() -> u64 {
    let mut hasher = Md5::new();

    for i in 0..std::u64::MAX {
        hasher.input(INPUT.as_bytes());
        hasher.input(i.to_string().as_bytes());
        let mut output = [0; 16];
        hasher.result(&mut output);

        if output[0] as i32 + output[1] as i32 + output[2] as i32 == 0 {
            return i;
        }
        hasher.reset();
    }
    unreachable!();
}

fn main() {

    println!("Answer part 1: {}", solve_part_1());
    println!("Answer part 2: {}", solve_part_2());

}
