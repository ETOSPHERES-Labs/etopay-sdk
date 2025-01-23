#![allow(clippy::unwrap_used)]

fn main() {
    // generate jni java bindings
    jnigen_build::generate("src/lib.rs", "jnigenit", "./src/main/java");
}
