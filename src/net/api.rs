use std::sync::mpsc::Sender;
use std::time::Duration;
use std::error;
use std::fmt;

#[derive(Default)]
pub struct PingConfig {
    pub addrs: Vec<String>,
    pub timeout: Duration,
}

pub type PingResult = Result<Box<PingResponse>, PingError>;

pub trait PingBackend {
    fn new(cfg: PingConfig) -> Self;

    fn prepare(&mut self) -> Result<(), PingError>;

    fn send(&mut self, tx: Sender<PingResult>);
}

pub trait PingResponse {
    fn get_latency(&self) -> Duration;
    fn get_request_address(&self) -> &str;
}

#[derive(Debug)]
pub enum PingErrorType {
    Connect,
    Receive,
    Timeout,
    Other,
}

#[derive(Debug)]
pub struct PingError {
    pub error_type: PingErrorType,
    pub message: String,
    pub address: Option<String>,
}

impl error::Error for PingError {
    fn description(&self) -> &str {
        return &self.message;
    }
}

impl fmt::Display for PingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "PingError/{:?}: {} (for {})",
               &self.error_type, &self.message, &self.address.clone().unwrap_or("???".into()))
    }
}