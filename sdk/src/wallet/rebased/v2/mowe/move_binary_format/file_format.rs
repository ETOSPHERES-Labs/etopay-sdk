/// Type parameters are encoded as indices. This index can also be used to
/// lookup the kind of a type parameter in the `FunctionHandle` and
/// `DatatypeHandle`.
pub type TypeParameterIndex = u16;

/// Index into the code stream for a jump. The offset is relative to the
/// beginning of the instruction stream.
pub type CodeOffset = u16;
