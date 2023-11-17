use thiserror::Error;

#[derive(Error, Debug)]
pub enum WrcError {
    #[error("Could not decode the packet.")]
    DecodingError {
        msg: String,
        #[source]
        source: bincode::Error,
    },
}
