# Clawbird

## Setup

* Install VSCode Dioxus Extension - https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus
* Install Rust Analyser - https://rust-analyzer.github.io/manual.html#vs-code
* Install Rust - https://dioxuslabs.com/learn/0.6/getting_started/#install-rust
* Install binstall - https://dioxuslabs.com/learn/0.6/getting_started/#install-cargo-binstall
* Install Dioxus CLI
  ```
  cargo binstall dioxus-cli
  ```
  * `dioxus` crate: the core Dioxus framework
  * `dx` tool: a CLI tool for building Dioxus apps (hot reloading, bundling)
    * **TODO** - Setup to use with VSCode
* Setup iOS development support - https://dioxuslabs.com/learn/0.6/getting_started/#ios
  * Update Xcode, and choose to install iOS SDK in popup
* Setup Android development support - https://dioxuslabs.com/learn/0.6/getting_started/#android
  * Note: At the appropriate step, add environment variables to ~/.zshrc permanently by running the following, ensuring the NDK version matches the version that was installed.
  ```
  echo 'export JAVA_HOME="/Applications/Android Studio.app/Contents/jbr/Contents/Home"' >> ~/.zshrc
  echo 'export ANDROID_HOME="$HOME/Library/Android/sdk"' >> ~/.zshrc
  echo 'export NDK_HOME="$ANDROID_HOME/ndk/29.0.13113456"' >> ~/.zshrc
  echo 'export PATH="$PATH:$ANDROID_HOME/emulator"' >> ~/.zshrc
  source ~/.zshrc
  ```

## Create New App

```
dx new my-app
```

### Workspace

If you choose a new workspace project it may contain a workspace member for each platform. With the router feature, each platform crate will have a `views` folder for your platform-specific views. You are provided with a `ui` crate for shared UI and if you chose to use fullstack, you will have a `server` crate for your shared server functions.

## Build

If you are using M1, you will have to run `cargo build --target x86_64-apple-ios` instead of `cargo apple build` if you want to run in simulator.

```
rustup target add x86_64-apple-ios
cargo build --target x86_64-apple-ios
```

## Serving

### Web

Navigate to the platform crate of your choice:
```bash
cd web
```

Serve to start developing.
If using Tailwind, make sure to run the Tailwind CLI.

```bash
dx serve \
  --package web \
  --platform web \
  --open
```

### Android

* Note: It is necessary to add a device via Android Studio > Device Manager. e.g. Google Pixel API Level 28 (Pie). It would then be available in $HOME/.android/avd/Pixel_6_API_Baklava.avd. Note: Pixel_API_28 is alternative. Restart the emulator. Note: This uses qemu-system-aarch64.

```sh
emulator -avd Pixel_6_API_Baklava -netdelay none -netspeed full
```

### iOS

Boot a virtual device:

```sh
open /Applications/Xcode.app/Contents/Developer/Applications/Simulator.app
```
> Alternative that may not work `xcrun simctl boot "iPhone 15 Pro Max"`

Run the app:
```sh
dx serve --package mobile --platform ios --open
```

## Troubleshooting

* If you get error `ERROR err=Other(Failed to bind server to: 0.0.0.0:8080, is there another devserver running?`
  * Force exit 
    ```
    kill $(lsof -t -i:8080)
    sudo pkill -9 "Simulator"
    ```

* If you get error `zsh: command not found: emulator` try running:
  ```
  source ~/.zshrc
  ```

## Resources

* Examples - https://dioxuslabs.com/learn/0.6/#
