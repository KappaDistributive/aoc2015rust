
fn increment(input: &String) -> String {
    let mut result: String = String::new();
    let mut carry: u8 = 0;
    let mut inc: u8 = 1;
    for c in input.chars().rev() {        
        if c == 'z' && (inc == 1 || carry == 1){
            result.push('a');
            carry = 1;
        }
        else {
            result.push((c as u8 + carry + inc) as char);
            carry = 0;
        }
        inc = 0;
    }
    result.chars().rev().collect()
}

fn is_legal(input: &String) -> bool {
    let mut increasing_straight = false;
    let mut confusing_letters = false;
    let mut pairs = false;
    let bytes: Vec<u8> = input.clone().into_bytes();
    for i in 2..bytes.len() {
        if bytes[i] == bytes[i-1]+1 && bytes[i-1] == bytes[i-2]+1 {
            increasing_straight = true;
            break;
        }
    }
    for c in input.clone().chars() {
        if c == 'i' || c == 'o' || c == 'l' {
            confusing_letters = true;
        }
    }
    for i in 1..bytes.len() {
        if bytes[i] == bytes[i-1] {
            for j in i+2..bytes.len() {
                if bytes[j] == bytes[j-1]{
                    pairs = true;
                }
            }
        }
    }
    increasing_straight && !confusing_letters && pairs
}

fn solve(input: &String) -> String {
    let mut attempt:String = input.clone();
    while !is_legal(&attempt) {
        attempt = increment(&attempt);
    }
    attempt
}

fn main() {
    println!("Answer part 1: {}", solve(&String::from("cqjxjnds")));
    println!("Answer part 2: {}", solve(&increment(&solve(&String::from("cqjxjnds")))));
}
