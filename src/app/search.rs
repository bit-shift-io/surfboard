use iced::{
    Point, 
    Rectangle, 
    Task
};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use crate::utils::*;
use crate::app::*;
use trie_rs::{TrieBuilder, inc_search::Answer};

static MIN_DISTANCE: f32 = 15.0; // pixels

#[derive(Debug, Clone)]
pub enum Message {
    Update(String, Rectangle),
    Reset,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Candidate {
    pub text: String,
    pub bounds: Rectangle,
    pub points: Vec<Point>,
    pub filtered_points: Vec<Point>,
    pub weight: u32,
    pub position_weight: u32,
    pub first_or_last_weight: bool,
    pub angles_change_weight: u32,
    pub is_complete: bool,
}

impl Candidate {
    pub fn add_point(&mut self, point: Point) {
        self.points.push(point);

        if self.filtered_points.len() > 1 {
            // distance check with the back/end item
            let prev = self.filtered_points.last().unwrap();
            let distance = Point::distance(&prev, point);
            if distance < MIN_DISTANCE {
                return
            }
            self.filtered_points.push(point);
        } else {
            self.filtered_points.push(point);
        }
    }

    pub fn update_weight(&mut self) -> bool {
        let mut new_weight = 0;

        // weight distance between the last point and the center of the bounds
        if self.points.len() > 1 {
            let falloff = (functions::gaussian_pdf(
                self.points[self.points.len() - 1], 
                self.bounds.center(), 
                15.0) * 100.0
            ) as u32;
            if falloff > self.position_weight {
                self.position_weight = falloff;
            }
        }
        new_weight += self.position_weight;

        // weight number of points
        // we get about 30 points if passing over the entire bounds
        let num_points = self.points.len();
        new_weight += num_points as u32;

        // weight change of angles
        // 0-360 degrees
        if self.filtered_points.len() > 2 {
            let angle = functions::angle_between_points(
                self.filtered_points[self.filtered_points.len() - 3], 
                self.filtered_points[self.filtered_points.len() - 2],
                self.filtered_points[self.filtered_points.len() - 1],
            ) as u32;
            info!("{} angle: {}", self.text,angle);
            if angle > self.angles_change_weight {
                self.angles_change_weight = angle as u32;
            }
            //info!("angle_change: {}", angle_change);
            new_weight += self.angles_change_weight;
        }

        // weight first and last points
        if self.first_or_last_weight {
            new_weight += 50;
        }
        
        // update the weight if it has changed
        if new_weight > self.weight {
            self.weight = new_weight;
            return true
        }
        false
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
                self.components.push(Candidate { 
                    text, 
                    bounds: rectangle, 
                    weight: 0, 
                    points: Vec::new(), 
                    filtered_points: Vec::new(),
                    position_weight: 0, 
                    first_or_last_weight: false,
                    angles_change_weight: 0,
                    is_complete: false
                });
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
        // update last item
        let last = self.weighted_items.last_mut().unwrap();
        last.first_or_last_weight = true;
        last.update_weight();

        // log
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

            match self.weighted_items.last_mut() {
                Some(last_item) if last_item.text == component.text => {
                    last_item.add_point(position);
                    last_item.update_weight();
                }
                _ => {
                    // update the previous item and mark as complete
                    if self.weighted_items.len() > 1 {
                        let length = self.weighted_items.len();
                        let previous_item = self.weighted_items.get_mut( length - 2).unwrap();
                        previous_item.is_complete = true;
                        previous_item.update_weight();
                    }

                    // mouse over new item or
                    // first item in the array
                    let mut new_item = component.clone();
                    new_item.add_point(position);

                    // first item
                    if self.weighted_items.is_empty() { 
                        new_item.first_or_last_weight = true
                    }

                    new_item.update_weight();
                    self.weighted_items.push(new_item);


                }
            }

            // only search when we first enter the key, and when we exit the key
            // the weight is always changing, so we dont want to keep searching
            self.search_word();
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

    pub fn load_dictionary(&mut self) {
        //let cracklib = include_str!("/usr/share/cracklib/cracklib-small").split_whitespace();
        let now = std::time::Instant::now();
        let mut builder = TrieBuilder::new();
        for word in globals::DICTIONARY.split_whitespace() {
            builder.push(word);
        }

        let mut trie = builder.build();
        info!("Dictionary loaded in {}ms", now.elapsed().as_millis());
        // let (mut hits, mut misses) = (0,0);
        // for word in cracklib {
        //     if trie.exact_match(word) { hits += 1 } else { misses += 1 };
        // }
        // info!("Hits {}, Misses {}", hits, misses);
    }

}