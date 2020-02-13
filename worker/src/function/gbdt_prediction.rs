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

use std::format;
use std::io::{self, BufRead, BufReader, Write};

use anyhow;
use serde_json;

use crate::function::TeaclaveFunction;
use crate::runtime::TeaclaveRuntime;
use teaclave_types::TeaclaveFunctionArguments;

use gbdt::decision_tree::Data;
use gbdt::gradient_boost::GBDT;

#[derive(Default)]
pub struct GbdtPrediction;

static IN_MODEL: &str = "if_model";
static IN_DATA: &str = "if_data";
static OUT_RESULT: &str = "of_result";

impl TeaclaveFunction for GbdtPrediction {
    fn execute(
        &self,
        runtime: Box<dyn TeaclaveRuntime + Send + Sync>,
        _args: TeaclaveFunctionArguments,
    ) -> anyhow::Result<String> {
        let mut json_model = String::new();
        let mut f = runtime.open_input(IN_MODEL)?;
        f.read_to_string(&mut json_model)?;

        let model: GBDT = serde_json::from_str(&json_model)?;

        let in_data = runtime.open_input(IN_DATA)?;
        let test_data = parse_test_data(in_data)?;

        let predict_set = model.predict(&test_data);

        let mut of_result = runtime.create_output(OUT_RESULT)?;
        for predict_value in predict_set.iter() {
            writeln!(&mut of_result, "{:.10}", predict_value)?
        }

        let summary = format!("Predict result has {} lines of data.", predict_set.len());
        Ok(summary)
    }
}

fn parse_data_line(line: &str) -> anyhow::Result<Data> {
    let trimed_line = line.trim();
    anyhow::ensure!(!trimed_line.is_empty(), "Empty line");

    let mut features: Vec<f32> = Vec::new();
    for feature_str in trimed_line.split(',') {
        let trimed_feature_str = feature_str.trim();
        anyhow::ensure!(!trimed_feature_str.is_empty(), "Empty feature");

        let feature: f32 = trimed_feature_str.parse()?;
        features.push(feature);
    }
    Ok(Data::new_test_data(features, None))
}

fn parse_test_data(input: impl io::Read) -> anyhow::Result<Vec<Data>> {
    let mut samples: Vec<Data> = Vec::new();

    let reader = BufReader::new(input);
    for line_result in reader.lines() {
        let line = line_result?;
        let data = parse_data_line(&line)?;
        samples.push(data);
    }

    Ok(samples)
}

#[cfg(feature = "enclave_unit_test")]
pub mod tests {
    use super::*;
    use teaclave_test_utils::*;

    use std::untrusted::fs;

    use teaclave_types::hashmap;
    use teaclave_types::TeaclaveFileCryptoInfo;
    use teaclave_types::TeaclaveFunctionArguments;
    use teaclave_types::TeaclaveWorkerFileInfo;
    use teaclave_types::TeaclaveWorkerFileRegistry;

    use crate::function::TeaclaveFunction;
    use crate::runtime::RawIoRuntime;

    pub fn run_tests() -> bool {
        run_tests!(test_gbdt_prediction)
    }

    fn test_gbdt_prediction() {
        let func_args = TeaclaveFunctionArguments::default();

        let plain_if_model = "test_cases/gbdt_prediction/model.txt";
        let plain_if_data = "test_cases/gbdt_prediction/test_data.txt";
        let plain_output = "test_cases/gbdt_prediction/result.txt.out";
        let expected_output = "test_cases/gbdt_prediction/expected_result.txt";

        let input_files = TeaclaveWorkerFileRegistry::new(hashmap!(
            IN_MODEL.to_string() =>
            TeaclaveWorkerFileInfo::new(plain_if_model, TeaclaveFileCryptoInfo::default()),
            IN_DATA.to_string() =>
            TeaclaveWorkerFileInfo::new(plain_if_data, TeaclaveFileCryptoInfo::default())
        ));

        let output_files = TeaclaveWorkerFileRegistry::new(hashmap!(
            OUT_RESULT.to_string() =>
            TeaclaveWorkerFileInfo::new(plain_output, TeaclaveFileCryptoInfo::default())
        ));

        let runtime = Box::new(RawIoRuntime::new(input_files, output_files));

        let function = GbdtPrediction;
        let summary = function.execute(runtime, func_args).unwrap();
        assert_eq!(summary, "Predict result has 30 lines of data.");

        let result = fs::read_to_string(&plain_output).unwrap();
        let expected = fs::read_to_string(&expected_output).unwrap();
        assert_eq!(&result[..], &expected[..]);
    }
}
