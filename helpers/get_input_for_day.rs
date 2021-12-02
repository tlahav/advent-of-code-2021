mod helpers {
    pub mod data {
        fn get_data_for_day(day: &str) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
            let url = format!("https://adventofcode.com/2021/day/{}/input", day);
            let session: String = fs::read_to_string("../token").unwrap();
            let client = reqwest::blocking::Client::new();
            let body: String = client.get(url).header("cookie", format!("session={}", session)).send()?.text()?;
            let v: Vec<&str> = body.lines().collect();
            let u: Vec<i32> = v.iter().map(|l| l.parse::<i32>().unwrap()).collect(); // convert to ints
            Ok(u)
        }
    }
}