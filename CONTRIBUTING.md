# Welcome to ETOPay – The Decentralized Payment Ecosystem

## Introduction

ETOPay is an open-source project designed to revolutionize P2P payments with decentralization at its core. By eliminating gatekeepers and respecting user rights, ETOPay provides a secure, legally compliant, and developer-friendly SDK that integrates seamlessly with **cryptocurrency** and **fiat** systems.

With ETOPay, we strive to empower developers, businesses, and end-users with tools that enable _secure payments_, _identity management_, and comprehensive _wallet solutions_, all within a Web3 context.

Our core principles of **Self-realization**, **Awareness**, **Fairness**, **Dexterity**, and **Community** guide us in building and growing this platform. By contributing to ETOPay, you become a part of this mission.

You can read more about our philosophy and principles [here](./sdk/docs/Choosing%20ETOPay/Philosophy.md).

#### Key Features

- Wallet Management with MFA (Multi-factor approvals)
- KYC Onboarding
- Fiat/Cryptocurrency Integration
- Multi-User App Support
- Seamless Identity Integration
- Integrated Invoice and Receipt Generation
- Multi-Platform Compatibility
- European Data Center
- Cyber-Secure

You can read more about our features and use-cases [here](./sdk/docs/Choosing%20ETOPay/Features.md).

## We want to build with you

We love your input! We welcome all forms of contributions, including:

- Reporting and fixing bugs
- Discussing the current state of the code
- Suggesting and implementing new features
- Developing new use-cases for the SDK
- Enhancing documentation or translations

We Use [Github Flow](https://docs.github.com/en/get-started/using-github/github-flow), so all code changes happens through **Pull Requests**.

## Contribution License

In short, when you submit code changes, your submissions are understood to be under the same [AGPLv3](./LICENSE) that covers the project. Feel free to contact the maintainers if that's a concern.

## Report bugs using Github

We use [Github's issues](https://github.com/ETOSPHERES-Labs/etopay-sdk/issues) to track public bugs. Report a bug by [opening a new issue](https://github.com/ETOSPHERES-Labs/etopay-sdk/issues/new). It's that easy!

We appreciate clear and detailed bug reports—they help us understand and resolve issues more effectively!
**Write bug reports with detail, background, and sample code. Great Bug Reports** tend to have:

- A brief summary of the issue or feature request.
- Steps to reproduce the issue (if it’s a bug).
- Expected versus actual behavior.
- Logs, screenshots, or any other relevant information.
- Your environment details (e.g., browser version, OS, or app version).

Please, check for duplicates to see if someone has already reported it. We _love_ thorough bug reports.

## Local Development with Dev Containers

To streamline your development process, we provide a pre-configured development container hosted at [etopay-sdk-devcontainer](https://github.com/ETOSPHERES-Labs/etopay-sdk-devcontainer). This container includes all necessary dependencies and is ready to use with Visual Studio Code.

### Steps to Use the Dev Container

1. **Clone the Dev Container Repository**  
   Clone the [etopay-sdk](https://github.com/ETOSPHERES-Labs/etopay-sdk) repository:

   ```bash
   git clone https://github.com/ETOSPHERES-Labs/etopay-sdk.git
   ```

2. **Open the Project in VS Code**  
   Open the `etopay-sdk` project in Visual Studio Code.

3. **Reopen in Container**  
   Ensure you have the [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) installed. Then, reopen the project in the dev container:
   - Press `F1` in VS Code.
   - Select **Dev Containers: Reopen in Container**.

4. **Start Developing**  
   Once the container is pulled and running, you can start developing immediately. The container includes all required tools and dependencies for the project.

For more details on using dev containers, refer to the [official documentation](https://code.visualstudio.com/docs/devcontainers/containers).

By using the dev container, you ensure a consistent development environment across all contributors. Happy coding!

## Local development requirements (without devcontainer)

### Building from source: Dependencies

To build the ETOPay SDK from source, you will need to install the following dependencies in addition to rustlang:

- `clang`: The C compiler
- `lld`: The LLVM linker
- `protoc`: The Protocol Buffers compiler used by ETOPay

#### Ubuntu/Debian

To install the dependencies on Ubuntu or Debian, run the following commands:

```bash
sudo apt update
sudo apt install clang llvm lld protobuf-compiler
```

#### Fedora

To install the dependencies on Fedora, run the following commands:

```bash
sudo dnf install clang llvm lld protobuf-compiler
```

#### macOS (with Homebrew)

To install the dependencies on macOS using Homebrew, run the following commands:

```bash
brew install clang llvm lld protobuf
```

#### Windows

We recommend using the [DevContainer](#local-development-with-dev-containers) with WSL2 to build the ETOPay SDK from source on Windows. This will provide a consistent and reliable build environment.

## Community

You can reach out to us via Github directly or [email](mailto:lobster@etospheres.com).

## Code of Conduct

See [CODE OF CONDUCT](./CODE_OF_CONDUCT.md)
