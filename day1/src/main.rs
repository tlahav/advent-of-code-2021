use std::fs;

fn main() {
    let filepath = "./src/data";
    let contents = fs::read_to_string(filepath).expect("Something went wrong reading the file");
    compute_increases(contents);
}

fn compute_increases(contents: String) {
    let lines = contents.lines();
    let initial_value = (0,0,0,0);
    let result = lines.fold(initial_value, | acc, item | {
        let num_of_increases;
        let parsed_new_item: i32 = item.parse().unwrap();
        if parsed_new_item > acc.1 {
            num_of_increases = acc.0 + 1;
        } else {
            num_of_increases = acc.0;
        }
        return (num_of_increases, acc.2, acc.3, parsed_new_item);
    });
    println!("Results - {}", result.0 - 3)
}