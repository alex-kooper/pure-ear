use leptos::prelude::*;
use pure_ear::App;
use thaw::ConfigProvider;

fn main() {
    // set up logging
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <ConfigProvider>
                <App />
            </ConfigProvider>
        }
    })
}
