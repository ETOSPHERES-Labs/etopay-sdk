use super::RebasedError;

pub trait ToFromBytes: AsRef<[u8]> + std::fmt::Debug + Sized {
    /// Parse an object from its byte representation
    fn from_bytes(bytes: &[u8]) -> Result<Self, RebasedError>;

    /// Borrow a byte slice representing the serialized form of this object
    fn as_bytes(&self) -> &[u8] {
        self.as_ref()
    }
}
