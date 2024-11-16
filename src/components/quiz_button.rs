use leptos::*;
use thaw::*;

#[derive(Default, Clone, Copy)]
pub enum QuizAnswer {
    #[default]
    None,
    Wrong,
    Right,
}

#[component]
pub fn QuizButton(name: String, answer: RwSignal<QuizAnswer>) -> impl IntoView {
    use QuizAnswer::*;

    let color = Signal::derive(move || match answer() {
        None => ButtonColor::Primary,
        Wrong => ButtonColor::Error,
        Right => ButtonColor::Success,
    });

    view! {
        <Button color>
            {name}
        </Button>
    }
}
