use std::fs;

#[allow(dead_code)]
pub fn get_data_from_file(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let body = fs::read_to_string(filepath).expect("Something went wrong reading the file");
    Ok(body)
}

#[allow(dead_code)]
pub fn get_data_for_day(day: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://adventofcode.com/2021/day/{}/input", day);
    let session: String = fs::read_to_string("../token").unwrap();
    let client = reqwest::blocking::Client::new();
    let body: String = client
        .get(url)
        .header("cookie", format!("session={}", session))
        .send()?
        .text()?;
    Ok(body)
}
