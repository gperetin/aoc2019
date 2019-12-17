use std::collections::HashSet;
use std::fs;

fn positions_for_move(starting_position: (i32, i32), move_: String) -> Vec<(i32, i32)> {
    let direction = move_.remove(0);
    let steps = move_.parse::<i32>().unwrap();
    let mut all_moves: Vec<(i32, i32)> = Vec::new();
    let mut current_position = starting_position;
    for i in 1..(steps+1) {
        if direction == 'U' {
            all_moves.push((current_position.0, current_position.1 + i));
            current_position.1 = current_position.1 + i;
        } else if direction == 'D' {
            all_moves.push((current_position.0, current_position.1 - i));
            current_position.1 = current_position.1 - i;
        } else if direction == 'L' {
            all_moves.push((current_position.0 - i, current_position.1));
            current_position.0 = current_position.0 - i;
        } else {
            all_moves.push((current_position.0 + i, current_position.1));
            current_position.0 = current_position.0 + i;
        }
    }

    all_moves
}


fn calculate_positions(moves: Vec<String>) {
    // 'o' is the (0, 0) in the coordinate system
    let mut current_position = (0, 0);

    moves.iter().map(|move_| {
        let all_moves = positions_for_move(current_position, move_);
        current_position = all_moves.last();
        all_moves.iter()
    }).flatten()
    // let wire1_positions = HashSet::new();
    // for mov in &wire1 {
    //     let direction = mov.remove(0);
    //     let steps = mov.parse::<i32>().unwrap();
    //     for i in 1..(steps+1) {
    //         if direction == 'U' {
    //             wire1_positions.insert((current_position.0, current_position.1 + i));
    //             current_position.1 = current_position.1 + i
    //         } else if direction == 'D' {
    //             wire1_positions.insert((current_position.0, current_position.1 - i));
    //             current_position.1 = current_position.1 - i
    //         } else if direction == 'L' {
    //             wire1_positions.insert((current_position.0 - i, current_position.1));
    //             current_position.0 = current_position.0 - i
    //         } else {
    //             wire1_positions.insert((current_position.0 + i, current_position.1));
    //             current_position.0 = current_position.0 + i
    //         }
    //     }
    // }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read inputs");
    let wires = input.split_ascii_whitespace().collect();
    let wire1: Vec<String> = wires[0].split(',');
    let wire2: Vec<String> = wires[1].split(',');

    let wire1_positions = calculate_positions(wire1).collect();
}
