use super::music::scale_degree::ScaleDegree;
use crate::model::quiz_item::QuizItem;

use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
pub struct Quiz {
    std_rng: StdRng,
    is_completed: bool,
    solved_items: Vec<QuizItem>,
    total_items: usize,
}

#[allow(unused)]
impl Quiz {
    pub fn generate_item(&mut self) -> QuizItem {
        QuizItem::new(
            *ScaleDegree::major_scale_degrees()
                .choose(&mut self.std_rng)
                .expect("Major scale degrees are empty"),
        )
    }

    pub fn new(total_items: usize) -> Quiz {
        Quiz {
            std_rng: StdRng::from_entropy(),
            is_completed: false,
            solved_items: Vec::new(),
            total_items,
        }
    }

    pub fn add_solved(&mut self, item: QuizItem) {
        assert!(item.is_solved());
        self.solved_items.push(item);
    }

    fn is_completed(&self) -> bool {
        self.is_completed
    }

    fn complete(&mut self) {
        self.is_completed = true;
    }

    pub fn solved(&self) -> usize {
        self.solved_items.len()
    }

    pub fn total_items(&self) -> usize {
        self.total_items
    }

    pub fn correct(&self) -> usize {
        self.solved_items
            .iter()
            .filter(|item| item.is_success())
            .count()
    }

    pub fn incorrect(&self) -> usize {
        self.solved_items
            .iter()
            .filter(|item| !item.is_success())
            .count()
    }
}
