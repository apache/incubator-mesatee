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

use super::common_setup::setup_kms_internal_client;

pub fn api_create_key() {
    trace!("Test kms: api_create_key.");
    let mut client = setup_kms_internal_client();
    let resp = client.request_create_key().unwrap();

    let key_id = resp.key_id;
    let key_config = resp.config;
    let resp = client.request_get_key(&key_id).unwrap();

    assert_eq!(key_config, resp.config);
}

pub fn api_get_deleted_key() {
    trace!("Test kms: api get deleted key.");
    let mut client = setup_kms_internal_client();
    let resp = client.request_get_key("fake_kms_record_to_be_deleted");
    assert!(resp.is_err());
}
