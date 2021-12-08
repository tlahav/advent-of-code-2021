mod helpers;

fn median(numbers: &mut [i32]) -> i32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn average(numbers: &[i32]) -> i32 {
    (numbers.iter().sum::<i32>() as f32 / numbers.len() as f32) as i32
}

//n(n+1)/2
fn calculate_fuel(numbers: &[i32], location: i32) -> f64 {
    numbers.iter().map(|num| {
        let diff = f64::abs(*num as f64-location as f64);
        diff * (diff + 1.00) * 0.500
    }).fold(0.00, |acc, item| acc + item)
}
fn main() {
    let data = helpers::get_data_for_day("7").unwrap();
    // let data = helpers::get_data_from_file("./src/testdata").unwrap();
    let mut crabs: Vec<i32> = data.replace('\n', "").split(',').map(|e| e.parse::<i32>().unwrap()).collect();
    println!("avergae   {} score {}", average(&crabs), calculate_fuel(&crabs, average(&crabs)));

}