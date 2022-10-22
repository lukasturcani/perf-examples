#[derive(Debug, Eq, PartialEq)]
pub struct OddsAndEvens {
    pub odds: Vec<u64>,
    pub evens: Vec<u64>,
}

impl OddsAndEvens {
    pub fn new() -> Self {
        Self {
            odds: Vec::new(),
            evens: Vec::new(),
        }
    }
}

impl Default for OddsAndEvens {
    fn default() -> Self {
        Self::new()
    }
}

pub fn split_odds_and_evens(numbers: impl Iterator<Item = u64>) -> OddsAndEvens {
    numbers.fold(OddsAndEvens::new(), |mut acc, item| {
        if item & 1 == 1 {
            acc.odds.push(item);
        } else {
            acc.evens.push(item);
        }
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_odds_and_evens() {
        assert_eq!(
            split_odds_and_evens(0..10),
            OddsAndEvens {
                odds: vec![1, 3, 5, 7, 9],
                evens: vec![0, 2, 4, 6, 8],
            },
        );
    }
}
