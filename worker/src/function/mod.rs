// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

#[cfg(feature = "mesalock_sgx")]
use std::prelude::v1::*;

use crate::runtime::TeaclaveRuntime;
use anyhow;
use teaclave_types::TeaclaveFunctionArguments;

pub trait TeaclaveFunction {
    fn execute(
        &self,
        runtime: Box<dyn TeaclaveRuntime + Send + Sync>,
        args: TeaclaveFunctionArguments,
    ) -> anyhow::Result<String>;

    // TODO: Add more flexible control support on a running function
    // fn stop();
    // fn handle_event();
}

mod gbdt_training;
mod mesapy;
pub use gbdt_training::GbdtTraining;
pub use mesapy::Mesapy;

#[cfg(feature = "enclave_unit_test")]
pub mod tests {
    use super::*;
    use sgx_tunittest::*;

    pub fn run_tests() -> usize {
        rsgx_unit_tests!(
            gbdt_training::tests::test_gbdt_parse_training_data,
            gbdt_training::tests::test_gbdt_training,
        )
    }
}
