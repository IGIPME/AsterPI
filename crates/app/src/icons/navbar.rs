use crate::icons::class::IconSvg;
use leptos::prelude::*;

#[component]
pub fn IconHome() -> impl IntoView {
    view! {
        <IconSvg class="">
            <path d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-4 0a1 1 0 01-1-1v-4a1 1 0 011-1h2a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 01-1 1h-2z" />
        </IconSvg>
    }
}

#[component]
pub fn IconCode() -> impl IntoView {
    view! {
        <IconSvg class="">
            <polyline points="16 18 22 12 16 6" />
            <polyline points="8 6 2 12 8 18" />
        </IconSvg>
    }
}

#[component]
pub fn IconIssue() -> impl IntoView {
    view! {
        <IconSvg class="">
            <circle cx="12" cy="12" r="10" />
            <line x1="12" y1="8" x2="12" y2="12" />
            <line x1="12" y1="16" x2="12.01" y2="16" />
        </IconSvg>
    }
}

#[component]
pub fn IconMerge() -> impl IntoView {
    view! {
        <IconSvg class="">
            <path d="M7 4v16M7 4l4 4M7 4L3 8" />
            <path d="M17 20v-7a3 3 0 00-3-3H7" />
            <path d="M17 20l-4-4M17 20l4-4" />
        </IconSvg>
    }
}

#[component]
pub fn IconSettings() -> impl IntoView {
    view! {
        <IconSvg class="">
            <circle cx="12" cy="12" r="3" />
            <path d="M19.4 15a1.65 1.65 0 00.33 1.82l.06.06a2 2 0 01-2.83 2.83l-.06-.06a1.65 1.65 0 00-1.82-.33 1.65 1.65 0 00-1 1.51V21a2 2 0 01-4 0v-.09A1.65 1.65 0 009 19.4a1.65 1.65 0 00-1.82.33l-.06.06a2 2 0 01-2.83-2.83l.06-.06A1.65 1.65 0 004.68 15a1.65 1.65 0 00-1.51-1H3a2 2 0 010-4h.09A1.65 1.65 0 004.6 9a1.65 1.65 0 00-.33-1.82l-.06-.06a2 2 0 012.83-2.83l.06.06A1.65 1.65 0 009 4.68a1.65 1.65 0 001-1.51V3a2 2 0 014 0v.09a1.65 1.65 0 001 1.51 1.65 1.65 0 001.82-.33l.06-.06a2 2 0 012.83 2.83l-.06.06A1.65 1.65 0 0019.4 9a1.65 1.65 0 001.51 1H21a2 2 0 010 4h-.09a1.65 1.65 0 00-1.51 1z" />
        </IconSvg>
    }
}

#[component]
pub fn IconHelp() -> impl IntoView {
    view! {
        <IconSvg class="">
            <circle cx="12" cy="12" r="10" />
            <path d="M9.09 9a3 3 0 015.83 1c0 2-3 3-3 3" />
            <line x1="12" y1="17" x2="12.01" y2="17" />
        </IconSvg>
    }
}

#[component]
pub fn IconBell() -> impl IntoView {
    view! {
        <IconSvg class="">
            <path d="M18 8A6 6 0 006 8c0 7-3 9-3 9h18s-3-2-3-9" />
            <path d="M13.73 21a2 2 0 01-3.46 0" />
        </IconSvg>
    }
}

#[component]
pub fn IconSearch() -> impl IntoView {
    view! {
        <IconSvg class="">
            <circle cx="11" cy="11" r="8" />
            <line x1="21" y1="21" x2="16.65" y2="16.65" />
        </IconSvg>
    }
}

#[component]
pub fn IconChevronDown() -> impl IntoView {
    view! {
        <IconSvg class="">
            <polyline points="6 9 12 15 18 9" />
        </IconSvg>
    }
}

#[component]
pub fn IconMenu() -> impl IntoView {
    view! {
        <IconSvg class="">
            <line x1="4" y1="6" x2="20" y2="6" />
            <line x1="4" y1="12" x2="20" y2="12" />
            <line x1="4" y1="8" x2="20" y2="18" />
        </IconSvg>
    }
}

#[component]
pub fn IconChevronLeft() -> impl IntoView {
    view! {
        <IconSvg class="">
            <polyline points="15 18 9 12 15 6" />
        </IconSvg>
    }
}
