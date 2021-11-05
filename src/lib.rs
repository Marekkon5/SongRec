pub mod fingerprinting {
    pub mod communication;
    pub mod algorithm;
    pub mod signature_format;
    mod user_agent;
    mod hanning;
}

// re-exports

pub use fingerprinting::algorithm::SignatureGenerator;
pub use fingerprinting::communication::{recognize_song_from_signature, obtain_raw_cover_image};
pub use fingerprinting::signature_format::{DecodedSignature, FrequencyBand, FrequencyPeak};