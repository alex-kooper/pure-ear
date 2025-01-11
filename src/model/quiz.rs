use crate::model::quiz_item::QuizItem;

struct Quiz {
    is_completed: bool,
    solved_items: Vec<QuizItem>,
}

#[allow(unused)]
impl Quiz {
    fn new() -> Quiz {
        Quiz {
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
        self.solved_items.iter().filter(|item| item.is_success()).count()
    }

    fn incorrect(&self) -> usize {
        self.solved_items.iter().filter(|item| !item.is_success()).count()
    }
}
