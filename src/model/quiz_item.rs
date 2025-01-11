use super::music::scale_degree::ScaleDegree;

#[derive(Clone)]
pub struct QuizItem {
    question: ScaleDegree,
    answers: Vec<ScaleDegree>,
}

#[allow(unused)]
impl QuizItem {
    pub fn new(question: ScaleDegree) -> QuizItem {
        QuizItem {
            question,
            answers: Vec::new(),
        }
    }

    pub fn set_question(&mut self, question: ScaleDegree) {
        self.question = question;
        self.answers.clear();
    }

    pub fn question(&self) -> ScaleDegree {
        self.question
    }

    pub fn answer_with(&mut self, answer: ScaleDegree) {
        if !self.answers.contains(&answer) {
            self.answers.push(answer)
        }
    }

    pub fn has_answer(&self, answer: ScaleDegree) -> bool {
        self.answers.iter().any(|&d| d == answer)
    }

    pub fn is_solved(&self) -> bool {
        self.has_answer(self.question)
    }

    pub fn is_success(&self) -> bool {
        self.answers.first() == Some(&self.question)
    }
}
