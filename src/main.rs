mod gui;

use tempo_core::*;
use crate::gui::run_gui;

#[async_std::main]
async fn main() -> iced::Result {
    let opts = run_cli().await;
    match opts {
        Some(flags) => run_gui(flags),
        None => Ok(())
    }
}
