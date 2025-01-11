use crate::components::quiz_ctl::QuizCtl;
use leptos::prelude::*;
use thaw::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
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
            <Flex vertical=true>
                <Flex align=FlexAlign::Center justify=FlexJustify::Center>
                    <QuizCtl />
                </Flex>
            </Flex>
        </ErrorBoundary>
    }
}
