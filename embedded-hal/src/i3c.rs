use crate::i2c::{self, SevenBitAddress};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[non_exhaustive]
pub enum ErrorKind {
    Nack,
    Other,
}

impl Error for i2c::Error {
    fn i3c_kind(&self) -> ErrorKind;
}

pub trait ErrorType {
    type Error: Error;
}

impl<T: ErrorType + ?Sized> ErrorType for &mut T {
    type Error = T::Error;
}

pub enum Transfer<'a, 'b> {
    I2c(&'a mut [i2c::Operation<'b>]),
    Sdr(&'a mut [i2c::Operation<'b>]),
    HdrDdr(&'a mut [DdrOperation<'b>]),
}

pub enum DdrOperation<'a> {
    Read { command: u8, buf: &'a mut [u8] }, //TODO might be u16 for buffer
    Write { command: u8, buf: &'a [u8] },
}
