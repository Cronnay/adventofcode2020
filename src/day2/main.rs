use std::env;

fn main() {
    let mut file: String = String::new();
    for args in env::args() {
        file = args;
    }

    let contents = std::fs::read_to_string(file).expect("Could not load file");
    let mut valid_passwords_1 = 0;
    for line in contents.lines() {
        // Handle each PW
        if is_valid_password(line) {
            valid_passwords_1 += 1;
        }
    }

    let mut valid_passwords_2 = 0;
    for line in contents.lines() {
        // Handle each PW
        if is_valid_password_2(line) {
            valid_passwords_2 += 1;
        }
    }


    println!("Amount of valid passwords for part 1: {}", valid_passwords_1);
    println!("Amount of valid passwords for part 2: {}", valid_passwords_2);
}

// PART 1
fn is_valid_password(assumed_password: &str) -> bool {
    let splitted_password_pattern: Vec<&str> = assumed_password.split(" ").collect();
    
    let mut counter: u32 = 0;
    for character in splitted_password_pattern.get(2).unwrap().chars() {

        if let Some(provided_character) = splitted_password_pattern.get(1) {
            if character == provided_character.chars().nth(0).unwrap() {
                counter += 1;
            }
        }
    }

    if let Some(limit) = splitted_password_pattern.get(0) {
        let max_min: Vec<&str> = limit.split("-").collect();

        let max = max_min.get(1).expect("Could not parse max from the password").parse::<u32>().unwrap();
        let min = max_min.get(0).expect("Could not parse min from the password").parse::<u32>().unwrap();
        if counter >= min && counter <= max {
            return true
        };
    }

    false
}

// PART 2
fn is_valid_password_2(assumed_password: &str) -> bool {
    let splitted_password_pattern: Vec<&str> = assumed_password.split(" ").collect();

    if let Some(provided_character) = splitted_password_pattern.get(1) {
        let indices: Vec<&str> = splitted_password_pattern.get(0).unwrap().split("-").collect();
        let first_index = indices.get(0).expect("Could not get first index from password").parse::<usize>().unwrap();
        let second_index = indices.get(1).expect("Could not get first index from password").parse::<usize>().unwrap();

        let pw_as_char: Vec<char> = splitted_password_pattern.get(2).unwrap().chars().collect();

        let matching_character = provided_character.chars().next().expect("Could not fetch characters from character string");
        
        if (pw_as_char.get(first_index - 1).unwrap() == &matching_character) != (pw_as_char.get(second_index - 1).unwrap() == &matching_character) {
            return true;
        }
    } 

    false
}
 
#[test]
fn test_is_valid_password() {
    assert!(is_valid_password("3-11 w: wwmwwwwwwwww"));
    assert_ne!(is_valid_password("2-5 d: dasjgh"), true);
    assert_eq!(is_valid_password("2-5 d: dasdjgh"), true);
}

#[test]
fn test_is_valid_password_2() {
    assert!(is_valid_password_2("1-3 a: abcde"));
    assert_ne!(is_valid_password_2("1-3 b: cdefg"), true);
    assert_eq!(is_valid_password_2("2-9 c: ccccccccc"), false);
}