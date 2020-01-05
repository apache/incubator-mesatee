#![cfg_attr(feature = "mesalock_sgx", no_std)]
#[cfg(feature = "mesalock_sgx")]
#[macro_use]
extern crate sgx_tstd as std;

use anyhow::{self, Result};
use teaclave_service_config as config;
pub use teaclave_service_enclave_utils_proc_macro::teaclave_service;

use log::debug;
use log::error;
use std::backtrace;

pub struct ServiceEnclave;

impl ServiceEnclave {
    pub fn init(name: &str) -> Result<()> {
        env_logger::init();

        debug!("Enclave initializing");

        if backtrace::enable_backtrace(format!("{}.signed.so", name), backtrace::PrintFormat::Full)
            .is_err()
        {
            error!("Cannot enable backtrace");
            return Err(anyhow::anyhow!("ecall error"));
        }
        if !config::is_runtime_config_initialized() {
            error!("Runtime config is not initialized");
            return Err(anyhow::anyhow!("ecall error"));
        }

        Ok(())
    }

    pub fn finalize() -> Result<()> {
        debug!("Enclave finalizing...");

        #[cfg(feature = "cov")]
        sgx_cov::cov_writeout();

        Ok(())
    }
}
