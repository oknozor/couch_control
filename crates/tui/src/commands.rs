use crate::sway_output::Screen;
use cursive::reexports::log::{error, info};
use nix::sys::wait::waitpid;
use std::os::unix::prelude::CommandExt;
use std::process::exit;
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
                unsafe { run_steam_and_exit() }
            }
        }

        Ok(())
    }
}

unsafe fn run_steam_and_exit() {
    use nix::unistd::{fork, ForkResult};

    match fork() {
        Ok(ForkResult::Parent { child, .. }) => {
            info!("Exiting");
            waitpid(child, None).unwrap();
            exit(0)
        }
        Ok(ForkResult::Child) => {
            let err = std::process::Command::new("steam")
                .arg("steam://open/bigpicture")
                .exec();

            error!("Failed to launch steam {}", err);
            exit(1);
        }
        Err(_) => error!("Forking process to launch steam failed"),
    }
}

#[test]
fn test() {
    unsafe { run_steam_and_exit() }
}
