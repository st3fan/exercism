#[derive(Debug)]
pub struct HighScores {
    _scores: Vec<u32>
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self { _scores: Vec::from(scores) }
    }

    pub fn scores(&self) -> &[u32] {
        self._scores.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        self._scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self._scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted = self._scores.clone();
        sorted.sort_by(|a, b| b.cmp(a));
        sorted.truncate(3);
        sorted
    }
}
