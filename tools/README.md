# `jnigen`

This is the folder for the crates that make up the `jnigen` macro and build script crates.

- `jnigen-macro` contains the definition of a procmacro that expands the rust functions
  into JNI-compatible functions, including conversion to/from the JNI types (for types
  like `bool` and `String`) and handling of panics and `Result` return types by raising
  `Exceptions` on the Java side.
- `jnigen-build` has code for generating the java wrapper file (with the `private static native ...`
  definitions) in a build script, as well as writing the generated code with the correct
  java folder structure to the file-system.
- `jnigen-common` contains code shared between the `macro` and `build` crates, mostly regarding
  parsing of argument and return values and extracting the correct type names for Rust / Java and
  for generating the correct wrappers / conversion methods in the Rust code.
- `jnigen-integration-tests` contains a full example of how the crates can be used in form of
  binding integration tests, including compilation of the generated Java
  code and execution of java unit tests using a `gradle` project. This serves as the main
  documentation of how to use the crates.

