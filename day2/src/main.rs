use std::fs;

fn main() {
    let data = get_data_for_day("2").unwrap();
    let v = data.lines().map(|s| {
        let mut x = s.split_whitespace();
        (x.next().unwrap(), x.next().unwrap().parse::<i64>().unwrap())  //(direction, magnitude)
    }).fold((0,0,0), | acc, item | { // aim, depth, distance
        match item.0 {
            "forward" => (acc.0, acc.1 + acc.0 * item.1, acc.2 + item.1),
            "down" => (acc.0 + item.1, acc.1, acc.2),
            "up" => (acc.0 - item.1, acc.1, acc.2),
            _ => (0,0,0)
        }
    });
    println!("aim: {} depth: {}, distance: {} - answer: {}", v.0,v.1,v.2, v.1 * v.2);
}


fn get_data_for_day(day: &str) -> Result<String, Box<dyn std::error::Error>> {
    // let url = format!("https://adventofcode.com/2021/day/{}/input", day);
    // let session: String = fs::read_to_string("../token").unwrap();
    // let client = reqwest::blocking::Client::new();
    // let body: String = client.get(url).header("cookie", format!("session={}", session)).send()?.text()?;
    let filepath = "./src/data";
    let body = fs::read_to_string(filepath).expect("Something went wrong reading the file");
    Ok(body)
}