use swayipc::Connection;

pub enum Screen {
    Desktop,
    TV,
}

impl Screen {
    pub fn place(&self, conn: &mut Connection) -> anyhow::Result<()> {
        conn.run_command(format!(
            "output {} pos {} res {}@{}",
            self.sway_id(),
            self.position(),
            self.resolution(),
            self.refresh_rate()
        ))?;
        Ok(())
    }

    pub fn enable(&self, conn: &mut Connection) -> anyhow::Result<()> {
        conn.run_command(format!("output {} enable", self.sway_id()))?;
        Ok(())
    }

    pub fn disable(&self, conn: &mut Connection) -> anyhow::Result<()> {
        conn.run_command(format!("output {} disable", self.sway_id()))?;
        Ok(())
    }

    fn sway_id(&self) -> &str {
        match self {
            Screen::Desktop => "DP-2",
            Screen::TV => "HDMI-A-1",
        }
    }

    fn resolution(&self) -> &str {
        match self {
            Screen::Desktop => "3440x1440",
            Screen::TV => "1920x1080",
        }
    }

    fn position(&self) -> &str {
        match self {
            Screen::Desktop => "1920 0",
            Screen::TV => "0 0",
        }
    }

    fn refresh_rate(&self) -> &str {
        "59.999Hz"
    }
}

#[cfg(test)]
mod test {
    use crate::sway_output::Screen;
    use swayipc::Connection;

    #[test]
    fn enable_tv() {
        let mut conn = Connection::new().unwrap();
        assert!(Screen::TV.enable(&mut conn).is_ok());
        assert!(Screen::TV.place(&mut conn).is_ok());
    }

    #[test]
    fn enable_desktop() {
        let mut conn = Connection::new().unwrap();
        assert!(Screen::Desktop.enable(&mut conn).is_ok());
        assert!(Screen::Desktop.place(&mut conn).is_ok());
    }
}
