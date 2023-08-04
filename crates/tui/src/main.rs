mod commands;
mod sway_output;

use crate::commands::Command;

use cursive::views::{CircularFocus};
use cursive::{
    traits::*,
    views::{Dialog},
    Cursive, CursiveRunnable,
};

fn main() -> anyhow::Result<()> {
    let mut tui = create_tui();
    tui.run();
    Ok(())
}

fn create_tui() -> CursiveRunnable {
    let mut siv = cursive::default();
    siv.load_toml(include_str!("../assets/theme.toml")).unwrap();
    siv.add_layer(main_layer());

    siv
}

fn main_layer() -> CircularFocus<Dialog> {
    Dialog::new()
        .title("Welcome back Okno")
        .button("Quit", |s| s.quit())
        .button("Display", show_displays)
        .button("Steam Mode", move |s| {
            Command::EnableSteamMode.run().unwrap();
            s.quit();
        })
        .button("Desktop Mode", |_clicked| {})
        .wrap_with(CircularFocus::new)
        .wrap_tab()
}

fn show_displays(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::text("Select a profile")
            .button("Back", |s| {
                s.pop_layer();
                s.add_layer(main_layer())
            })
            .title("Profile")
            .button("Desktop", |s| {
                Command::DesktopOnly.run().unwrap();
                s.quit()
            })
            .button("TV", |s| {
                Command::TvOnly.run().unwrap();
                s.quit()
            })
            .button("Hybrid", |s| {
                Command::TvAndDesktop.run().unwrap();
                s.quit()
            }),
    );
}
