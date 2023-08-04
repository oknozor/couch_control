mod commands;
mod sway_output;

use crate::commands::Command;
use cursive::view::Margins;
use cursive::views::Button;
use cursive::{
    traits::*,
    views::{Dialog, ListView},
    CursiveRunnable,
};

fn main() -> anyhow::Result<()> {
    let mut tui = create_tui();
    tui.run();
    Ok(())
}

fn create_tui() -> CursiveRunnable {
    let mut siv = cursive::default();

    let mut dialog = Dialog::new()
        .title("Welcome back Okno")
        .button("Quit", |s| s.quit())
        .content(
            ListView::new()
                // Each child is a single-line view with a label
                .child("", Button::new("Audio", |_s| {}).fixed_width(10))
                .child("", Button::new("Display", |_s| {}).fixed_width(10))
                .child(
                    "",
                    Button::new("Kodi Mode", |_s| {
                        // TODO
                    })
                    .fixed_width(10),
                )
                .child(
                    "",
                    Button::new("Steam Mode", move |s| {
                        Command::EnableSteamMode.run().unwrap();
                        s.quit();
                    })
                    .fixed_width(10),
                )
                .child(
                    "",
                    Button::new("Desktop Mode", |_clicked| {}).fixed_width(10),
                ),
        );

    dialog.set_padding(Margins::lrtb(3, 3, 3, 3));

    siv.add_layer(dialog);

    siv
}
