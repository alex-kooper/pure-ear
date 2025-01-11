use crate::{components::quiz_item_ctl::QuizItemCtl, model::quiz::Quiz};

use leptos::prelude::*;
use thaw::*;

#[component]
pub fn QuizCtl() -> impl IntoView {
    let mut quiz = Quiz::new();
    let quiz_item = RwSignal::new(quiz.generate_quiz_item());

    view! {
        <Flex vertical=true>
            <Flex align=FlexAlign::Center justify=FlexJustify::Center>
                <QuizItemCtl quiz_item />
            </Flex>
        </Flex>
    }
}
