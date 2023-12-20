mod error;

pub use self::error::{{ project-name | upper_camel_case }}Error;

pub type {{ project-name | upper_camel_case }}Result<T> = std::result::Result<T, {{ project-name | upper_camel_case }}Error>;
{% if crate_type == "bin" -%}
{%- if async %}
pub async fn run() -> {{ project-name | upper_camel_case }}Result<()> {
    std::future::ready(Ok(())).await
{%- else %}
pub fn run() -> {{ project-name | upper_camel_case }}Result<()> {
    Ok(())
{%- endif %}
}
{% endif %}