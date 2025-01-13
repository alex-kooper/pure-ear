use crate::{components::quiz_item_ctl::QuizItemCtl, model::quiz::Quiz};

use leptos::prelude::*;
use thaw::*;

#[component]
pub fn QuizCtl() -> impl IntoView {
    let quiz = RwSignal::new(Quiz::new());
    let quiz_item = RwSignal::new(quiz.write().generate_item());

    let next_disabled = Signal::derive(move || !quiz_item.read().is_solved());
    let on_next = move |_| quiz_item.set(quiz.write().generate_item());
    let on_complete = move |quiz_item| quiz.write().add_solved(quiz_item);

    view! {
        <Flex vertical=true gap=FlexGap::Large>
            <Flex align=FlexAlign::Center justify=FlexJustify::Center>
                <Caption1Strong style="font-size: 3em; line-height: unset; margin-bottom: 1em">
                    "Scale Degree Quiz"
                </Caption1Strong>
            </Flex>
            <Flex align=FlexAlign::Center justify=FlexJustify::Center>
                <Text>
                    {move || quiz.read().correct()} " of " {move || quiz.read().solved()} " correct"
                </Text>
            </Flex>
            <Flex align=FlexAlign::Center justify=FlexJustify::Center>
                <QuizItemCtl quiz_item on_complete />
            </Flex>
            <Flex align=FlexAlign::Center justify=FlexJustify::Center style:margin-top="3em">
                <Divider />
            </Flex>
            <Flex align=FlexAlign::Center justify=FlexJustify::Center>
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
    }
}
