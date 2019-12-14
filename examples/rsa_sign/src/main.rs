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

use lazy_static::lazy_static;
use mesatee_sdk::{Mesatee, MesateeEnclaveInfo};
use std::net::SocketAddr;
use std::{env, fs};

static FUNCTION_NAME: &str = "rsa_sign";
static USER_ID: &str = "uid";
static USER_TOKEN: &str = "token";

lazy_static! {
    static ref TMS_ADDR: SocketAddr = "127.0.0.1:5554".parse().unwrap();
    static ref TDFS_ADDR: SocketAddr = "127.0.0.1:5065".parse().unwrap();
}

fn print_usage() {
    let msg = "
    ./rsa_sign upload_key key_path key_file_id_saving_path
    ./rsa_sign sign key_file_id input_path output_path
    ";
    println!("usage: \n{}", msg);
}

fn upload_key(info: &MesateeEnclaveInfo, key_path: &str, key_file_id_saving_path: &str) {
    let mesatee = Mesatee::new(info, USER_ID, USER_TOKEN, *TMS_ADDR, *TDFS_ADDR).unwrap();
    let file_id = mesatee.upload_file(key_path).unwrap();
    fs::write(key_file_id_saving_path, file_id.as_bytes()).unwrap();
}

fn sign(
    info: &MesateeEnclaveInfo,
    key_file_id: &str,
    input_data_path: &str,
    output_sig_path: &str,
) {
    let input = fs::read(input_data_path).unwrap();
    let base64_input = base64::encode(&input);
    let mesatee = Mesatee::new(info, USER_ID, USER_TOKEN, *TMS_ADDR, *TDFS_ADDR).unwrap();
    let task = mesatee
        .create_task_with_files(FUNCTION_NAME, &[key_file_id])
        .unwrap();
    let result = task.invoke_with_payload(&base64_input).unwrap();
    let sig = base64::decode(&result).unwrap();
    fs::write(output_sig_path, &sig).unwrap();
}

fn main() {
    let auditors = vec![
        (
            "../services/auditors/godzilla/godzilla.public.der",
            "../services/auditors/godzilla/godzilla.sign.sha256",
        ),
        (
            "../services/auditors/optimus_prime/optimus_prime.public.der",
            "../services/auditors/optimus_prime/optimus_prime.sign.sha256",
        ),
        (
            "../services/auditors/albus_dumbledore/albus_dumbledore.public.der",
            "../services/auditors/albus_dumbledore/albus_dumbledore.sign.sha256",
        ),
    ];
    let enclave_info_file_path = "../services/enclave_info.toml";

    let mesatee_enclave_info = MesateeEnclaveInfo::load(auditors, enclave_info_file_path).unwrap();

    let args_string: Vec<String> = env::args().collect();
    let args: Vec<&str> = args_string.iter().map(|s| s.as_str()).collect();
    if args.len() < 2 {
        print_usage();
        return;
    }
    let action = args[1];
    match action {
        "upload_key" => {
            if args.len() != 4 {
                print_usage();
                return;
            }
            upload_key(&mesatee_enclave_info, args[2], args[3]);
        }
        "sign" => {
            if args.len() != 5 {
                print_usage();
                return;
            }
            sign(&mesatee_enclave_info, args[2], args[3], args[4]);
        }
        _ => {
            print_usage();
        }
    }
}
