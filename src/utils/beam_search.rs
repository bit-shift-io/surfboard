use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::cmp::{Ord, PartialOrd, Ordering};

const EPSILON: f32 = 1e-6;  // Define an acceptable tolerance for floating-point comparisons

#[derive(Clone)]
struct Candidate {
    sequence: Vec<String>,  // Sequence of keys
    score: f32,             // Probability score (higher is better)
}

impl Candidate {
    fn new(sequence: Vec<String>, score: f32) -> Self {
        Candidate { sequence, score }
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// Manually implement Eq with a tolerance for f32
impl Eq for Candidate {}

impl PartialEq for Candidate {
    fn eq(&self, other: &Self) -> bool {
        (self.score - other.score).abs() < EPSILON
    }
}

fn beam_search(beam_width: usize, predictions: Vec<Vec<String>>, scores: Vec<f32>) -> Vec<String> {
    let mut heap = BinaryHeap::new();

    for (i, prediction) in predictions.iter().enumerate() {
        let score = scores[i];
        heap.push(Reverse(Candidate::new(prediction.clone(), score)));
    }

    let mut best_sequence = Vec::new();
    for _ in 0..beam_width {
        if let Some(Reverse(candidate)) = heap.pop() {
            best_sequence = candidate.sequence;
        }
    }

    best_sequence
}

pub fn test() {
    let predictions = vec![
        vec!["a".to_string(), "b".to_string()],
        vec!["b".to_string(), "c".to_string()],
        vec!["c".to_string(), "d".to_string()],
    ];
    let scores = vec![0.9, 0.85, 0.95];

    let result = beam_search(3, predictions, scores);
    println!("{:?}", result);
}
