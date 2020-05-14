//! A pure rust implementation bound to the C version of minilzo.
//!
//! Example
//!
//! ```rust
//! // test compress
//! let mut lzo = minilzo_rs::LZO::init().unwrap();
//! let input = [0x00u8; 1024];
//! let out = lzo.compress(&input).unwrap();
//!
//! //test decompress
//! let input = lzo.decompress_safe(&out[..], 1024);
//! let input = input.unwrap();
//! assert_eq!(input.len(), 1024);
//! ```
//!
mod minilzo;
use std::mem::{size_of, MaybeUninit};
use std::os::raw::{c_int, c_long, c_short, c_uint};

type LZOResult<T> = Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    Error,
    OutOfMemory,
    NotCompressible,
    InputOverrun,
    OutputOverrun,
    LookbehindOverrun,
    EOFNotFound,
    InputNotConsumed,
    NotYetImplemented,
    InvalidArgument,
    InvalidAlignment,
    OutputNotConsumed,
    InternalError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::OutOfMemory => "out of memory",
            Error::NotCompressible => "not compressible",
            Error::InputOverrun => "input overrun",
            Error::OutputOverrun => "output overrun",
            Error::LookbehindOverrun => "lookbehind overrun",
            Error::EOFNotFound => "EOF not found",
            Error::InputNotConsumed => "input not consumed",
            Error::NotYetImplemented => "not yet implemented",
            Error::InvalidArgument => "invalid argument",
            Error::InvalidAlignment => "invalid alignment",
            Error::OutputNotConsumed => "output not consumed",
            Error::InternalError => "internal error",
            Error::Error => "error",
        }
    }
}

fn lzo_err_code_to_result<T>(code: i32, value: T) -> LZOResult<T> {
    let error = match code {
        0 => return Ok(value),
        -1 => Error::Error,
        -2 => Error::OutOfMemory,
        -3 => Error::NotCompressible,
        -4 => Error::InputOverrun,
        -5 => Error::OutputOverrun,
        -6 => Error::LookbehindOverrun,
        -7 => Error::EOFNotFound,
        -8 => Error::InputNotConsumed,
        -9 => Error::NotYetImplemented,
        -10 => Error::InvalidArgument,
        -11 => Error::InvalidArgument,
        -12 => Error::OutputNotConsumed,
        -99 => Error::InternalError,
        _ => Error::Error,
    };
    Err(error)
}

/// An example of LZO compression.
///
/// Example
///
/// ```rust
/// // test compress
/// let mut lzo = minilzo_rs::LZO::init().unwrap();
/// let input = [0x00u8; 1024];
/// let out = lzo.compress(&input).unwrap();
///
/// //test decompress
/// let input = lzo.decompress_safe(&out[..], 1024);
/// let input = input.unwrap();
/// assert_eq!(input.len(), 1024);
/// ```
pub struct LZO {
    wrkmem: [u8; minilzo::LZO1X_1_MEM_COMPRESS],
}

impl LZO {
    /// Initializing an LZO instance.
    pub fn init() -> LZOResult<Self> {
        match Self::lzo_init() {
            Ok(_) => Ok(LZO {
                wrkmem: unsafe { MaybeUninit::uninit().assume_init() },
            }),
            Err(e) => Err(e),
        }
    }

    fn lzo_init() -> LZOResult<()> {
        let code = unsafe {
            minilzo::__lzo_init_v2(
                minilzo::LZO_VERSION as c_uint,
                size_of::<c_short>() as c_int,
                size_of::<c_int>() as c_int,
                size_of::<c_long>() as c_int,
                size_of::<u32>() as c_int,
                size_of::<minilzo::lzo_uint>() as c_int,
                size_of::<usize>() as c_int,
                size_of::<usize>() as c_int,
                size_of::<usize>() as c_int,
                size_of::<minilzo::lzo_callback_t>() as c_int,
            )
        };
        lzo_err_code_to_result(code, ())
    }

    /// Compress the src data and return an error if it fails.
    pub fn compress(&mut self, src: &[u8]) -> LZOResult<Vec<u8>> {
        let mut out_len = (src.len() + src.len() / 16 + 64 + 3) as u64;
        let mut out: Vec<u8> = vec![0u8; out_len as usize];
        let code = unsafe {
            minilzo::lzo1x_1_compress(
                src.as_ptr(),
                src.len() as u64,
                out.as_mut_ptr(),
                &mut out_len,
                self.wrkmem.as_mut_ptr() as *mut _,
            )
        };
        out.resize(out_len as usize, 0);
        lzo_err_code_to_result(code, out)
    }

    /// Decompress data.
    pub fn decompress(&self, src: &[u8], dst_len: usize) -> LZOResult<Vec<u8>> {
        let mut dst = vec![0u8; dst_len];
        let code = unsafe {
            minilzo::lzo1x_decompress(
                src.as_ptr(),
                src.len() as u64,
                dst.as_mut_ptr(),
                &dst_len as *const _ as *mut _,
                std::ptr::null_mut(),
            )
        };

        if code == 0 && dst.len() < dst_len as usize {
            dst.resize(dst_len as usize, 0);
        }
        lzo_err_code_to_result(code, dst)
    }

    /// safe decompression with overrun testing.
    pub fn decompress_safe(&self, src: &[u8], dst_len: usize) -> LZOResult<Vec<u8>> {
        let mut dst = vec![0u8; dst_len];
        let code = unsafe {
            minilzo::lzo1x_decompress_safe(
                src.as_ptr(),
                src.len() as minilzo::lzo_uint,
                dst.as_mut_ptr(),
                &dst_len as *const _ as *mut _,
                std::ptr::null_mut(),
            )
        };

        if code == 0 && dst.len() < dst_len as usize {
            dst.resize(dst_len as usize, 0);
        }
        lzo_err_code_to_result(code, dst)
    }
}

/// Calculate the adler32 value of the data.
///
/// Example
///
/// ```rust
/// let buff = [0x09u8; 1024];
/// let checksum = minilzo_rs::adler32(&buff[..]);
/// assert_eq!(checksum, 439886849);
/// ```
pub fn adler32(buf: &[u8]) -> u32 {
    let checksum = 1u32;
    let checksum = unsafe { minilzo::lzo_adler32(checksum, buf.as_ptr(), buf.len() as u64) };
    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lzo_cmpress() {
        // test compress
        let mut lzo = LZO::init().unwrap();
        let input = [0x00u8; 1024];
        let out = lzo.compress(&input).unwrap();

        // test decompress
        let input = lzo.decompress_safe(&out[..], 1024);
        let input = input.unwrap();
        assert_eq!(input.len(), 1024);
    }

    #[test]
    fn test_adler32() {
        let buff = [0x09u8; 1024];
        let checksum = adler32(&buff);
        assert_eq!(checksum, 439886849);
    }
}
