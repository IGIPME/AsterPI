use crate::components::navbar::*;
use leptos::prelude::*;
use leptos_router::components::Outlet;
use leptos_router::hooks::use_location;

#[component]
pub fn AppLayout() -> impl IntoView {
    let collapsed = RwSignal::new(false);

    // 获取当前路径（用于高亮）
    let location = use_location();
    let pathname = move || location.pathname.get();

    view! {
        <TopNavbar/>
        <Sidebar collapsed=collapsed active_path=Signal::derive(pathname)/>
        <main class=("main-content", true) class:expanded=move || collapsed.get()>
            <Outlet/>
        </main>
    }
}
