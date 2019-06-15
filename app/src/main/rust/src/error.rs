use crate::android_audio::SlError;
use std::borrow::Cow;
use std::fmt;
use std::net::AddrParseError;
use std::sync::PoisonError;

#[derive(Debug)]
pub struct Error {
    pub repr: Box<ErrorRepr>,
}

impl Error {
    #[allow(dead_code)]
    pub fn new_wrong_argument(description: String) -> Self {
        Error {
            repr: Box::new(ErrorRepr::WrongArgument(description)),
        }
    }

    #[allow(dead_code)]
    pub fn new_wrong_state(description: String) -> Self {
        Error {
            repr: Box::new(ErrorRepr::WrongState(description)),
        }
    }

    pub fn new_null_ptr(description: String) -> Self {
        Error {
            repr: Box::new(ErrorRepr::NullPointer(description)),
        }
    }

    #[allow(dead_code)]
    pub fn new_io(e: std::io::Error, f_name: String) -> Self {
        Error {
            repr: Box::new(ErrorRepr::Io((e, f_name))),
        }
    }

    pub fn new_net_parse<A>(e: AddrParseError, addr: A) -> Self
    where
        A: Into<Cow<'static, str>>,
    {
        Error {
            repr: Box::new(ErrorRepr::NetParse((e, addr.into()))),
        }
    }
}

#[derive(Debug)]
pub enum ErrorRepr {
    WrongArgument(String),
    WrongState(String),
    NullPointer(String),
    Io((std::io::Error, String)),
    SlError(SlError),
    NetParse((AddrParseError, Cow<'static, str>)),
    LockPoison(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self.repr {
            ErrorRepr::WrongArgument(s) => write!(f, "Wrong argument: {}", s),
            ErrorRepr::WrongState(s) => write!(f, "Wrong state: {}", s),
            ErrorRepr::NullPointer(s) => write!(f, "Null Pointer: {}", s),
            ErrorRepr::Io((e, f_name)) => {
                write!(f, "{}", e)?;
                if !f_name.is_empty() {
                    write!(f, ". File name: {}", f_name)?;
                }
                Ok(())
            }
            ErrorRepr::SlError(e) => e.fmt(f),
            ErrorRepr::NetParse((e, addr)) => write!(f, "{} of {}", e, addr),
            ErrorRepr::LockPoison(descr) => write!(f, "{}", descr),
        }
    }
}

impl ::std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error {
            repr: Box::new(ErrorRepr::Io((e, String::new()))),
        }
    }
}
impl From<SlError> for Error {
    fn from(e: SlError) -> Self {
        Error {
            repr: Box::new(ErrorRepr::SlError(e)),
        }
    }
}
impl<T> From<PoisonError<T>> for Error {
    fn from(e: PoisonError<T>) -> Self {
        Error {
            repr: Box::new(ErrorRepr::LockPoison(format!("{}", e))),
        }
    }
}
