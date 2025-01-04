use leptos::prelude::*;
use thaw::*;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
    let on_click = move |_| on_click();

    let color = move || match state() {
        QuizButtonState::Unselected => "",
        QuizButtonState::Disabled => "var(--colorBrandBackground)",
        QuizButtonState::Incorrect => "red",
        QuizButtonState::Correct => "green",
    };

    let disabled = Signal::derive(move || state() == QuizButtonState::Disabled);

    let cursor = move || {
        if state() == QuizButtonState::Unselected {
            ""
        } else {
            "not-allowed"
        }
    };

    view! {
        <Button
            style:cursor=cursor
            style:background-color=color
            style:min-width="4em"
            appearance=ButtonAppearance::Primary
            on_click
            disabled
        >
            {name}
        </Button>
    }
}
