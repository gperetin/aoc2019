use std::collections::HashMap;
use std::fs;

fn calculate_positions(moves: Vec<String>) -> HashMap<(i32, i32), i32> {
    // 'o' is the (0, 0) in the coordinate system
    let mut current_position = (0, 0);
    let mut wire1_positions = HashMap::new();
    let mut step_no = 0;
    for mut mov in moves {
        let direction = mov.remove(0);
        let steps = mov.parse::<i32>().unwrap();
        for i in 1..(steps+1) {
            step_no += 1;
            if direction == 'U' {
                wire1_positions.insert((current_position.0, current_position.1 + 1), step_no);
                current_position.1 = current_position.1 + 1;
            } else if direction == 'D' {
                wire1_positions.insert((current_position.0, current_position.1 - 1), step_no);
                current_position.1 = current_position.1 - 1;
            } else if direction == 'L' {
                wire1_positions.insert((current_position.0 - 1, current_position.1), step_no);
                current_position.0 = current_position.0 - 1;
            } else {
                wire1_positions.insert((current_position.0 + 1, current_position.1), step_no);
                current_position.0 = current_position.0 + 1;
            }
        }
    }
    wire1_positions
}

fn distance_from_center(position: (i32, i32)) -> i32 {
    position.0.abs() + position.1.abs()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read inputs");
    let wires: Vec<&str> = input.split_ascii_whitespace().collect();
    let wire1: Vec<String> = wires[0].split(',').map(|x| x.to_string()).collect();
    let wire2: Vec<String> = wires[1].split(',').map(|x| x.to_string()).collect();

    let wire1_positions = calculate_positions(wire1);
    let wire2_positions = calculate_positions(wire2);

    let mut fewest_steps = i32::max_value();

    for pos in wire2_positions.keys() {
        if wire1_positions.contains_key(&pos) {
            let num_steps = wire1_positions[pos] + wire2_positions[pos];
            if num_steps < fewest_steps {
                fewest_steps = num_steps;
            }
        }
    }

    println!("{:?}", fewest_steps);
    
}
