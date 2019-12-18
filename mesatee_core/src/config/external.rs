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

use super::get_trusted_enclave_attr;
use super::ServiceConfig;
use super::{InboundDesc, OutboundDesc, TargetDesc};
use teaclave_config::runtime_config;

pub struct External;
impl External {
    pub fn tms() -> ServiceConfig {
        ServiceConfig::new(
            runtime_config::config().api_endpoints.tms.listen_address,
            InboundDesc::External,
        )
    }

    pub fn fns() -> ServiceConfig {
        ServiceConfig::new(
            runtime_config::config().api_endpoints.fns.listen_address,
            InboundDesc::External,
        )
    }

    pub fn tdfs() -> ServiceConfig {
        ServiceConfig::new(
            runtime_config::config().api_endpoints.tdfs.listen_address,
            InboundDesc::External,
        )
    }

    pub fn target_fns() -> TargetDesc {
        TargetDesc::new(
            runtime_config::config()
                .api_endpoints
                .fns
                .advertised_address,
            OutboundDesc::Sgx(get_trusted_enclave_attr(vec!["fns"])),
        )
    }
}
