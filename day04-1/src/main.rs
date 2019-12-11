use std::i32;

const REQUIRED_LENGTH: i32 = 6;

fn get_length(number: i32) -> i32 {
    let mut len = 1;
    let mut num = number;

    while num > 10 {
        len += 1;
        num /= 10;
    }

    len
}

fn validate_candidate(candidate: i32) -> bool {
    let mut fn_vector: Vec<&dyn Fn(i32) -> bool> = vec![];

    let length_check = |x: i32| -> bool {
        get_length(x) == REQUIRED_LENGTH
    };

    let non_decreasing_check = |x: i32| -> bool {
        let mut max = 0;
        let mut ree = i32::pow(10, REQUIRED_LENGTH as u32 - 1);

        while ree > 0 {
            let current = (x / ree) % 10;

            if current < max {
                return false;
            }

            max = current;
            ree /= 10;
        }

        true
    };

    let two_adjacent_check = |x: i32| -> bool {
        let mut first = i32::pow(10, REQUIRED_LENGTH as u32 - 1);
        let mut second = i32::pow(10, REQUIRED_LENGTH as u32 - 2);

        while second > 0 {
            let a = (x / first) % 10;
            let b = (x /second) % 10;

            if a == b {
                return true;
            }

            second /= 10;
            first /= 10;
        }

        false
    };

    fn_vector.push(&length_check);
    fn_vector.push(&non_decreasing_check);
    fn_vector.push(&two_adjacent_check);

    let result = fn_vector.into_iter().map(|x| x(candidate)).fold(true, |a, b| a & b);

    result
}

fn main() {
    let input = String::from("264793-803935");
    let split: Vec<&str> = input.split("-").collect();

    let start = split[0].parse::<i32>().unwrap();
    let end = split[1].parse::<i32>().unwrap();

    let mut viable_passwords: Vec<i32> = vec![];

    for candidate in start..end {
        if validate_candidate(candidate) {
            viable_passwords.push(candidate);
        }
    }

    println!("Final :: {}", viable_passwords.len());
}
