use leptos::prelude::*;
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
pub fn QuizButton<F: Fn() + Send + Sync + 'static>(
    name: String,
    state: Signal<QuizButtonState>,
    on_click: F,
) -> impl IntoView {
    let disabled = Signal::derive(move || state() == QuizButtonState::Disabled);
    let on_click = move |_| on_click();

    view! {
        <Button attr:style="width:4em" on_click disabled>
            {name}
        </Button>
    }
}
