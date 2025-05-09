pub struct ModeInfo {
    pub id: usize,
    pub name: &'static str,
    pub entry: fn(),
}

pub fn list() -> Vec<ModeInfo> {
    vec![
        ModeInfo { id: 1, name: "🎭 互動小說模式", entry: crate::app::interactive_novel::run },
        ModeInfo { id: 2, name: "📚 KB Dashboard", entry: crate::tools::tui_dashboard::run },
        ModeInfo { id: 3, name: "🔄 自我升級", entry: crate::ui::self_upgrade_menu::run },
        ModeInfo { id: 4, name: "⚙️ 設定", entry: crate::app::settings::run },
        ModeInfo { id: 5, name: "🔍 系統自省", entry: crate::app::self_inspect_menu::run },
    ]
}
