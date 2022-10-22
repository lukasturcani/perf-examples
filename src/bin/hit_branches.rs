fn main() {
    let num_numbers: u64 = 6_000_000;
    let mut numbers: Vec<u64> = Vec::with_capacity(num_numbers as usize);
    numbers.extend((0..num_numbers).step_by(2));
    numbers.extend((1..num_numbers).step_by(2));
    let odds_and_evens = perf_examples::split_odds_and_evens(numbers.into_iter());
    println!(
        "num odds: {} num evens: {}",
        odds_and_evens.odds.len(),
        odds_and_evens.evens.len()
    );
}
