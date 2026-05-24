use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    EnvFilter, filter::ParseError, layer::SubscriberExt, util::SubscriberInitExt,
};

const PKG_NAME: &str = env!("CARGO_CRATE_NAME");

pub struct Telemetry {
    _guard: WorkerGuard,
}

impl Telemetry {
    pub fn init(filter: &str) -> anyhow::Result<Self> {
        let filter = filter_from_value(filter)?;
        let (writer, guard) = tracing_appender::non_blocking(std::io::stdout());

        tracing_subscriber::registry()
            .with(filter)
            .with(tracing_subscriber::fmt::layer().with_writer(writer))
            .try_init()?;

        Ok(Self { _guard: guard })
    }
}

pub fn filter_from_value(value: &str) -> Result<EnvFilter, ParseError> {
    match value.trim() {
        "none" | "off" => Ok(EnvFilter::default()),
        "error" => Ok(EnvFilter::default().add_directive(tracing::Level::ERROR.into())),
        "warn" => Ok(EnvFilter::default().add_directive(tracing::Level::WARN.into())),
        "info" => Ok(EnvFilter::default()
            .add_directive(tracing::Level::WARN.into())
            .add_directive(format!("{PKG_NAME}=info").parse()?)),
        "debug" => Ok(EnvFilter::default()
            .add_directive(tracing::Level::WARN.into())
            .add_directive(format!("{PKG_NAME}=debug").parse()?)),
        "trace" => Ok(EnvFilter::default()
            .add_directive(tracing::Level::WARN.into())
            .add_directive(format!("{PKG_NAME}=trace").parse()?)),
        custom => EnvFilter::builder().parse(custom),
    }
}
