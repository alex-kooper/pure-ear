use leptos::prelude::*;
use thaw::*;
use thaw_utils::BoxCallback;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum QuizButtonState {
    #[default]
    Unselected,
    Disabled,
    Incorrect,
    Correct,
}

#[component]
pub fn QuizButton(
    name: String,
    state: Signal<QuizButtonState>,
    #[prop(into)] on_click: BoxCallback,
) -> impl IntoView {
    let on_click = move |_| on_click();

    let color = move || match state() {
        QuizButtonState::Unselected => "",
        QuizButtonState::Disabled => "var(--colorBrandBackground)",
        QuizButtonState::Incorrect => "var(--colorPaletteRedBackground3)",
        QuizButtonState::Correct => "var(--colorPaletteGreenBackground3)",
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
            style:min-width="3.5em"
            appearance=ButtonAppearance::Primary
            on_click
            disabled
        >
            {name}
        </Button>
    }
}
