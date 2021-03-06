use tracing::{error, instrument};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    tracing_subscriber::registry()
        // .with(tracing_subscriber::EnvFilter::from_default_env())
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
        traces_sample_rate: 1.0,
        ..Default::default()
    });

    make_an_error("span_field_value");
}

// "span_field" doesn't appear at sentry.io
// "event_field" appears at sentry.io
#[instrument]
fn make_an_error(span_field: &str) {
    error!(event_field = "event_field_value", "test");
}
