use super::music::scale_degree::ScaleDegree;
use crate::model::quiz_item::QuizItem;

use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
pub struct Quiz {
    std_rng: StdRng,
    is_completed: bool,
    solved_items: Vec<QuizItem>,
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

    pub fn new() -> Quiz {
        Quiz {
            std_rng: StdRng::from_entropy(),
            is_completed: false,
            solved_items: Vec::new(),
        }
    }

    fn add_solved(&mut self, item: QuizItem) {
        assert!(item.is_solved());
        self.solved_items.push(item);
    }

    fn is_completed(&self) -> bool {
        self.is_completed
    }

    fn complete(&mut self) {
        self.is_completed = true;
    }

    fn solved(&self) -> usize {
        self.solved_items.len()
    }

    fn correct(&self) -> usize {
        self.solved_items
            .iter()
            .filter(|item| item.is_success())
            .count()
    }

    fn incorrect(&self) -> usize {
        self.solved_items
            .iter()
            .filter(|item| !item.is_success())
            .count()
    }
}
