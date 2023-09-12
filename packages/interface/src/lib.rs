pub mod note;
pub mod proxy;
pub mod voice;
pub use note::PolytoneNote;
pub use proxy::PolytoneProxy;
pub use voice::PolytoneVoice;

pub mod deploy;
pub mod interface;

pub use interface::Polytone;
pub use interface::PolytoneConnection;

#[cfg(test)]
pub mod tests;
