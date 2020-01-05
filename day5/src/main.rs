use rayon::prelude::*;

fn main() {
    let input = vec![
        3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1001, 191, 50, 224, 101, -64, 224, 224, 4,
        224, 1002, 223, 8, 223, 101, 5, 224, 224, 1, 224, 223, 223, 2, 150, 218, 224, 1001, 224,
        -1537, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 2, 224, 1, 223, 224, 223, 1002, 154, 5,
        224, 101, -35, 224, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 5, 224, 1, 224, 223, 223,
        1102, 76, 17, 225, 1102, 21, 44, 224, 1001, 224, -924, 224, 4, 224, 102, 8, 223, 223, 1001,
        224, 4, 224, 1, 224, 223, 223, 101, 37, 161, 224, 101, -70, 224, 224, 4, 224, 1002, 223, 8,
        223, 101, 6, 224, 224, 1, 223, 224, 223, 102, 46, 157, 224, 1001, 224, -1978, 224, 4, 224,
        102, 8, 223, 223, 1001, 224, 5, 224, 1, 224, 223, 223, 1102, 5, 29, 225, 1101, 10, 7, 225,
        1101, 43, 38, 225, 1102, 33, 46, 225, 1, 80, 188, 224, 1001, 224, -73, 224, 4, 224, 102, 8,
        223, 223, 101, 4, 224, 224, 1, 224, 223, 223, 1101, 52, 56, 225, 1101, 14, 22, 225, 1101,
        66, 49, 224, 1001, 224, -115, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 7, 224, 1, 224,
        223, 223, 1101, 25, 53, 225, 4, 223, 99, 0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        1105, 0, 99999, 1105, 227, 247, 1105, 1, 99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1,
        99999, 1106, 227, 99999, 1106, 0, 265, 1105, 1, 99999, 1006, 0, 99999, 1006, 227, 274,
        1105, 1, 99999, 1105, 1, 280, 1105, 1, 99999, 1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0,
        1105, 1, 99999, 1106, 0, 300, 1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0,
        1105, 1, 99999, 108, 226, 226, 224, 1002, 223, 2, 223, 1005, 224, 329, 101, 1, 223, 223,
        108, 677, 677, 224, 1002, 223, 2, 223, 1006, 224, 344, 1001, 223, 1, 223, 8, 677, 677, 224,
        102, 2, 223, 223, 1006, 224, 359, 101, 1, 223, 223, 7, 226, 677, 224, 102, 2, 223, 223,
        1005, 224, 374, 101, 1, 223, 223, 107, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 389,
        101, 1, 223, 223, 7, 677, 226, 224, 1002, 223, 2, 223, 1006, 224, 404, 1001, 223, 1, 223,
        1107, 677, 226, 224, 1002, 223, 2, 223, 1006, 224, 419, 1001, 223, 1, 223, 1007, 226, 226,
        224, 102, 2, 223, 223, 1005, 224, 434, 101, 1, 223, 223, 1008, 226, 677, 224, 102, 2, 223,
        223, 1005, 224, 449, 1001, 223, 1, 223, 1007, 677, 677, 224, 1002, 223, 2, 223, 1006, 224,
        464, 1001, 223, 1, 223, 1008, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 479, 101, 1, 223,
        223, 1007, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 494, 1001, 223, 1, 223, 108, 226,
        677, 224, 1002, 223, 2, 223, 1006, 224, 509, 101, 1, 223, 223, 8, 226, 677, 224, 102, 2,
        223, 223, 1005, 224, 524, 1001, 223, 1, 223, 107, 677, 677, 224, 1002, 223, 2, 223, 1005,
        224, 539, 101, 1, 223, 223, 107, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 554, 101, 1,
        223, 223, 1107, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 569, 1001, 223, 1, 223, 1108,
        677, 226, 224, 102, 2, 223, 223, 1005, 224, 584, 1001, 223, 1, 223, 1008, 677, 677, 224,
        102, 2, 223, 223, 1005, 224, 599, 1001, 223, 1, 223, 1107, 677, 677, 224, 102, 2, 223, 223,
        1006, 224, 614, 101, 1, 223, 223, 7, 226, 226, 224, 102, 2, 223, 223, 1005, 224, 629, 1001,
        223, 1, 223, 1108, 677, 677, 224, 102, 2, 223, 223, 1006, 224, 644, 1001, 223, 1, 223, 8,
        677, 226, 224, 1002, 223, 2, 223, 1005, 224, 659, 101, 1, 223, 223, 1108, 226, 677, 224,
        102, 2, 223, 223, 1005, 224, 674, 101, 1, 223, 223, 4, 223, 99, 226,
    ];

    let part_1 = run_with_standard_repair(input.clone());
    println!("Part 1 = {:?}", part_1[0]);

    let part_2 = guess_input_fixes(input.clone());
    println!("Part 2 = {:?}", part_2);
}

pub fn run_with_standard_repair(mut input: Vec<i32>) -> Vec<i32> {
    if let Some(i) = input.get_mut(1) {
        *i = 12
    };
    if let Some(i) = input.get_mut(2) {
        *i = 2
    };
    run_int_code(input)
}

pub fn guess_input_fixes(input: Vec<i32>) -> i32 {
    let expected_output = 19690720;

    let mut coordinates: Vec<(i32, i32)> = vec![];
    let max = match input.len() < 99 {
        true => input.len(),
        false => 99,
    };
    for i in 0..max + 1 {
        for j in 0..max + 1 {
            coordinates.push((i as i32, j as i32));
        }
    }

    let result = coordinates.par_iter().find_any(|(a, b)| {
        let mut program = input.clone();
        if let Some(i) = program.get_mut(1) {
            *i = *a
        };
        if let Some(i) = program.get_mut(2) {
            *i = *b
        };

        let result = run_int_code(program);
        result[0] == expected_output
    });
    match result {
        Some((x, y)) => x * 100 + y,
        None => 0,
    }
}

pub struct State {
    program: Vec<i32>,
    pos: usize,
}

pub fn run_int_code(program: Vec<i32>) -> Vec<i32> {
    let mut state: State = State { program, pos: 0 };
    loop {
        match run_int_code_at_pos(state.program, state.pos) {
            (new_program, None) => return new_program,
            (new_program, Some(new_pos)) => {
                state = State {
                    program: new_program,
                    pos: new_pos,
                }
            }
        };
    }
}

pub fn run_int_code_at_pos(program: Vec<i32>, pos: usize) -> (Vec<i32>, Option<usize>) {
    match program.get(pos as usize).unwrap() {
        1 => {
            let mut new_program = program.clone();
            let b1 = new_program[pos + 1] as usize;
            let b2 = new_program[pos + 2] as usize;
            let result = new_program[b1] + new_program[b2];
            let store_index = new_program[pos + 3] as usize;
            let where_to_store = new_program.get_mut(store_index).unwrap();
            *where_to_store = result;
            (new_program, Some(pos + 4))
        }
        2 => {
            let mut new_program = program.clone();
            let b1 = new_program[pos + 1] as usize;
            let b2 = new_program[pos + 2] as usize;
            let result = new_program[b1] * new_program[b2];
            let store_index = new_program[pos + 3] as usize;
            let where_to_store = new_program.get_mut(store_index).unwrap();
            *where_to_store = result;
            (new_program, Some(pos + 4))
        }
        99 => (program, None),
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_program_correctly() {
        assert_eq!(run_int_code(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
        assert_eq!(run_int_code(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
        assert_eq!(
            run_int_code(vec![2, 4, 4, 5, 99, 0]),
            vec![2, 4, 4, 5, 99, 9801]
        );
        assert_eq!(
            run_int_code(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
}
