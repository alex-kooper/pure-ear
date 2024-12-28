use leptos::*;
use thaw::*;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum QuizButtonState {
    #[default]
    Unselected,
    Disabled,
    Incorrect,
    Correct,
}

#[component]
pub fn QuizButton<F: Fn() + 'static>(
    name: String,
    state: Signal<QuizButtonState>,
    on_click: F,
) -> impl IntoView {
    let color = Signal::derive(move || match state() {
        QuizButtonState::Unselected => ButtonColor::Primary,
        QuizButtonState::Disabled => ButtonColor::Primary,
        QuizButtonState::Incorrect => ButtonColor::Error,
        QuizButtonState::Correct => ButtonColor::Success,
    });

    let disabled = Signal::derive(move || state() == QuizButtonState::Disabled);
    let on_click = Callback::new(move |_| on_click());

    view! {
        <Button style="width:4em" color on_click disabled>
            {name}
        </Button>
    }
}
