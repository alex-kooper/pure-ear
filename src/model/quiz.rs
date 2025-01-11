use super::music::scale_degree::ScaleDegree;
use crate::model::quiz_item::QuizItem;

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;

struct Quiz {
    thread_rng: ThreadRng,
    is_completed: bool,
    solved_items: Vec<QuizItem>,
}

#[allow(unused)]
impl Quiz {
    fn generate_quiz_item(&mut self) -> QuizItem {
        QuizItem::new(
            *ScaleDegree::major_scale_degrees()
                .choose(&mut self.thread_rng)
                .expect("Major scale degrees are empty"),
        )
    }

    fn new() -> Quiz {
        Quiz {
            thread_rng: thread_rng(),
            is_completed: false,
            solved_items: Vec::new(),
        }
    }

    fn add_solved(&mut self, item: QuizItem) {
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
