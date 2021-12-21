use sentry_tracing::EventFilter;
use tracing::{error, instrument, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .with(sentry_tracing::layer())
        .init();

    let _guard = sentry::init(sentry::ClientOptions {
        dsn: Some(
            dbg!(std::env::var("SENTRY_DSN"))
                .expect("SENTRY_DSN is missing")
                .parse()
                .expect("valid sentry DSN"),
        ),
        environment: dbg!(std::env::var("SENTRY_ENVIRONMENT"))
            .ok()
            .map(Into::into),
        ..Default::default()
    });

    make_an_error("abc");
}

#[instrument]
fn make_an_error(arg: &str) {
    error!("test");
}
