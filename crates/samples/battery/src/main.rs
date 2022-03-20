use ::windows_app::Microsoft::Windows::System::Power::*;
use ::windows_app::*;

fn main() -> ::windows::core::Result<()> {
    bootstrap::initialize()?;
    let charge = PowerManager::RemainingChargePercent()?;
    println!("Remaining charge: {charge}%");
    bootstrap::uninitialize()
}
