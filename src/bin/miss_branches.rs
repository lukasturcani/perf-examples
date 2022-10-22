use rand::Fill;

fn main() {
    let mut rng = rand::thread_rng();
    let num_numbers: u64 = 6_000_000;
    let mut numbers: Vec<u64> = vec![0; num_numbers as usize];
    numbers.try_fill(&mut rng).unwrap();
    let odds_and_evens = perf_examples::split_odds_and_evens(numbers.into_iter());
    println!(
        "num odds: {} num evens: {}",
        odds_and_evens.odds.len(),
        odds_and_evens.evens.len()
    );
}
