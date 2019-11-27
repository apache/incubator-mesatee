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

use super::common_setup::setup_tdfs_internal_client;
use crate::log::trace;

pub fn read_not_exist_file() {
    trace!("Test tdfs: read file.");
    let mut client = setup_tdfs_internal_client();
    let resp = client.read_file("xx", None);
    assert!(resp.is_err());
}

pub fn save_and_read() {
    trace!("Test tdfs: save and read file.");
    let mut client = setup_tdfs_internal_client();

    let data = b"abc";
    let user_id = "user1";
    let disallowed_user = "user2";
    let task_id = "task1";
    let allow_policy = 0;

    let file_id = client
        .save_file(data, user_id, task_id, &[], allow_policy)
        .unwrap();

    let plaintxt = client.read_file(&file_id, None).unwrap();
    assert_eq!(plaintxt, b"abc");

    let plaintxt = client.read_file(&file_id, Some(user_id)).unwrap();
    assert_eq!(plaintxt, b"abc");

    let read_err = client.read_file(&file_id, Some(disallowed_user));
    assert!(read_err.is_err());
}

pub fn check_file_permission() {
    trace!("Test tdfs: check file permission file.");
    let mut client = setup_tdfs_internal_client();

    let data = b"abcd";
    let user_id = "user1";
    let disallowed_user = "user2";
    let task_id = "task1";
    let allow_policy = 0;

    let file_id = client
        .save_file(data, user_id, task_id, &[], allow_policy)
        .unwrap();

    let plaintxt = client.read_file(&file_id, None).unwrap();
    assert_eq!(plaintxt, b"abcd");

    let accessible = client
        .check_access_permission(&file_id, &disallowed_user)
        .unwrap();
    assert!(!accessible);
}
pub fn task_share_file() {
    trace!("Test tdfs: save a file for user and collaborator.");
    let mut client = setup_tdfs_internal_client();

    let data = b"bcd";
    let user_id = "user1";
    let collorabor_list = vec!["user2"];
    let disallowed_user = "user3";

    let task_id = "task1";
    let allow_policy = 1;

    let file_id = client
        .save_file(data, user_id, task_id, &collorabor_list, allow_policy)
        .unwrap();

    let plaintxt = client.read_file(&file_id, None).unwrap();
    assert_eq!(plaintxt, b"bcd");

    let plaintxt = client.read_file(&file_id, Some(user_id)).unwrap();
    assert_eq!(plaintxt, b"bcd");

    let plaintxt = client.read_file(&file_id, Some("user2")).unwrap();
    assert_eq!(plaintxt, b"bcd");

    let read_err = client.read_file(&file_id, Some(disallowed_user));
    assert!(read_err.is_err());
}

pub fn global_share_file() {
    trace!("Test tdfs: global share file.");
    let mut client = setup_tdfs_internal_client();

    let data = b"cde";
    let user_id = "user1";
    let another_user = "user2";
    let task_id = "task1";
    let allow_policy = 2;

    let file_id = client
        .save_file(data, user_id, task_id, &[], allow_policy)
        .unwrap();

    let plaintxt = client.read_file(&file_id, None).unwrap();
    assert_eq!(plaintxt, b"cde");

    let plaintxt = client.read_file(&file_id, Some(user_id)).unwrap();
    assert_eq!(plaintxt, b"cde");

    let plaintxt = client.read_file(&file_id, Some(another_user)).unwrap();
    assert_eq!(plaintxt, b"cde");
}
