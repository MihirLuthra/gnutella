use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to serialize the input: {}", reason))]
    SerializationFailed { reason: String },
    #[snafu(display("Failed to deserialize the input: {}", reason))]
    DeserializationFailed { reason: String },
}
