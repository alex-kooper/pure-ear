use crate::{
    components::quiz_button::{QuizAnswer, QuizButton},
    model::music::scale_degree::ScaleDegree,
};
use leptos::*;
use thaw::*;

#[component]
pub fn QuizQuestion() -> impl IntoView {
    view! {
        <Space vertical=true>
            <Space align=SpaceAlign::Center justify=SpaceJustify::Center>
                <div style="font-size: 2.7rem; font-weight: 500">
                    Choices
                </div>
            </Space>
            <div style="display:inline-flex; align-items:center; justify-content: center; gap: 0.3em">
                {
                    ScaleDegree::major_scale_degrees().map(|degree| {
                        let answer: RwSignal<QuizAnswer> = RwSignal::default();
                        let on_click = move || answer.set(QuizAnswer::Right);

                        view! {
                            <div>
                                <QuizButton name=format!("{}", degree) answer on_click/>
                            </div>
                        }
                    }).collect_view()
                }
            </div>
        </Space>
    }
}
