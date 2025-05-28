#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    InvalidParams = -32602,
}

impl ErrorCode {
    pub fn code(self) -> i32 {
        self as i32
    }
}
