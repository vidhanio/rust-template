mod error;

pub use self::error::{{ project-name | upper_camel_case }}Error;

pub type {{ project-name | upper_camel_case }}Result<T> = std::result::Result<T, {{ project-name | upper_camel_case }}Error>;
{% if crate_type == "bin" -%}
{%- if async %}
pub async fn run() -> {{ project-name | upper_camel_case }}Result<()> {
{%- else %}
pub fn run() -> {{ project-name | upper_camel_case }}Result<()> {
{%- endif %}
    Ok(())
}
{% endif %}