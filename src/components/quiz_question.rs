use crate::components::quiz_button::{QuizAnswer, QuizButton};
use leptos::*;
use thaw::*;

#[component]
pub fn QuizQuestion() -> impl IntoView {
    let answer1 = RwSignal::default();
    let on_click1 = move || answer1.set(QuizAnswer::Right);

    let answer2 = RwSignal::default();
    let on_click2 = move || answer2.set(QuizAnswer::Wrong);

    let answer3 = RwSignal::default();

    let on_click3 = move || {
        answer1.set(QuizAnswer::default());
        answer2.set(QuizAnswer::default())
    };

    view! {
        <Space vertical=true>
            <Space align=SpaceAlign::Center justify=SpaceJustify::Center>
                <div style="font-size: 2.7rem; font-weight: 500">
                    Choices
                </div>
            </Space>
            <Space align=SpaceAlign::Center justify=SpaceJustify::Center gap=SpaceGap::Size(4)>
                <QuizButton name="I".into() answer=answer1 on_click=on_click1/>
                <QuizButton name="II".into() answer=answer2 on_click=on_click2/>
                <QuizButton name="Reset".into() answer=answer3 on_click=on_click3/>
            </Space>
        </Space>
    }
}
