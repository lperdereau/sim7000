use crate::Error;

use super::{AtCommand, AtDecode, AtExecute};

pub struct Cgnscpy;

impl AtCommand for Cgnscpy {
    const COMMAND: &'static str = "AT+CGNSCPY";
}

impl AtExecute for Cgnscpy {
    type Output = CopyResult;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CopyResult {
    Success = 0,
    FileMissing = 1,
}

impl AtDecode for CopyResult {
    fn decode<B: crate::SerialReadTimeout>(
        decoder: &mut super::Decoder<B>,
        timeout: embedded_time::duration::Milliseconds,
    ) -> Result<Self, Error<B::SerialError>> {
        decoder.expect_str("+CGNSCPY: ", timeout)?;

        let result = match decoder.decode_scalar(timeout)? {
            0 => CopyResult::Success,
            1 => CopyResult::FileMissing,
            _ => return Err(Error::DecodingFailed),
        };

        decoder.end_line();
        decoder.expect_str("OK", timeout)?;

        Ok(result)
    }
}
