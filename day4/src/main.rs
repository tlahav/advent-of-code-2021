use std::fs;

const CROSSED: i32 = -999;
fn main() {
    let (allboards, draw_numbers) = get_data();

    // Part 1
    let try_to_win = true;
    let (winning_board, winning_number) = play_some_bingo(&allboards, &draw_numbers, try_to_win);
    print_results(try_to_win, calculate_remainder(&winning_board), winning_number);
   
    // Part 2
    let try_to_win = false;
    let (winning_board, winning_number) = play_some_bingo(&allboards, &draw_numbers, try_to_win);
    print_results(try_to_win, calculate_remainder(&winning_board), winning_number);
}

fn play_some_bingo(allboards: &Vec<Vec<i32>>, draw_numbers: &Vec<i32>, try_to_win: bool) -> (Vec<i32>, i32) {
    let mut winner_found = false;
    let mut copy_boards = allboards.clone();
    let mut copy_of_draw_numbers = draw_numbers.clone();

    let mut winning_board: Vec<i32> = vec![0];
    let mut winning_number: i32 = 0;
    while copy_of_draw_numbers.len() > 0 && (!winner_found || !try_to_win) {
        let drawn_number: i32 = copy_of_draw_numbers.pop().unwrap();
        copy_boards = copy_boards.iter()
            .map(|e| {
                let new = cross_off_number_from(&e, drawn_number);
                if check_for_winning_condition(&new) {
                    winning_number = drawn_number;
                    winner_found = true;
                    winning_board = new.to_vec();
                }
                new
            }).collect();
        if !try_to_win {
            copy_boards.retain(|x|!check_for_winning_condition(&x));
        }
    }
    (winning_board, winning_number)
}

fn check_for_winning_condition(board: &Vec<i32>) -> bool {
    let mut winner = false;
    (0..5).for_each(|e| {
        let mut horz_slice: Vec<i32> = board[e * 5..e * 5 + 5].to_vec();
        horz_slice.retain(|&x| x != CROSSED);

        let mut vert_slice: Vec<i32> = vec![
            board[e + 5 * 0],
            board[e + 5 * 1],
            board[e + 5 * 2],
            board[e + 5 * 3],
            board[e + 5 * 4],
        ];
        vert_slice.retain(|&x| x != CROSSED);
        if vert_slice.is_empty() || horz_slice.is_empty() {
            winner = true
        }
    });
    winner
}

fn calculate_remainder(board: &Vec<i32>) -> i32 {
    let mut copy = board.clone();
    copy.retain(|&x| x != CROSSED);
    copy.iter().sum()
}

fn cross_off_number_from(board: &Vec<i32>, number: i32) -> Vec<i32> {
    let index_found = board.iter().position(|&r| r == number).unwrap_or(999);
    let mut new_board = board.clone();

    if index_found != 999 {
        new_board[index_found] = CROSSED;
    }
    new_board
}

fn get_data() -> (Vec<Vec<i32>>, Vec<i32>) {
    let _draw_numbers: String = get_data_from_file("./src/testnumbers").unwrap();
    let draw_numbers: Vec<i32> = _draw_numbers.split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .rev()
        .collect();
    let boards: String = get_data_from_file("./src/testboards").unwrap();
    let allboards: Vec<Vec<i32>> = boards
        .split("\n\n")
        .map(|e| {
            let line = e.replace("\n", " ").replace("  ", " ");
            line.split_whitespace()
                .map(|e| e.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    (allboards, draw_numbers)
}

fn get_data_from_file(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    // let url = format!("https://adventofcode.com/2021/day/{}/input", day);
    // let session: String = fs::read_to_string("../token").unwrap();
    // let client = reqwest::blocking::Client::new();
    // let body: String = client.get(url).header("cookie", format!("session={}", session)).send()?.text()?;
    let body = fs::read_to_string(filepath).expect("Something went wrong reading the file");
    Ok(body)
}

fn print_results(try_to_win: bool, sum_of_unmarked: i32, winning_number: i32) {
    println!("Results when I'm{}trying to win is: {}", 
        if try_to_win { " " } else { " NOT "},
        sum_of_unmarked * winning_number,
    );
}