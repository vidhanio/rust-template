mod error;

pub use self::error::{{ project-name | upper_camel_case }}Error;

pub type {{ project-name | upper_camel_case }}Result<T> = std::result::Result<T, {{ project-name | upper_camel_case }}Error>;
{% if crate_type == "bin" %}
/// Run the application.
///
/// # Errors
///
/// Returns an error if the application fails to run.
{%- if async %}
pub async fn run() -> {{ project-name | upper_camel_case }}Result<()> {
    std::future::ready(todo!()).await
{%- else %}
pub fn run() -> {{ project-name | upper_camel_case }}Result<()> {
    todo!()
{%- endif %}
}
{% endif %}