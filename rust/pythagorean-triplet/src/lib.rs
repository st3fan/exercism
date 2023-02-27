use std::collections::HashSet;

// Running the tests with release optimization, `cargo test --release`, make
// them go 15x faster. (11.14s vs 0.74s on an M1 Air)

/// Given the sum {sum}, return all possible Pythagorean triplets,
/// which produce the said sum, or an empty HashSet if there are no
/// such triplets. Note that you are expected to return triplets in
/// [a, b, c] order, where a < b < c
pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let fsum = sum as f64;
    let mut triplets = HashSet::new();
    for a in 1..sum {
        let a = a as f64;
        for b in 1..sum {
            let b = b as f64;
            let c = ((a * a + b * b) as f64).sqrt();
            if a < b && b < c {
                if a + b + c == fsum {
                    triplets.insert([a as u32, b as u32, c as u32]);
                }
            }
        }
    }
    triplets
}

// pub fn find(sum: u32) -> HashSet<[u32; 3]> {
//     let mut triplets = HashSet::new();
//     for a in 1..sum {
//         for b in 1..sum {
//             let c = ((a * a + b * b) as f64).sqrt();
//             if (a as f64) < (b as f64) && (b as f64) < c {
//                 if a as f64 + b as f64 + c == sum as f64 {
//                     triplets.insert([a, b, c as u32]);
//                 }
//             }
//         }
//     }
//     triplets
// }

// pub fn find(sum: u32) -> HashSet<[u32; 3]> {
//     let mut triplets = HashSet::new();
//     for c in 1..=sum {
//         for b in 1..=sum {
//             for a in 1..=sum {
//                 if a < b && b < c {
//                     if a + b + c == sum && a * a + b * b == c * c {
//                         triplets.insert([a, b, c]);
//                     }
//                 }
//             }
//         }
//     }
//     triplets
// }
