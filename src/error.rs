pub enum Error<I2C: embedded_hal_async::i2c::I2c> {
    MeasurementTimeout,
    I2CError(I2C::Error),
}

impl<I2C_ERROR: embedded_hal_async::i2c::Error> From<I2C_ERROR> for Error<I2C_ERROR> {
    fn from(error: <I2C as embedded_hal_async::i2c::I2c>::Error) -> Self {
        Self::I2CError(error)
    }
}
