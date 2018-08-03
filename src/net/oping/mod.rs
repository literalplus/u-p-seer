use super::api::*;
use std::sync::mpsc::Sender;
use std::time::Duration;
use oping::{Ping as OPing, PingItem as OPingItem, PingError as OPingError};
use floating_duration::TimeAsFloat;
use std::error::Error;
use std::boxed::Box;
use std::mem;

#[derive(Default)]
pub struct OPingBackend {
    cfg: PingConfig,
    ping: Option<OPing>,
}

impl PingBackend for OPingBackend {
    fn new(cfg: PingConfig) -> Self {
        OPingBackend {
            cfg,
            ..Default::default()
        }
    }

    fn prepare(&mut self) -> Result<(), PingError> {
        self.ping = self.mkping()?;
        Ok(())
    }

    fn send(&mut self, tx: Sender<PingResult>) {
        let ping = self.mkping().unwrap();
        let my_ping = mem::replace(&mut self.ping, ping);
        let send_res = my_ping.unwrap().send();
        match send_res {
            Ok(responses) => {
                for response in responses {
                    tx.send(self.item_to_res(response)).ok();
                }
            }
            Err(err) => {
                tx.send(Err(PingError {
                    error_type: PingErrorType::Other,
                    message: err.description().into(),
                    address: None,
                })).ok();
            }
        };
    }
}

impl OPingBackend {
    fn mkping(&self) -> Result<Option<OPing>, OPingError> {
        let mut ping = OPing::new();
        ping.set_timeout(self.cfg.timeout.as_fractional_secs())?;
        for ref addr in &self.cfg.addrs {
            ping.add_host(&addr)?;
        }
        Ok(Some(ping))
    }

    fn item_to_res(&self, item: OPingItem) -> PingResult {
        match item.dropped {
            0 => Ok(Box::new(item)),
            _ => Err(PingError {
                error_type: PingErrorType::Timeout,
                message: "timed out".into(),
                address: Some(item.get_request_address().to_string()),
            })
        }
    }
}

impl PingResponse for OPingItem {
    fn get_latency(&self) -> Duration {
        let latency_micros = self.latency_ms * 1000.0;
        return Duration::from_micros(latency_micros as u64);
    }

    fn get_request_address(&self) -> &str {
        return &self.address;
    }
}

impl From<OPingError> for PingError {
    fn from(src: OPingError) -> Self {
        return PingError {
            error_type: PingErrorType::Other,
            message: src.description().into(),
            address: None,
        }
    }
}