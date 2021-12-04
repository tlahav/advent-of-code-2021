use std::fs;

fn main() {
    let data = get_data_for_day("3").unwrap();
    let vec: Vec<Vec<usize>> = data.lines().map(get_vec_of_numbers_from_string).collect();
    let mut possible_values = vec.clone();
    let mut index = 0;
    while index < 12 && possible_values.len() > 1 {
            let discriminator = get_digit_at_index(&possible_values, index);
            possible_values.retain(|y| y[index] == discriminator);
            //possible_values.retain(|y| y[index] != discriminator);
            index += 1;
    }
    println!("Array of binaries: {:?}", possible_values);
}

fn get_digit_at_index(vec: &Vec<Vec<usize>>, index: usize) -> usize {
    let xx: usize = vec.iter().map(|row| row[index]).sum();
    if xx as f32 / vec.len() as f32 >= 0.5 { return 1 } else {return 0}
}

fn get_vec_of_numbers_from_string(input: &str) -> Vec<usize> {
    let res = input.chars().map(|c| {
        if c == '1' { 
            return 1
        }
        return 0
    }).collect();
    res
}

fn get_data_for_day(day: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://adventofcode.com/2021/day/{}/input", day);
    let session: String = fs::read_to_string("../token").unwrap();
    let client = reqwest::blocking::Client::new();
    let body: String = client.get(url).header("cookie", format!("session={}", session)).send()?.text()?;
    // let filepath = "./src/data";
    // let body = fs::read_to_string(filepath).expect("Something went wrong reading the file");
    Ok(body)
}