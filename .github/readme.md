# Rust for Windows App SDK Template

The `windows-app-rs` [template](https://github.com/cargo-generate/cargo-generate) creates a [Windows App SDK](https://github.com/microsoft/WindowsAppSDK) (formerly known as Project Reunion) project. It is powered by the [windows](https://github.com/microsoft/windows-rs) crate.

It's early days, but the `windows-app-rs` template is meant to make it much easier to use the Windows App SDK from Rust. As this new set of APIs requires bootstrapping and various other hooks to get it up and running, using only the `windows` crate—while possible—is a little more cumbersome.

So while the `windows` crate is still essential as it provides all of the language support, the `windows-app-rs` template will provide the necessary bootstrapping unique to the Windows App SDK.

## Usage

1. `cargo install cargo-generate`
2. `cargo generate --name my-project https://github.com/microsoft/windows-app-rs`
3. `cd my-project`
4. `cargo build`

## Preview & Experimental releases

The Windows App SDK is [available in 3 release channels](https://docs.microsoft.com/en-us/windows/apps/windows-app-sdk/release-channels). Those channels are represented via branches in this repository, enabling you to generate projects targeting a specific channel via the branch (`-b`) parameter (e.g. `cargo generate ... -b preview`).

## Add Favorites

`cargo-generate` supports the creation of _favorites_, further simplifying project creation. To add `windows-app-rs` favorites, do the following:

1. Create `%USERPROFILE%\.cargo\cargo-generate` (or `$CARGO_HOME/cargo-generate`)
2. Add the following lines and save:

    ```toml
    [favorites.was]
    description = "Rust for Windows App SDK Template (Stable channel)"
    git = "https://github.com/microsoft/windows-app-rs"
    branch = "stable"

    [favorites.was-pre]
    description = "Rust for Windows App SDK Template (Preview channel)"
    git = "https://github.com/microsoft/windows-app-rs"
    branch = "preview"

    [favorites.was-exp]
    description = "Rust for Windows App SDK Template (Experimental channel)"
    git = "https://github.com/microsoft/windows-app-rs"
    branch = "experimental"
    ```
3. Try it out!
   
    ```bash
    cargo generate was --name my-project
    ```