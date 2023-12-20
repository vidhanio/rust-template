use std::env;

use tracing_subscriber::EnvFilter;
{% if async %}
#[tokio::main]
async fn main() -> color_eyre::Result<()> {
{% else %}
fn main() -> color_eyre::Result<()> {
{% endif %}    color_eyre::install()?;

    _ = dotenvy::dotenv();

    tracing_subscriber::fmt()
        .with_env_filter(
            env::var(EnvFilter::DEFAULT_ENV)
                .as_deref()
                .unwrap_or("info,test=debug"),
        )
        .init();
{% if async %}
    {{ crate_name }}::run().await?;
{% else %}
    {{ crate_name }}::run()?;
{%- endif %}
    Ok(())
}
