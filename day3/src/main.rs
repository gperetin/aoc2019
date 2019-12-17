use std::collections::HashSet;
use std::fs;

// fn positions_for_move(starting_position: (i32, i32), move_: String) -> Vec<(i32, i32)> {
//     let direction = move_.remove(0);
//     let steps = move_.parse::<i32>().unwrap();
//     let mut all_moves: Vec<(i32, i32)> = Vec::new();
//     let mut current_position = starting_position;
//     for i in 1..(steps+1) {
//         if direction == 'U' {
//             all_moves.push((current_position.0, current_position.1 + i));
//             current_position.1 = current_position.1 + i;
//         } else if direction == 'D' {
//             all_moves.push((current_position.0, current_position.1 - i));
//             current_position.1 = current_position.1 - i;
//         } else if direction == 'L' {
//             all_moves.push((current_position.0 - i, current_position.1));
//             current_position.0 = current_position.0 - i;
//         } else {
//             all_moves.push((current_position.0 + i, current_position.1));
//             current_position.0 = current_position.0 + i;
//         }
//     }

//     all_moves
// }


fn calculate_positions(moves: Vec<String>) -> HashSet<(i32, i32)> {
    // 'o' is the (0, 0) in the coordinate system
    let mut current_position = (0, 0);
    let mut wire1_positions = HashSet::new();
    for mut mov in moves {
        let direction = mov.remove(0);
        let steps = mov.parse::<i32>().unwrap();
        for i in 1..(steps+1) {
            if direction == 'U' {
                wire1_positions.insert((current_position.0, current_position.1 + 1));
                current_position.1 = current_position.1 + 1;
            } else if direction == 'D' {
                wire1_positions.insert((current_position.0, current_position.1 - 1));
                current_position.1 = current_position.1 - 1;
            } else if direction == 'L' {
                wire1_positions.insert((current_position.0 - 1, current_position.1));
                current_position.0 = current_position.0 - 1;
            } else {
                wire1_positions.insert((current_position.0 + 1, current_position.1));
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

    let mut min_distance = i32::max_value();
    let mut closest_point = (0, 0);

    for pos in wire2_positions {
        if wire1_positions.contains(&pos) {
            if distance_from_center(pos) < min_distance {
                min_distance = distance_from_center(pos);
                closest_point = pos;
            }
        }
    }

    println!("{:?}", distance_from_center(closest_point));
    
}
