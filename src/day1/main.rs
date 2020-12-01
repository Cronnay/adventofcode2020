use std::env;

fn main() {
    let mut file: String = String::new();
    for args in env::args() {
        file = args;
    }

    let contents = std::fs::read_to_string(file).expect("Could not load file");
    let mut all_inputs: Vec<i32> = Vec::new();
    for line in contents.lines() {
        if let Ok(parsed) = line.parse::<i32>() {
            all_inputs.push(parsed);
        }
    }

    if let Some(output_number) = get_entries(all_inputs, 2020) {
        println!("The value is {}", output_number);
    }
}

fn get_entries(list_of_numbers: Vec<i32>, sum: i32) -> Option<i32> {
    let mut return_value: Option<i32> = None;
    for i in list_of_numbers.iter() {
        for j in list_of_numbers.iter() {
            let filtered_numbers: Vec<&i32> = list_of_numbers.iter().filter(|x| (sum - i - j) >= **x).collect();
            // Skip all empty
            if filtered_numbers.len() == 0 {
                continue;
            }

            for filtered_number in filtered_numbers.iter() {
                if (*filtered_number + i + j) == sum {
                    return_value = Some(*filtered_number * i * j); 
                }
            }
        }
    }
    return_value
}


#[test]
fn test_get_entries() {
    assert_eq!(get_entries(vec![10, 25, 77], 112).unwrap(), 19250);
    assert_eq!(get_entries(vec![12391, 1923192, 10293912, 1000, 56132, 123], 13514).unwrap(), 1524093000);
}