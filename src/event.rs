use std::collections::HashMap;

use crate::proxy::ProxyType;

pub(crate) enum AppEvent {
    GeoDbTotal(Option<u64>),
    GeoDbDownloaded(usize),

    SourcesTotal(ProxyType, usize),
    SourceScraped(ProxyType),

    TotalProxies(ProxyType, usize),
    ProxyChecked(ProxyType),
    ProxyWorking(ProxyType),

    Done,
}

pub(crate) enum Event {
    Tick,
    Crossterm(crossterm::event::Event),
    App(AppEvent),
}

#[derive(Default)]
pub(crate) enum AppMode {
    #[default]
    Running,
    Done,
    Quit,
}

#[derive(Default)]
pub(crate) struct AppState {
    pub(crate) mode: AppMode,

    pub(crate) geodb_total: u64,
    pub(crate) geodb_downloaded: usize,

    pub(crate) sources_total: HashMap<ProxyType, usize>,
    pub(crate) sources_scraped: HashMap<ProxyType, usize>,

    pub(crate) proxies_total: HashMap<ProxyType, usize>,
    pub(crate) proxies_checked: HashMap<ProxyType, usize>,
    pub(crate) proxies_working: HashMap<ProxyType, usize>,
}

impl AppState {
    pub(crate) fn new() -> Self {
        AppState::default()
    }
}
