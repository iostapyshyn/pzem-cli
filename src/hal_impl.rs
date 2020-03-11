extern crate embedded_hal as hal;
extern crate nb;
extern crate void;

extern crate serialport;

pub struct Uart {
    pub port: Box<dyn serialport::SerialPort>,
}

impl hal::serial::Write<u8> for Uart {
    type Error = std::io::Error;
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        let buf = [word];
        match self.port.write(&buf) {
            Ok(1) => Ok(()),
            Ok(0) => Err(nb::Error::WouldBlock),
            Ok(_) => unreachable!(),
            Err(e) => {
                if e.kind() == std::io::ErrorKind::TimedOut {
                    Err(nb::Error::WouldBlock)
                } else {
                    Err(nb::Error::Other(e))
                }
            }
        }
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        self.port.flush().map_err(nb::Error::Other)
    }
}

impl hal::serial::Read<u8> for Uart {
    type Error = std::io::Error;
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        let mut buf = [0 as u8];
        match self.port.read(&mut buf) {
            Ok(1) => Ok(buf[0]),
            Ok(0) => Err(nb::Error::WouldBlock),
            Ok(_) => unreachable!(),
            Err(e) => {
                if e.kind() == std::io::ErrorKind::TimedOut {
                    Err(nb::Error::WouldBlock)
                } else {
                    Err(nb::Error::Other(e))
                }
            }
        }
    }
}

pub struct Timer {
    end: std::time::Instant,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            end: unsafe { std::mem::MaybeUninit::uninit().assume_init() },
        }
    }
}

impl hal::timer::CountDown for Timer {
    type Time = std::time::Duration;

    fn start<T: Into<Self::Time>>(&mut self, count: T) {
        let now = std::time::Instant::now();
        self.end = now + count.into();
    }

    fn wait(&mut self) -> nb::Result<(), void::Void> {
        if std::time::Instant::now() < self.end {
            Err(nb::Error::WouldBlock)
        } else {
            Ok(())
        }
    }
}
