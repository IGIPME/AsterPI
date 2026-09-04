use leptos::prelude::*;
use leptos_router::components::A;
use leptos_i18n::t;
use crate::i18n::use_i18n;

#[component]
pub fn Navbar() -> impl IntoView {
    let i18n = use_i18n();

    let (community_open, set_community_open) = signal(false);
    let (lang_open, set_lang_open) = signal(false);
    let (theme_open, set_theme_open) = signal(false);

    view! {
        <nav class="navbar">
            <div class="navbar-left">
                <div class="logo-wrapper">
                    <A href="/">
                        <img src="/logo.png" alt="AsterPI Logo" />
                    </A>
                </div>
                <div class="nav-links">
                    <A href="/home">
                        {t!(i18n, home)}
                    </A>
                    <A href="/hardware">
                        {t!(i18n, hardware)}
                    </A>
                    <A href="/models">
                        {t!(i18n, models)}
                    </A>
                    <A href="/datasets">
                        {t!(i18n, datasets)}
                    </A>
                    <A href="/studio">
                        {t!(i18n, studio)}
                    </A>
                    <A href="/spotlight">
                        {t!(i18n, spotlight)}
                    </A>
                    <a href="https://docs.asterpi.com" target="_blank" rel="noopener noreferrer">
                        {t!(i18n, docs)}
                    </a>
                </div>
                <div class="dropdown">
                    <button class="dropdown-trigger" on:click=move |_| {
                        set_community_open.update(|v| *v = !*v);
                        set_lang_open.set(false);
                        set_theme_open.set(false);
                    }>
                        {t!(i18n, community)}
                    </button>
                </div>
            </div>

            <div class="navbar-right">
                <div class="nav-links">
                    <a href="https://github.com/IGIPME/AsterPI" target="_blank" rel="noopener noreferrer">
                        GitHub
                    </a>
                </div>
                <div class="dropdown">
                    <button class="dropdown-trigger" on:click=move |_| {
                        set_lang_open.update(|v| *v = !*v);
                        set_community_open.set(false);
                        set_theme_open.set(false);
                    }>
                        translate
                    </button>
                </div>
            </div>
        </nav>
    }
}
