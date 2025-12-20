use crate::title_block::TitleBlock;
use leptos::prelude::*;
mod title_block;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <TitleBlock title="roazanas" subtitle="i am just a dev :)"/>
    }
}
