use thiserror::Error;

#[derive(Debug, Clone, Copy, Error)]
#[allow(clippy::module_name_repetitions)]
pub enum {{ project-name | upper_camel_case }}Error {}
