use ::windows::core::*;
use ::windows_app::*;
use windows_app::Microsoft::UI::{ColorHelper, Colors};

fn main() -> Result<()> {
    bootstrap::initialize()?;
    sample_main()?;
    bootstrap::uninitialize()
}

fn sample_main() -> Result<()> {
    println!("A peculiar sampling of colors follows... ");

    let color = Colors::SkyBlue()?;
    println!(
        "\x1b[38;2;{};{};{}m Sky Blue \x1b[0m",
        color.R, color.G, color.B
    );

    let color = Colors::SeaGreen()?;
    println!(
        "\x1b[38;2;{};{};{}m Sea Green \x1b[0m",
        color.R, color.G, color.B
    );

    let color = Colors::SeaShell()?;
    println!(
        "\x1b[38;2;{};{};{}m Sea Shell \x1b[0m",
        color.R, color.G, color.B
    );

    let color = ColorHelper::FromArgb(255, 247, 76, 0)?;
    println!(
        "\x1b[38;2;{};{};{}m Ferris 🦀 \x1b[0m",
        color.R, color.G, color.B
    );

    Ok(())
}
