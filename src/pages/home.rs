use crate::components::{
    counter_btn::Button,
    quiz_button::{QuizAnswer, QuizButton},
};
use leptos::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let answer1 = RwSignal::default();
    let on_click1 = move || answer1.set(QuizAnswer::Right);

    let answer2 = RwSignal::default();
    let on_click2 = move || answer2.set(QuizAnswer::Wrong);

    let answer3 = RwSignal::default();
    
    let on_click3 = move || {
        answer1.set(QuizAnswer::default());
        answer2.set(QuizAnswer::default())
    };

    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <div class="container">

                <picture>
                    <source
                        srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg"
                        media="(prefers-color-scheme: dark)"
                    />
                    <img
                        src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg"
                        alt="Leptos Logo"
                        height="200"
                        width="400"
                    />
                </picture>

                <h1>"Welcome to Leptos"</h1>

                <div class="buttons">
                    <Button/>
                    <Button increment=5/>
                </div>
                <div>
                    <QuizButton name="I".into() answer=answer1 on_click=on_click1/>
                    <QuizButton name="II".into() answer=answer2 on_click=on_click2/>
                    <QuizButton name="Reset".into() answer=answer3 on_click=on_click3/>
                </div>

            </div>
        </ErrorBoundary>
    }
}
