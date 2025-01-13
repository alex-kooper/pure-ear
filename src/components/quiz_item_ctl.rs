use crate::{
    components::quiz_button::{QuizButton, QuizButtonState},
    model::{music::scale_degree::ScaleDegree, quiz_item::QuizItem},
};
use leptos::prelude::*;
use thaw::*;
use thaw_utils::ArcOneCallback;

#[component]
pub fn QuizItemCtl(
    quiz_item: RwSignal<QuizItem>,
    #[prop(into)] on_complete: ArcOneCallback<QuizItem>,
) -> impl IntoView {
    let button_state = move |degree| {
        Signal::derive(move || {
            quiz_item.with(|quiz_item| {
                if quiz_item.has_answer(degree) {
                    if quiz_item.question() == degree {
                        QuizButtonState::Correct
                    } else {
                        QuizButtonState::Incorrect
                    }
                } else if quiz_item.is_solved() {
                    QuizButtonState::Disabled
                } else {
                    QuizButtonState::Unselected
                }
            })
        })
    };

    let on_click = move |degree| {
        let on_complete = on_complete.clone();

        move || {
            quiz_item.write().answer_with(degree);

            quiz_item.with(|quiz_item| {
                if quiz_item.is_solved() {
                    on_complete(quiz_item.clone());
                }
            });
        }
    };

    view! {
        <Flex vertical=true>
            <Flex align=FlexAlign::Center justify=FlexJustify::Center>
                <Caption1 style="font-size: 2.5em; line-height: unset">Choices</Caption1>
            </Flex>
            <div style="display:inline-flex; align-items:center; justify-content: center; gap: 0.3em">
                {ScaleDegree::major_scale_degrees()
                    .iter()
                    .map(|degree| {
                        view! {
                            <QuizButton
                                name=format!("{}", degree)
                                state=button_state(*degree)
                                on_click=on_click(*degree)
                            />
                        }
                    })
                    .collect_view()}
            </div>
        </Flex>
    }
}
