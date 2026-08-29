use leptos::prelude::*;

#[component]
pub fn IconSvg(#[prop(into)] class: String, children: Children) -> impl IntoView {
    view! {
        <svg class={class} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            {children()}
        </svg>
    }
}
