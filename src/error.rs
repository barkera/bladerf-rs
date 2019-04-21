use std::ffi::NulError;

pub enum Error {
    Nul(NulError),
    Unknown(i32),
    Unexpected,
    Range,
    Inval,
    Mem,
    IO,
    Timeout,
    NoDev,
    Unsupported,
    Misaligned,
    Checksum,
    NoFile,
    UpdateFPGA,
    UpdateFW,
    TimePast,
    QueueFull,
    FPGAOp,
    Permission,
    WouldBlock,
    NotInit,
}

impl From<i32> for Error {
    fn from(rc: i32) -> Self {
        match rc {
            -1 => Error::Unexpected,
            -2 => Error::Range,
            -3 => Error::Inval,
            -4 => Error::Mem,
            -5 => Error::IO,
            -6 => Error::Timeout,
            -7 => Error::NoDev,
            -8 => Error::Unsupported,
            -9 => Error::Misaligned,
            -10 => Error::Checksum,
            -11 => Error::NoFile,
            -12 => Error::UpdateFPGA,
            -13 => Error::UpdateFW,
            -14 => Error::TimePast,
            -15 => Error::QueueFull,
            -16 => Error::FPGAOp,
            -17 => Error::Permission,
            -18 => Error::WouldBlock,
            -19 => Error::NotInit,
            _ => Error::Unknown(rc),
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", match self {
            Error::Unexpected => "An unexpected failure occurred".to_string(),
            Error::Range => "Provided parameter is out of range".to_string(),
            Error::Inval => "Invalid operation/parameter".to_string(),
            Error::Mem => "Memory allocation error".to_string(),
            Error::IO => "File/Device I/O error".to_string(),
            Error::Timeout => "Operation timed out".to_string(),
            Error::NoDev => "No device(s) available".to_string(),
            Error::Unsupported => "Operation not supported".to_string(),
            Error::Misaligned => "Misaligned flash access".to_string(),
            Error::Checksum => "Invalid checksum".to_string(),
            Error::NoFile => "File not found".to_string(),
            Error::UpdateFPGA => "An FPGA update is required".to_string(),
            Error::UpdateFW => "A firmware update is requied".to_string(),
            Error::TimePast => "Requested timestamp is in the past".to_string(),
            Error::QueueFull => "Could not enqueue data into full queue".to_string(),
            Error::FPGAOp => "An FPGA operation reported failure".to_string(),
            Error::Permission => "Insufficient permissions for the requested operation".to_string(),
            Error::WouldBlock => "Operation would block.to_string(), but has been requested to be non-blocking".to_string(),
            Error::NotInit => "Device insufficiently initialized for operation".to_string(),
            Error::Unknown(num)=> format!("An unknown error occurred ({})", num),
            Error::Nul(e) => format!("{:?}", e),
        })

        // write!(fmt, "{}", s)
    }
}

impl From<NulError> for Error {
    fn from(e: NulError) -> Self {
        Error::Nul(e)
    }
}
