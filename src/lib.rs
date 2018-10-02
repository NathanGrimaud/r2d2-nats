pub extern crate nats;
pub extern crate r2d2;

use nats::{Client, NatsError};
use std::error;
use std::error::Error as _StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Other(NatsError),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.cause() {
            Some(cause) => write!(fmt, "{}: {}", self.description(), cause),
            None => write!(fmt, "{}", self.description()),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Other(ref err) => err.description(),
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Other(ref err) => err.cause(),
        }
    }
}

#[derive(Debug)]
pub struct NatsConnectionManager {
    params: String,
}

impl NatsConnectionManager {
    pub fn new(connection_string: String) -> Result<NatsConnectionManager, NatsError> {
        Ok(NatsConnectionManager {
            params: connection_string,
        })
    }
}

impl r2d2::ManageConnection for NatsConnectionManager {
    type Connection = Client;
    type Error = Error;

    fn connect(&self) -> Result<Client, Error> {
        match Client::new(self.params.to_owned()) {
            Ok(client) => Ok(client),
            Err(err) => Err(Error::Other(err)),
        }
    }

    fn is_valid(&self, conn: &mut Client) -> Result<(), Error> {
        match conn.publish("r2d2_nats", "PING".as_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::Other(err)),
        }
    }

    fn has_broken(&self, _conn: &mut Client) -> bool {
        false
    }
}
