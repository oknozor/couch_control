use crate::sway_output::Screen;
use cursive::reexports::log::error;
use std::os::unix::prelude::CommandExt;
use swayipc::Connection;

pub enum Command {
    TvOnly,
    DesktopOnly,
    TvAndDesktop,
    EnableSteamMode,
}

impl Command {
    pub fn run(&self) -> anyhow::Result<()> {
        let conn = &mut Connection::new()?;
        match self {
            Command::TvOnly => {
                Screen::TV.place(conn)?;
                Screen::TV.enable(conn)?;
                Screen::Desktop.disable(conn)?;
            }
            Command::DesktopOnly => {
                Screen::Desktop.place(conn)?;
                Screen::Desktop.enable(conn)?;
                Screen::TV.disable(conn)?;
            }
            Command::TvAndDesktop => {
                Screen::TV.place(conn)?;
                Screen::TV.enable(conn)?;
                Screen::Desktop.place(conn)?;
                Screen::Desktop.enable(conn)?;
            }
            Command::EnableSteamMode => {
                Command::TvOnly.run()?;
                conn.run_command("workspace 8")?;
                let err = std::process::Command::new("steam")
                    .arg("steam://open/bigpicture")
                    .exec();

                error!("Failed to launch steam: {err}");
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use cursive::reexports::log::error;
    use std::os::unix::prelude::CommandExt;

    #[test]
    fn run_steam() {
        let err = std::process::Command::new("steam")
            .arg("steam://open/bigpicture")
            .exec();

        error!("Failed to launch steam: {err}");
    }
}
