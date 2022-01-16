use super::commands::{self, AtWrite, ConnectionResult};
use crate::{AtModem, Error};
use log::*;

pub struct GnssClient {
    read_timeout_ms: u32,
    write_timeout_ms: u32,
}

impl Default for GnssClient {
    fn default() -> Self {
        Self {
            read_timeout_ms: 2000,
            write_timeout_ms: 5000,
        }
    }
}

impl GnssClient {
    pub fn set_read_timeout(&mut self, timeout_ms: u32) {
        self.read_timeout_ms = timeout_ms;
    }

    pub fn read_timeout(&self) -> Option<u32> {
        Some(self.read_timeout_ms)
    }

    pub fn set_write_timeout(&mut self, timeout_ms: u32) {
        self.write_timeout_ms = timeout_ms;
    }

    pub fn write_timeout(&self) -> Option<u32> {
        Some(self.write_timeout_ms)
    }

    pub fn gnss_start<T>(
        &mut self,
        modem: &mut T,
        timeout: Option<u32>,
    ) -> Result<(), Error<T::SerialError>>
    where
        T: AtModem,
    {
        let result = commands::Cgnspwr.write(
            commands::PowerStatus::On,
            modem,
            timeout.unwrap_or(self.write_timeout_ms))?;
        Ok(())
    }
}
