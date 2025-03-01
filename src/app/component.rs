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
pub struct ComponentData {
    pub text: String,
    pub bounds: Rectangle,
    pub weight: u32,
}

/// Handles the state of widget/components.  
/// This is used for the glide typing.
#[derive(Clone, Debug)]
pub struct ComponentHandler {
    components: Vec<ComponentData>,
    weighted_items: Vec<ComponentData>,
}

impl ComponentHandler {
    pub fn new() -> Self {
        ComponentHandler {
            components: Vec::new(),
            weighted_items: Vec::new(),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<main_app::Message> {
        match message {
            Message::Update(text, rectangle) => {
                self.components.push(ComponentData { text, bounds: rectangle, weight: 0 });
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
        info!("ComponentHandler started");
        Task::none()
    }

    pub fn end(&mut self) -> Task<main_app::Message> {
        let formatted_items: String = self.weighted_items
            .iter()
            .map(|item| format!("{} - {}\n", item.text, item.weight))
            .collect();
        info!("weighted letters:\n{}", formatted_items);
        Task::none()
    }

    pub fn update_move(&mut self, position: Point) -> Task<main_app::Message> {

        // loop through components, and find the one that is closest to the position
        let mut closest_component = None;
        let mut closest_distance = f32::MAX;
        for component in &self.components {
            let distance = component.bounds.center().distance(position);
            if distance < closest_distance {
                closest_distance = distance;
                closest_component = Some(component);
            }
        }

        if let Some(component) = closest_component {
            if let Some(last_item) = self.weighted_items.last() {
                if last_item.text != component.text {
                    self.weighted_items.push(component.clone());
                    //info!("{:?}", component.text);
                }
            } else {
                self.weighted_items.push(component.clone());
                //info!("first {:?}", component.text);
            }

            let weight_changed = self.update_weight(position);
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

    pub fn update_weight(&mut self, position: Point) -> bool {
        if let Some(last_item) = self.weighted_items.last() {
            let distance = last_item.bounds.center().distance(position);
            let weight = ((1.0 - (distance / 100.0)) * 100.0) as u32;
            if weight > last_item.weight {
                self.weighted_items.last_mut().unwrap().weight = weight;
                //info!("weight: {}", weight);
                return true
            }
        }
        false
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