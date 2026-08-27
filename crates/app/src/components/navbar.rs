use leptos::prelude::*;
use leptos_router::components::A;
use leptos_i18n::t;
use crate::i18n::use_i18n;
use crate::i18n::Locale;

#[component]
pub fn Navbar() -> impl IntoView {
    let i18n = use_i18n();
}
