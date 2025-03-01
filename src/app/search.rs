use iced::{
    Point, 
    Rectangle, 
    Task
};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use crate::utils::*;
use crate::app::*;

#[derive(Debug, Clone)]
pub enum Message {
    Update(String, Rectangle),
    Reset,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Candidate {
    pub text: String,
    pub bounds: Rectangle,
    pub weight: u32,
    pub points: Vec<Point>,
}

impl Candidate {
    pub fn update_weight(&mut self) -> bool {
        let mut weight_changed = false;
        // calculate the distance between the last two points
        if self.points.len() > 1 {
            let distance = self.points[self.points.len() - 1].distance(self.points[self.points.len() - 2]);
            let weight = ((1.0 - (distance / 100.0)) * 100.0) as u32;
            if weight > self.weight {
                self.weight = weight;
                weight_changed = true;
            }
        }

        // todo: calculate the number of points

        // todo: calculate the change of angles

        // todo: calculate the first and last points
        
        weight_changed
    }
}

/// Handles the state of widget/components.  
/// This is used for the glide typing.
#[derive(Clone, Debug)]
pub struct SearchHandler {
    components: Vec<Candidate>,
    weighted_items: Vec<Candidate>,
}

impl SearchHandler {
    pub fn new() -> Self {
        SearchHandler {
            components: Vec::new(),
            weighted_items: Vec::new(),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<main_app::Message> {
        match message {
            Message::Update(text, rectangle) => {
                self.components.push(Candidate { text, bounds: rectangle, weight: 0, points: Vec::new() });
                Task::none()
            }
            Message::Reset => {
                self.components.clear();
                self.weighted_items.clear();
                Task::none()
            }
        }
    }

    pub fn start(&mut self) -> Task<main_app::Message> {
        self.weighted_items.clear();
        Task::none()
    }

    pub fn end(&mut self) -> Task<main_app::Message> {
        let formatted_items: String = self.weighted_items
            .iter()
            .map(|item| format!("{} - {}\n", item.text, item.weight))
            .collect();
        info!("weighted:\n{}", formatted_items);
        Task::none()
    }

    pub fn update_move(&mut self, position: Point) -> Task<main_app::Message> {

        // loop through items, and find the one that contains the position
        let mut selected_component = None;
        for component in &mut self.components {
            if component.bounds.contains(position) {
                selected_component = Some(component);
                break; // Exit the loop once we find the first component containing the position
            }
        }

        // update the weighted item
        if let Some(component) = selected_component {
            let mut weight_changed = false;

            match self.weighted_items.last_mut() {
                Some(last_item) if last_item.text == component.text => {
                    // update the last item
                    last_item.points.push(position);
                    weight_changed = last_item.update_weight();
                }
                _ => {
                    // mouse over new item or
                    // first item in the array
                    let mut new_item = component.clone();
                    new_item.points.push(position);
                    weight_changed = new_item.update_weight();
                    self.weighted_items.push(new_item);
                }
            }

            if weight_changed {
                self.search_word();
            }
        }

        Task::none()
    }

    pub fn search_word(&mut self) {
        //info!("words: {:?}", self.beam_search());
        //info!("words: {}", self.beam_search().join(", "));
    }

    // given a vector of letters and a weighted score,
    // we want to search the static DICTIONARY
    // and return the best 3 matches
    fn beam_search(&mut self) -> Vec<String> {
        // Create a BinaryHeap to store the candidate matches
        let mut heap = BinaryHeap::new();
        //let mut seen = HashSet::new();
                
        // Iterate over the weighted items to create initial candidates
        for item in &self.weighted_items {
            heap.push(Reverse((vec![item.text.clone()], item.weight)));
        }
        
        // Perform beam search
        let mut best_matches = Vec::new();
        while let Some(Reverse((sequence, score))) = heap.pop() {
            let word = sequence.join("");
            if globals::DICTIONARY.contains(&word.as_str()) {
                best_matches.push(word);
                if best_matches.len() == 3 {
                    break;
                }
            }

            // Expand the candidate by adding more letters
            for item in &self.weighted_items {
                let mut new_sequence = sequence.clone();
                new_sequence.push(item.text.clone());
                let new_score = score + item.weight;
                heap.push(Reverse((new_sequence, new_score)));
            }
        }

        best_matches
    }

}