## Building the public docs

To build the public docs, you need to do some preparation. First, setup a new python virtual environment and install the required python and nodejs packages:

```bash
python3 -m venv .venv
source .venv/bin/activate
pip install mkdocs-material mike mkdocs-git-revision-date-localized-plugin mkdocs-git-committers-plugin-2 mkdocs-git-authors-plugin mkdocs-typedoc
pnpm install
```

Next you need to build the `wasm` bindings, since the generated typescript definitions are used to publish its API reference: (you can skip this step if you have already built the bindings once)

```bash
cd ../sdk/bindings/wasm
wasm-pack build --dev --scope etospheres
```

Similarly for Java, you need to build the bindings and copy over the generated javadoc:
```bash
cd ../sdk/bindings/android
cargo build
make build_javadoc
mv javadoc ../../../docs/docs/
```

And for Rust:
```bash
cd ../sdk
cargo doc --no-deps --all-features --lib
mkdir -p ../docs/docs/rust-docs/doc/
mv ../target/doc/* ../docs/docs/rust-docs/doc/
```

Finally you can build and serve the docs:

```bash
mkdocs serve
```

