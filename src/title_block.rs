use leptos::prelude::*;
#[component]
pub fn TitleBlock(title: &'static str, subtitle: &'static str) -> impl IntoView {
    view! {
        <div class="title-block">
            <h1>{title}</h1>
            <p>{subtitle}</p>
        </div>
    }
}
