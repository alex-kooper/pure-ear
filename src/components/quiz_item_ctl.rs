use crate::{
    components::quiz_button::{QuizButton, QuizButtonState},
    model::{music::scale_degree::ScaleDegree, quiz_item::QuizItem},
};
use leptos::*;
use thaw::*;

#[component]
pub fn QuizItemCtl(quiz_item: RwSignal<QuizItem>) -> impl IntoView {
    view! {
        <Space vertical=true>
            <Space align=SpaceAlign::Center justify=SpaceJustify::Center>
                <div style="font-size: 2.7rem; font-weight: 500">Choices</div>
            </Space>
            <div style="display:inline-flex; align-items:center; justify-content: center; gap: 0.3em">
                {ScaleDegree::major_scale_degrees()
                    .map(|degree| {
                        let button_state: Signal<QuizButtonState> = Signal::derive(move || {
                            quiz_item
                                .with(|quiz_item| {
                                    if quiz_item.has_answer(degree) {
                                        if quiz_item.question() == degree {
                                            QuizButtonState::Correct
                                        } else {
                                            QuizButtonState::Incorrect
                                        }
                                    } else {
                                        QuizButtonState::Unselected
                                    }
                                })
                        });
                        let on_click = move || {
                            quiz_item
                                .update(|quiz_item| {
                                    quiz_item.answer_with(degree);
                                })
                        };

                        view! {
                            <div>
                                <QuizButton name=format!("{}", degree) state=button_state on_click />
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </Space>
    }
}
