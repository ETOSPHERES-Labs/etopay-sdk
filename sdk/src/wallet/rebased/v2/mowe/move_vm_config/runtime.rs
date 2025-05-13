#[derive(Clone, Debug)]
pub struct VMProfilerConfig {
    /// User configured full path override
    pub full_path: std::path::PathBuf,
    /// Whether or not to track bytecode instructions
    pub track_bytecode_instructions: bool,
    /// Whether or not to use the long name for functions
    pub use_long_function_name: bool,
}
