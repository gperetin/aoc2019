use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read inputs");
    let mut all_codes: Vec<usize> = input.trim().split(',').map(|x| x.parse::<usize>().unwrap()).collect();

    let mut i: usize = 0;
    loop {
        let opcode = all_codes[i];
        if opcode == 99 { break; }

        let first_op = all_codes[all_codes[i+1]];
        let second_op = all_codes[all_codes[i+2]];

        let res_position = all_codes[i+3];
        if opcode == 1 {
            all_codes[res_position] = first_op + second_op;
        } else if opcode == 2{
            all_codes[res_position] = first_op * second_op;
        }
        i = i + 4
    }

    println!("{:?}", all_codes);
}
