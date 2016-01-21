extern crate bit_set;

use bit_set::BitSet;

pub fn solve(problems: Vec<usize>) -> usize {
    let mut possible_scores = BitSet::new();

    possible_scores.insert(0);

    for p in problems {
        for s in possible_scores.clone().iter() {
            possible_scores.insert(p + s);
        }
    }

    possible_scores.iter().count()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn sample1() {
        assert_eq!(7, solve(vec![2, 3, 5]));
    }

    #[test]
    fn sample2() {
        assert_eq!(11, solve(vec![1; 10]));
    }
}
