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
use super::runtime_config;
use super::InboundDesc;
use super::OutboundDesc;
use super::ServiceConfig;
use super::TargetDesc;

pub struct Internal;
impl Internal {
    pub fn tms() -> ServiceConfig {
        ServiceConfig::new(
            runtime_config().internal_endpoints.tms.listen_address,
            InboundDesc::Sgx(get_trusted_enclave_attr(vec!["fns"])),
        )
    }

    pub fn kms() -> ServiceConfig {
        ServiceConfig::new(
            runtime_config().internal_endpoints.kms.listen_address,
            InboundDesc::Sgx(get_trusted_enclave_attr(vec!["fns", "tdfs"])),
        )
    }

    pub fn tdfs() -> ServiceConfig {
        ServiceConfig::new(
            runtime_config().internal_endpoints.tdfs.listen_address,
            InboundDesc::Sgx(get_trusted_enclave_attr(vec!["fns", "tms"])),
        )
    }

    pub fn acs() -> ServiceConfig {
        ServiceConfig::new(
            runtime_config().internal_endpoints.acs.listen_address,
            InboundDesc::Sgx(get_trusted_enclave_attr(vec!["kms", "tms", "tdfs"])),
        )
    }
    pub fn dbs() -> ServiceConfig {
        ServiceConfig::new(
            runtime_config().internal_endpoints.dbs.listen_address,
            InboundDesc::External,
        )
    }

    pub fn target_tms() -> TargetDesc {
        TargetDesc::new(
            runtime_config().internal_endpoints.tms.advertised_address,
            OutboundDesc::Sgx(get_trusted_enclave_attr(vec!["tms"])),
        )
    }

    pub fn target_kms() -> TargetDesc {
        TargetDesc::new(
            runtime_config().internal_endpoints.kms.advertised_address,
            OutboundDesc::Sgx(get_trusted_enclave_attr(vec!["kms"])),
        )
    }

    pub fn target_tdfs() -> TargetDesc {
        TargetDesc::new(
            runtime_config().internal_endpoints.tdfs.advertised_address,
            OutboundDesc::Sgx(get_trusted_enclave_attr(vec!["tdfs"])),
        )
    }

    pub fn target_acs() -> TargetDesc {
        TargetDesc::new(
            runtime_config().internal_endpoints.acs.advertised_address,
            OutboundDesc::Sgx(get_trusted_enclave_attr(vec!["acs"])),
        )
    }
}
