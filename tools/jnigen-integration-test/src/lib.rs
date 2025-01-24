/// This module contains test functions to be called from the java unit tests.
#[jnigen_macro::generate("com.jnigen.tests.MyTestClass")]
mod ffi {

    // simple passthrough functions to test round-trip of values
    pub fn passthroughI32(value: i32) -> i32 {
        value
    }
    pub fn passthroughI64(value: i64) -> i64 {
        value
    }
    pub fn passthroughF64(value: f64) -> f64 {
        value
    }
    pub fn passthroughF32(value: f32) -> f32 {
        value
    }
    pub fn passthroughBool(value: bool) -> bool {
        value
    }
    pub fn passthroughString(value: String) -> String {
        value
    }

    // passthrough with modification
    pub fn passthroughModI32(value: i32) -> i32 {
        value + 1
    }
    pub fn passthroughModI64(value: i64) -> i64 {
        value + 1
    }
    pub fn passthroughModF64(value: f64) -> f64 {
        value + 1.0
    }
    pub fn passthroughModF32(value: f32) -> f32 {
        value + 1.0
    }
    pub fn passthroughModBool(value: bool) -> bool {
        !value
    }
    pub fn passthroughModString(value: String) -> String {
        format!("{value}Mod")
    }

    // panicks / exceptions
    #[allow(unreachable_code, clippy::diverging_sub_expression)]
    pub fn shouldPanic() {
        panic!();
    }

    #[allow(unreachable_code, clippy::diverging_sub_expression)]
    pub fn shouldPanicResultF64() -> Result<f64, String> {
        panic!();
    }

    #[allow(unreachable_code, clippy::diverging_sub_expression)]
    pub fn shouldPanicResultString() -> Result<String, String> {
        panic!();
    }

    // result
    pub fn returnsResultOk() -> Result<i32, String> {
        Ok(10)
    }
    pub fn returnsResultErrOkUnit() -> Result<(), String> {
        Err("Error Text".to_string())
    }

    pub fn returnsResultErrOkString() -> Result<String, String> {
        Err("Error Text".to_string())
    }
    pub fn returnsResultErrOkBool() -> Result<bool, String> {
        Err("Error Text".to_string())
    }

    // custom public name
    #[public_name = "customPublicName"]
    pub fn internalName() {
        // Intentionally left empty
    }

    // vector / ArrayList
    pub fn stringVecArgument(values: Vec<String>) -> String {
        values.join("")
    }

    // vector / ArrayList
    pub fn bytesVecArgument(values: Vec<u8>) -> String {
        format!("{:?}", values)
    }
    pub fn bytesVecReturn(values: Vec<u8>) -> Vec<u8> {
        values
    }
    pub fn bytesVecReturnResult(values: Vec<u8>) -> Result<Vec<u8>, String> {
        Ok(values)
    }
    pub fn bytesVecReturnResultErr() -> Result<Vec<u8>, String> {
        Err("error".to_string())
    }

    // Option
    pub fn optionString(value: Option<String>) -> String {
        format!("{value:?}")
    }
    pub fn optionByteArray(value: Option<Vec<u8>>) -> i32 {
        match value {
            Some(values) => values.iter().sum::<u8>() as i32,
            None => -1,
        }
    }
    pub fn optionStringArray(value: Option<Vec<String>>) -> String {
        format!("{value:?}")
    }

    pub fn optionStringReturn(is_none: bool) -> Option<String> {
        if is_none {
            None
        } else {
            Some("string".to_string())
        }
    }
}
