#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("{source}")]
    FromEnvError {
        #[from]
        source: tracing_subscriber::filter::FromEnvError,
    },

    #[error("{source}")]
    StdIo {
        #[from]
        source: std::io::Error,
    },
}
