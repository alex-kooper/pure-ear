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
pub fn QuizButton<F: Fn() + 'static>(
    name: String,
    answer: RwSignal<QuizAnswer>,
    on_click: F,
) -> impl IntoView {
    use QuizAnswer::*;

    let color = Signal::derive(move || match answer() {
        None => ButtonColor::Primary,
        Wrong => ButtonColor::Error,
        Right => ButtonColor::Success,
    });

    let on_click = Callback::new(move |_| on_click());

    view! {
        <Button style="width:4em" color on_click>
            {name}
        </Button>
    }
}
