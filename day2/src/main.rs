use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read inputs");
    let mut original_all_codes: Vec<usize> = input.trim().split(',').map(|x| x.parse::<usize>().unwrap()).collect();

    for noun in 0..100 {
        for verb in 0..100 {
            let mut all_codes = original_all_codes.clone();
            all_codes[1] = noun;
            all_codes[2] = verb;

            let mut ip: usize = 0;

            loop {
                let opcode = all_codes[ip];
                if opcode == 99 { break; }

                let first_op = all_codes[all_codes[ip+1]];
                let second_op = all_codes[all_codes[ip+2]];

                let res_position = all_codes[ip+3];
                if opcode == 1 {
                    all_codes[res_position] = first_op + second_op;
                } else if opcode == 2{
                    all_codes[res_position] = first_op * second_op;
                }
                ip = ip + 4
            }

            if all_codes[0] == 19690720 {
                println!("{} {}", noun, verb);
            }

        }
    }
}
