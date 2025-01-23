use leptos::{either::Either, prelude::*};
use thaw::*;

#[component]
pub fn Meter(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] color: Signal<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let style = move || {
        format!("color: {}; border-color: {}; font-size: var(--fontSizeBase300); font-weight: var(--fontWeightSemibold); border: 4px solid; border-radius: 50%;  width: 3em; height: 3em; display: flex; align-items: center; justify-content: center;", color(), color())
    };

    view! {
        <Flex vertical=true align=FlexAlign::Center gap=FlexGap::Size(0)>
            <div style=style>{value}</div>
            {if let Some(children) = children {
                Either::Left(children())
            } else {
                Either::Right(())
            }}
        </Flex>
    }
}
