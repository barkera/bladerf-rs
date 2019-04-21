#[derive(Debug)]
pub enum Error {
    Unknown,
    Unexpected = -1,
    Range = -2,
    Inval = -3,
    Mem = -4,
    IO = -5,
    Timeout = -6,
    NoDev = -7,
    Unsupported = -8,
    Misaligned = -9,
    Checksum = -10,
    NoFile = -11,
    UpdateFPGA = -12,
    UpdateFW = -13,
    TimePast = -14,
    QueueFull = -15,
    FPGAOp = -16,
    Permission = -17,
    WouldBlock = -18,
    NotInit = -19,
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
            _ => Error::Unknown,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Error::Unexpected => "An unexpected failure occurred",
            Error::Range => "Provided parameter is out of range",
            Error::Inval => "Invalid operation/parameter",
            Error::Mem => "Memory allocation error",
            Error::IO => "File/Device I/O error",
            Error::Timeout => "Operation timed out",
            Error::NoDev => "No device(s) available",
            Error::Unsupported => "Operation not supported",
            Error::Misaligned => "Misaligned flash access",
            Error::Checksum => "Invalid checksum",
            Error::NoFile => "File not found",
            Error::UpdateFPGA => "An FPGA update is required",
            Error::UpdateFW => "A firmware update is requied",
            Error::TimePast => "Requested timestamp is in the past",
            Error::QueueFull => "Could not enqueue data into full queue",
            Error::FPGAOp => "An FPGA operation reported failure",
            Error::Permission => "Insufficient permissions for the requested operation",
            Error::WouldBlock => "Operation would block, but has been requested to be non-blocking",
            Error::NotInit => "Device insufficiently initialized for operation",
            Error::Unknown => "An unknown error occurred",
        };

        write!(fmt, "{}", s)
    }
}
