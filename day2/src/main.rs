use std::fs;

fn main() {
    let data = get_data_for_day("2").unwrap();
    let mut distance = 0;
    let mut depth = 0;
    let mut aim = 0;

    // data.iter().enumerate().map()
    for val in data {
        let tuple = to_tuple(&val);
        if tuple.0 == "forward" {
            distance += tuple.1;
            depth += tuple.1 * aim;
        } else {
            if tuple.0 == "down" {
                aim += tuple.1;
            } else {
                aim -= tuple.1;
            }
        }
    }
    println!("Distance: {} Depth: {}, multiply: {} ", distance, depth, distance * depth);
}

fn to_tuple(s: &String) -> (&str, i32) {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return (&s[0..i], s[i+1..].parse::<i32>().unwrap());
        }
    }
    ("", 0)
}

fn get_data_for_day(day: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let url = format!("https://adventofcode.com/2021/day/{}/input", day);
    let session: String = fs::read_to_string("../token").unwrap();
    let client = reqwest::blocking::Client::new();
    let body: String = client.get(url).header("cookie", format!("session={}", session)).send()?.text()?;
    let v: Vec<String> = body.lines().map(|s| s.to_owned()).collect();
    Ok(v)
}