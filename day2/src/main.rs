use rayon::prelude::*;

fn main() {
    let input = vec![
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 10, 19, 1, 6, 19, 23, 1, 10, 23, 27,
        2, 27, 13, 31, 1, 31, 6, 35, 2, 6, 35, 39, 1, 39, 5, 43, 1, 6, 43, 47, 2, 6, 47, 51, 1, 51,
        5, 55, 2, 55, 9, 59, 1, 6, 59, 63, 1, 9, 63, 67, 1, 67, 10, 71, 2, 9, 71, 75, 1, 6, 75, 79,
        1, 5, 79, 83, 2, 83, 10, 87, 1, 87, 5, 91, 1, 91, 9, 95, 1, 6, 95, 99, 2, 99, 10, 103, 1,
        103, 5, 107, 2, 107, 6, 111, 1, 111, 5, 115, 1, 9, 115, 119, 2, 119, 10, 123, 1, 6, 123,
        127, 2, 13, 127, 131, 1, 131, 6, 135, 1, 135, 10, 139, 1, 13, 139, 143, 1, 143, 13, 147, 1,
        5, 147, 151, 1, 151, 2, 155, 1, 155, 5, 0, 99, 2, 0, 14, 0,
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
