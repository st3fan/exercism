/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    Vec::new()
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

// To make this exercise a bit more interesting, here is an Iterator that
// generates the Fibonacci sequence. It stops when the value overflows for
// the chosen type.

struct Fibonacci {
    prev: u8,
    last: u8,
}

impl Fibonacci {
    fn new() -> Self {
        Self { prev: 1, last: 0 }
    }
}

impl Iterator for Fibonacci {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.prev + self.last;
        self.prev = self.last;
        self.last = next;
        Some(next)
    }
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let fibonacci: Fibonacci = Fibonacci::new();
    fibonacci.take(5).collect()
}
