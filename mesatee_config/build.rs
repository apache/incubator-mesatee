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

use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").expect("$OUT_DIR not set. Please build with cargo");
    let dest_file = Path::new(&out_dir)
        .to_path_buf()
        .join("gen_build_config.rs");
    // the hardcoded path is a little ugly here, but makes it easy to debug without cmake
    let mut cmd = Command::new("../release/examples/config_gen");
    cmd.arg(dest_file);
    match cmd.status() {
        Ok(status) => {
            if !status.success() {
                panic!(
                    "Unspecified or invalid build config file. Please check $MESATEE_BUILD_CFG_DIR"
                );
            }
        }
        Err(e) => panic!("Failed to run config_gen: {:?}", e),
    }
}
