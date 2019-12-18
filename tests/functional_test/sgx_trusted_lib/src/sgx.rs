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

use mesatee_core::ipc::protos::ecall::{RunFunctionalTestInput, RunFunctionalTestOutput};
use mesatee_core::prelude::*;
use mesatee_core::Result;

use crate::tests;
use sgx_tunittest::*;

register_ecall_handler!(
    type ECallCommand,
    (ECallCommand::RunFunctionalTest, RunFunctionalTestInput, RunFunctionalTestOutput),
    (ECallCommand::InitEnclave, InitEnclaveInput, InitEnclaveOutput),
    (ECallCommand::FinalizeEnclave, FinalizeEnclaveInput, FinalizeEnclaveOutput),
);

#[handle_ecall]
fn handle_run_functional_test(_args: &RunFunctionalTestInput) -> Result<RunFunctionalTestOutput> {
    let nfailed = rsgx_unit_tests!(
        tests::leveldb_test::test_write_a_lot,
        tests::protected_fs_test::read_write_large_file,
        tests::kms_test::api_create_key,
        tests::kms_test::api_get_deleted_key,
        tests::tdfs_test::read_not_exist_file,
        tests::tdfs_test::save_and_read,
        tests::tdfs_test::check_file_permission,
        tests::tdfs_test::task_share_file,
        tests::tdfs_test::global_share_file,
        tests::tms_test::get_task,
        tests::tms_test::update_task_result,
        tests::tms_test::update_private_result,
        tests::tms_test::update_status,
        tests::acs_test::access_control_model,
    );

    Ok(RunFunctionalTestOutput::new(nfailed))
}

#[handle_ecall]
fn handle_init_enclave(_args: &InitEnclaveInput) -> Result<InitEnclaveOutput> {
    mesatee_core::init_service(env!("CARGO_PKG_NAME"))?;

    Ok(InitEnclaveOutput::default())
}

#[handle_ecall]
fn handle_finalize_enclave(_args: &FinalizeEnclaveInput) -> Result<FinalizeEnclaveOutput> {
    #[cfg(feature = "cov")]
    sgx_cov::cov_writeout();

    info!("Enclave [Functional Test]: Finalized.");
    Ok(FinalizeEnclaveOutput::default())
}
