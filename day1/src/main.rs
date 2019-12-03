pub const INPUT: [i32; 100] = [
    88623, 101095, 149899, 89383, 54755, 73496, 115697, 99839, 65903, 140201, 95734, 144728,
    113534, 82199, 98256, 107126, 54686, 61243, 54763, 119048, 58863, 134097, 84959, 130490,
    145813, 115794, 130398, 69864, 133973, 58382, 75635, 77153, 132645, 91661, 126536, 118977,
    137568, 100341, 142080, 63342, 84557, 51961, 61956, 87922, 92488, 107572, 51373, 70148, 80672,
    134880, 105721, 100138, 80394, 145117, 50567, 122606, 112408, 110737, 111521, 144309, 65761,
    113147, 58920, 96623, 65479, 66576, 94244, 64493, 142334, 65969, 99461, 143656, 134661, 90115,
    122994, 66994, 135658, 134336, 102958, 111410, 107930, 54711, 101397, 111350, 86453, 134383,
    134276, 130342, 80522, 64875, 68182, 83400, 121302, 105996, 123580, 130373, 123987, 107932,
    78930, 132068,
];

fn fuel_for_mass(size: i32) -> i32 {
    ((size as f32) / 3.0).floor() as i32 - 2
}

fn total_fuel(size: i32) -> i32 {
    match fuel_for_mass(size) {
        x if x > 0 => x + total_fuel(x),
        _ => 0,
    }
}

fn main() {
    let part1: i32 = INPUT.iter().map(|mass| fuel_for_mass(*mass)).sum();
    println!("Part 1 Answer = {}", part1);
    let part2: i32 = INPUT.iter().map(|mass| total_fuel(*mass)).sum();
    println!("Part 2 Answer = {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuel_for_mass_is_good() {
        assert_eq!(fuel_for_mass(12), 2);
        assert_eq!(fuel_for_mass(14), 2);
        assert_eq!(fuel_for_mass(1969), 654);
        assert_eq!(fuel_for_mass(100756), 33583);
    }

    #[test]
    fn total_fuel_is_good() {
        assert_eq!(total_fuel(12), 2);
        assert_eq!(total_fuel(14), 2);
        assert_eq!(total_fuel(1969), 966);
        assert_eq!(total_fuel(100756), 50346);
    }
}
