use crate::icons::*;
use crate::i18n::use_i18n;
use leptos::prelude::*;
use leptos_i18n::t;
use leptos_router::hooks::use_navigate;
use leptos_router::NavigateOptions;

// 导航项数据结构
#[derive(Clone)]
pub struct NavItem {
    pub label: &'static str,
    pub path: &'static str,
    pub icon: fn() -> AnyView,
}

// 顶部导航栏
#[component]
pub fn TopNavbar() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <nav class="top-navbar">
            {/* 左侧 */}
            <div class="navbar-left">
                <a href="/" class="logo">
                    <svg viewBox="0 0 24 24" fill="#e24329">
                        <path d="M12 2L2 7l10 5 10-5-10-5z"/>
                        <path d="M2 17l10 5 10-5M2 12l10 5 10-5"/>
                    </svg>
                    <span>AsterPI</span>
                </a>

                <span class="divider"></span>

                <div class="project-selector">
                    <span>
                        {t!(i18n, project)} / {t!(i18n, group)}
                    </span>
                    <span class="chevron"><IconChevronDown/></span>
                </div>

                <div class="search-box">
                    <IconSearch/>
                    <input type="text" placeholder="搜索或跳转到..." />
                    <span class="kbd-hint">K</span>
                </div>
            </div>

            {/* 右侧 */}
            <div class="navbar-right">
                <button class="nav-icon-btn" title="新建">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <line x1="12" y1="5" x2="12" y2="19"/>
                        <line x1="5" y1="12" x2="19" y2="12"/>
                    </svg>
                </button>

                <button class="nav-icon-btn" title="代办事项">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <rect x="3" y="4" width="18" height="18" rx="2" ry="2"/>
                        <line x1="16" y1="2" x2="16" y2="6"/>
                        <line x1="8" y1="2" x2="8" y2="6"/>
                        <line x1="3" y1="10" x2="21" y2="10"/>
                    </svg>
                </button>

                <button class="nav-icon-btn" title="通知">
                    <IconBell/>
                    <span class="badge"></span>
                </button>

                <button class="nav-icon-btn avatar" title="用户">
                    <img src="https://ui-avatars.com/api/?name=User&background=4a9eff&color=fff&size=34" alt="avatar" />
                </button>
            </div>
        </nav>
    }
}

// 侧边栏
#[component]
pub fn Sidebar(
    #[prop(into)] collapsed: RwSignal<bool>,
    #[prop(into)] active_path: Signal<String>,
) -> impl IntoView {
    let navigate = use_navigate();

    // 导航菜单项
    let nav_items = vec![
        NavItem { label: "项目", path: "/projects", icon: || IconHome().into_any() },
        NavItem { label: "代码仓库", path: "/repos", icon: || IconCode().into_any() },
        NavItem { label: "议题", path: "/issues", icon: || IconIssue().into_any() },
        NavItem { label: "合并请求", path: "/merge_requests", icon: || IconMerge().into_any() },
        NavItem { label: "设置", path: "/settings", icon: || IconSettings().into_any() },
    ];

    let is_active = move |path: &str| {
        let path = path.to_string();
        move || {
            let current = active_path.get();
            current == path || (path != "/" && current.starts_with(&path))
        }
    };

    view! {
        <aside class:sidebar=true class:collapsed=move || collapsed.get()>
            <div class="sidebar-section-title">菜单</div>

            {nav_items.into_iter().map(|item| {
                let path = item.path;
                let label = item.label;
                let icon = item.icon;
                let active = is_active(path);
                let navigate = navigate.clone();
                view! {
                    <a
                        href=path
                        class="nav-item"
                        class:active=active
                        title=move || if collapsed.get() { label } else { "" }
                        on:click=move |ev| {
                            ev.prevent_default();
                            let _ = navigate(path, NavigateOptions::default());
                        }
                    >
                        <span class="icon">{icon()}</span>
                        <span class="label">{label}</span>
                    </a>
                }
            }).collect_view()}

            {/* 折叠切换按钮 */}
            <div style="flex: 1;"></div>

            <div class="sidebar-footer">
                <div class="avatar-small">U</div>
                <div class="user-info">
                    <div class="name">用户</div>
                    <div class="username">@username</div>
                </div>
                <button
                    class="collapse-toggle"
                    class:rotated=move || !collapsed.get()
                    on:click=move |_| collapsed.update(|c| *c = !*c)
                    title=move || if collapsed.get() { "展开侧边栏" } else { "折叠侧边栏" }
                >
                    <IconChevronLeft/>
                </button>
            </div>
        </aside>
    }
}
