use leptos::prelude::*;

/// A parameterized incrementing button
#[component]
#[allow(dead_code)]
pub fn Button(#[prop(default = 1)] increment: i32) -> impl IntoView {
    let (count, set_count) = signal(0);
    view! {
        <button on:click=move |_| {
            set_count(count() + increment)
        }>"Click me! Number of Clicks: " {count}</button>
    }
}
