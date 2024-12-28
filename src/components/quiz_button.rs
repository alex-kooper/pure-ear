use leptos::*;
use thaw::*;

#[derive(Default, Clone, Copy)]
#[allow(dead_code)]
pub enum QuizButtonState {
    #[default]
    Unselected,
    Incorrect,
    Correct
}

#[component]
pub fn QuizButton<F: Fn() + 'static>(
    name: String,
    state: Signal<QuizButtonState>,
    on_click: F,
) -> impl IntoView {
    use QuizButtonState::*;

    let color = Signal::derive(move || match state() {
        Unselected => ButtonColor::Primary,
        Incorrect => ButtonColor::Error,
        Correct => ButtonColor::Success,
    });

    let on_click = Callback::new(move |_| on_click());

    view! {
        <Button style="width:4em" color on_click>
            {name}
        </Button>
    }
}
