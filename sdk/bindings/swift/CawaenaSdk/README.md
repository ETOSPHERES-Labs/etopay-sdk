# Cawaena Swift Package

This repository contains the swift package for the Cawaena SDK.

## Usage

Simply add this repository in to the `dependencies` in the `Package.swift` file like so:

```swift
import PackageDescription

let package = Package(
    name: "program",
    dependencies: [
        .package(url: "https://github.com/ETOSPHERES-Labs/cawaena-sdk-swift.git", from: "0.0.1")
    ],
    targets: [
        .executableTarget(
            name: "main",
            dependencies: [
                .product(name: "CawaenaSdk", package: "cawaena-swift")
            ]),
    ]
)
```

The `CawaenaSdk` module is then available for import in your project.

## Access

For the Swift Package Manager to be able to download the binaries stored in JFrog, you need to setup
an access token in your `~/.netrc` file.

1. Visit [JFrog](https://repo.farmunited.com/) and log in. In the top right corner, click your name
   and then on _Edit Profile_. Under _Identity Tokens_ click _Generate an Identity Token_ and optionally
   give the token a name.

2. If not already existing, create the file `.netrc` in your home folder and add the follwing lines (e.g. using `nano ~/.netrc`)

    ```
    machine repo.farmunited.com
        login <your JFrog username>
        password <your access token>
    ```

3. You should now be able to access and use the package in your Swift projects!
