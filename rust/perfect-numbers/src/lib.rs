#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

fn factors(num: u64) -> Vec<u64> {
    (1..=num / 2).into_iter().filter(|n| num % n == 0).collect()
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    let sum: u64 = factors(num).iter().sum();

    if sum == num {
        Some(Classification::Perfect)
    } else {
        if sum < num {
            Some(Classification::Deficient)
        } else {
            Some(Classification::Abundant)
        }
    }
}
