/// This file configures and generates metadata for the SDK build process, including information
/// about the environment, version, and other build-related properties.
fn main() -> shadow_rs::SdResult<()> {
    // Create a default deny set which excludes `CARGO_METADATA`.
    let mut deny = shadow_rs::default_deny();

    // Exclude additional unnecessary properties.
    deny.insert(shadow_rs::CARGO_TREE);
    deny.insert(shadow_rs::CARGO_MANIFEST_DIR);
    deny.insert(shadow_rs::COMMIT_AUTHOR);
    deny.insert(shadow_rs::COMMIT_EMAIL);

    // Generate the build information, excluding the properties specified in `deny`.
    shadow_rs::new_deny(deny)
}
