mod helpers;
use std::collections::VecDeque;

fn simulate(fishies: &Vec<usize>, days: u16) -> u64 {
    let mut fishy_idx: VecDeque<usize> = VecDeque::from(vec![0;9]);
    for index in 0..fishies.len() {
        fishy_idx[fishies[index]] += 1_usize;
    }
    (0..days).for_each(|_day| {
        let bucket = fishy_idx.pop_front();
        fishy_idx[6] += bucket.unwrap();
        fishy_idx.extend(bucket);
    });
    fishy_idx.iter().fold(0, | acc, item | acc + item) as u64
}

fn main() {
    let data = helpers::get_data_for_day("6").unwrap();
    let fishies: Vec<usize> = data.replace('\n', "").split(',').map(|e| e.parse::<usize>().unwrap()).collect();
    println!("Part 1: {}", simulate(&fishies, 80));
    println!("Part 2: {}", simulate(&fishies, 256));
}