use super::music::scale_degree::ScaleDegree;

#[allow(unused)]
pub struct QuizItem {
    question: ScaleDegree,
    answers: Vec<ScaleDegree>,
}

#[allow(unused)]
impl QuizItem {
    fn new(question: ScaleDegree) -> QuizItem {
        QuizItem {
            question,
            answers: Vec::new(),
        }
    }

    fn set_question(&mut self, question: ScaleDegree) {
        self.question = question;
        self.answers.clear();
    }

    fn answer_with(&mut self, answer: ScaleDegree) {
        self.answers.push(answer)
    }

    fn has_answer(&self, answer: ScaleDegree) -> bool {
        self.answers.iter().any(|&d| d == answer)
    }

    fn is_done(&self) -> bool {
        self.has_answer(self.question)
    }

    fn is_success(&self) -> bool {
        self.answers.first() == Some(&self.question)
    }
}
