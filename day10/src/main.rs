
//const INPUT: Vec<u32> = [1,1,1,3,2,2,2,1,1,3].to_vec();

fn look_and_say(input: &Vec<usize>) -> Vec<usize>{
    let mut result: Vec<usize> = Vec::new();
    let mut i: usize = 0;
    
    while i < input.len() {
        let digit = input[i];
        let mut counter: usize = 0;
        while i < input.len() && input[i] == digit {
            counter += 1;
            i += 1;
        }
        result.push(counter);
        result.push(digit);
    }
    result
}
    

fn main() {
    let mut input: Vec<usize> = [1,1,1,3,2,2,2,1,1,3].to_vec();
    for _ in 0..50 {
        input = look_and_say(&input);
    }
    println!("Answer part 1:{}", input.len());
}
