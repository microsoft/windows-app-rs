[![Build and Test](https://github.com/microsoft/windows-app-rs/workflows/Build%20and%20Test/badge.svg?event=push)](https://github.com/microsoft/windows-app-rs/actions)

## Rust template for the Windows App SDK

This template makes the [Windows App SDK](https://github.com/microsoft/WindowsAppSDK) (formerly known as Project Reunion) available to Rust developers. It is powered by the [windows](https://github.com/microsoft/windows-rs) crate.

It's early days, but the template is meant to make it much easier to use the Windows App SDK from Rust. As this new set of APIs requires bootstrapping and various other hooks to get it up and running, using only the `windows` crate—while possible—is a little more cumbersome for these new APIs.

So while the `windows` crate is still essential as it provides all of the language support, this template will provide the necessary bootstrapping unique to the Windows App SDK.

As [WinUI](https://microsoft.github.io/microsoft-ui-xaml/) is a large part of the Windows App SDK, one goal is to also support the latest WinUI app development.
