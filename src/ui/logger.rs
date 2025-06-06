use color_eyre::eyre::WrapErr as _;

use crate::event::Event;

pub struct LoggerUI;

impl super::UI for LoggerUI {
    fn new() -> color_eyre::Result<Self> {
        env_logger::Builder::new()
            .filter_level(log::LevelFilter::Info)
            .try_init()
            .wrap_err("failed to initialize env_logger")?;
        Ok(Self {})
    }

    fn set_log_level(log_level: log::LevelFilter) {
        log::set_max_level(log_level);
    }

    async fn run(
        self,
        _tx: tokio::sync::mpsc::UnboundedSender<Event>,
        _rx: tokio::sync::mpsc::UnboundedReceiver<Event>,
    ) -> color_eyre::Result<()> {
        Ok(())
    }
}
