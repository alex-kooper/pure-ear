use crate::{components::quiz_item_ctl::QuizItemCtl, model::quiz::Quiz};

use leptos::prelude::*;
use thaw::*;

#[component]
pub fn QuizProgress(current_item: Signal<usize>, total_items: usize) -> impl IntoView {
    let percent = move || current_item() * 100 / total_items;
    let text = move || format!("{} of {} ({:.02}%)", current_item(), total_items, percent());

    view! {
        <Flex vertical=true align=FlexAlign::Center gap=FlexGap::Small style="width: 100%">
            <Text>{text}</Text>
            <ProgressBar
                value=Signal::derive(move || current_item() as f64)
                color=ProgressBarColor::Success
                max=Signal::derive(move || total_items as f64)
                style:height="7px"
            />
        </Flex>
    }
}

#[component]
pub fn QuizCtl() -> impl IntoView {
    let quiz = RwSignal::new(Quiz::new(10));
    let quiz_item = RwSignal::new(quiz.write().generate_item());

    let total_items = quiz.read().total_items();
    let solved = Signal::derive(move || quiz.read().solved());

    let next_disabled =
        Signal::derive(move || !quiz_item.read().is_solved() || quiz.read().solved() >= total_items);

    let on_next = move |_| quiz_item.set(quiz.write().generate_item());
    let on_complete = move |quiz_item| quiz.write().add_solved(quiz_item);

    view! {
        <Flex justify=FlexJustify::Center>
            <Flex vertical=true align=FlexAlign::Center gap=FlexGap::Size(20)>
                <Text
                    tag=TextTag::H1
                    style="font-size: var(--fontSizeBase900); font-weight: var(--fontWeightSemibold); line-height: var(--lineHeightBase900)"
                >
                    "Scale Degree Quiz"
                </Text>

                <QuizProgress current_item=solved total_items=quiz.read().total_items() />

                <Text>
                    {move || quiz.read().correct()} " of " {move || quiz.read().solved()} " correct"
                </Text>

                <QuizItemCtl quiz_item on_complete />

                <Divider style:margin-top="2em" />

                <Flex>
                    <Button
                        size=ButtonSize::Large
                        appearance=ButtonAppearance::Primary
                        disabled=next_disabled
                        on_click=on_next
                    >
                        Next
                    </Button>
                    <Button size=ButtonSize::Large appearance=ButtonAppearance::Primary>
                        End Quiz
                    </Button>
                </Flex>
            </Flex>
        </Flex>
    }
}
