use crate::file::{InputFile, OutputFile};
use crate::fusion_data::FusionData;
use anyhow::{anyhow, Result};
use std::prelude::v1::*;
use std::sync::{Arc, SgxMutex as Mutex};
use teaclave_proto::teaclave_frontend_service::{
    GetFusionDataRequest, GetFusionDataResponse, GetOutputFileRequest, GetOutputFileResponse,
    RegisterInputFileRequest, RegisterInputFileResponse, RegisterOutputFileRequest,
    RegisterOutputFileResponse,
};
use teaclave_proto::teaclave_management_service::TeaclaveManagement;
use teaclave_proto::teaclave_storage_service::{GetRequest, PutRequest, TeaclaveStorageClient};
use teaclave_rpc::endpoint::Endpoint;
use teaclave_rpc::Request;
use teaclave_service_enclave_utils::teaclave_service;
use teaclave_types::{TeaclaveServiceResponseError, TeaclaveServiceResponseResult};
use thiserror::Error;

#[derive(Error, Debug)]
enum TeaclaveManagementError {
    #[error("invalid request")]
    InvalidRequest,
    #[error("data error")]
    DataError,
    #[error("storage error")]
    StorageError,
    #[error("permission denied")]
    PermissionDenied,
}

impl From<TeaclaveManagementError> for TeaclaveServiceResponseError {
    fn from(error: TeaclaveManagementError) -> Self {
        TeaclaveServiceResponseError::RequestError(error.to_string())
    }
}

#[teaclave_service(
    teaclave_management_service,
    TeaclaveManagement,
    TeaclaveManagementError
)]
#[derive(Clone)]
pub(crate) struct TeaclaveManagementService {
    storage_client: Arc<Mutex<TeaclaveStorageClient>>,
}

impl TeaclaveManagement for TeaclaveManagementService {
    fn register_input_file(
        &self,
        request: Request<RegisterInputFileRequest>,
    ) -> TeaclaveServiceResponseResult<RegisterInputFileResponse> {
        let user_id = request
            .metadata
            .get("id")
            .ok_or_else(|| TeaclaveManagementError::InvalidRequest)?
            .to_string();

        let request = request.message;
        let input_file = InputFile::new(request.url, request.hash, request.crypto_info, user_id);
        let key = input_file.get_key_vec();
        let value = input_file
            .to_vec()
            .map_err(|_| TeaclaveManagementError::DataError)?;

        self.write_to_storage(&key, &value)
            .map_err(|_| TeaclaveManagementError::StorageError)?;
        let response = RegisterInputFileResponse {
            data_id: input_file.data_id,
        };
        Ok(response)
    }

    fn register_output_file(
        &self,
        request: Request<RegisterOutputFileRequest>,
    ) -> TeaclaveServiceResponseResult<RegisterOutputFileResponse> {
        let user_id = request
            .metadata
            .get("id")
            .ok_or_else(|| TeaclaveManagementError::InvalidRequest)?
            .to_string();

        let request = request.message;
        let output_file = OutputFile::new(request.url, request.crypto_info, user_id);
        let key = output_file.get_key_vec();
        let value = output_file
            .to_vec()
            .map_err(|_| TeaclaveManagementError::DataError)?;

        self.write_to_storage(&key, &value)
            .map_err(|_| TeaclaveManagementError::StorageError)?;
        let response = RegisterOutputFileResponse {
            data_id: output_file.data_id,
        };
        Ok(response)
    }

    fn get_output_file(
        &self,
        request: Request<GetOutputFileRequest>,
    ) -> TeaclaveServiceResponseResult<GetOutputFileResponse> {
        let user_id = request
            .metadata
            .get("id")
            .ok_or_else(|| TeaclaveManagementError::InvalidRequest)?
            .to_string();
        let data_id = request.message.data_id;
        if !OutputFile::is_output_file_id(&data_id) {
            return Err(TeaclaveManagementError::PermissionDenied.into());
        }
        let key: &[u8] = data_id.as_bytes();
        let value = self
            .read_from_storage(key)
            .map_err(|_| TeaclaveManagementError::StorageError)?;
        let output_file =
            OutputFile::from_slice(&value).map_err(|_| TeaclaveManagementError::DataError)?;
        if output_file.owner != user_id {
            return Err(TeaclaveManagementError::PermissionDenied.into());
        }
        let response = GetOutputFileResponse {
            hash: output_file.hash.unwrap_or_else(|| "".to_string()),
        };
        Ok(response)
    }

    fn get_fusion_data(
        &self,
        request: Request<GetFusionDataRequest>,
    ) -> TeaclaveServiceResponseResult<GetFusionDataResponse> {
        let user_id = request
            .metadata
            .get("id")
            .ok_or_else(|| TeaclaveManagementError::InvalidRequest)?
            .to_string();
        let data_id = request.message.data_id;
        if !FusionData::is_fusion_data_id(&data_id) {
            return Err(TeaclaveManagementError::PermissionDenied.into());
        }
        let key = data_id.as_bytes();
        let value = self
            .read_from_storage(key)
            .map_err(|_| TeaclaveManagementError::StorageError)?;
        let fusion_data =
            FusionData::from_slice(&value).map_err(|_| TeaclaveManagementError::DataError)?;
        if !fusion_data.data_owner_id_list.contains(&user_id) {
            return Err(TeaclaveManagementError::PermissionDenied.into());
        }
        let response = GetFusionDataResponse {
            hash: fusion_data.hash.unwrap_or_else(|| "".to_string()),
            data_owner_id_list: fusion_data.data_owner_id_list,
        };
        Ok(response)
    }
}

impl TeaclaveManagementService {
    #[cfg(test_mode)]
    fn add_mock_data(&self) -> Result<()> {
        let mut fusion_data =
            FusionData::new(vec!["mock_user_a".to_string(), "mock_user_b".to_string()])?;
        fusion_data.data_id = "fusion-data-mock-data".to_string();
        let key = fusion_data.get_key_vec();
        let value = fusion_data.to_vec()?;
        self.write_to_storage(&key, &value)?;
        Ok(())
    }

    pub(crate) fn new(storage_service_endpoint: Endpoint) -> Result<Self> {
        let channel = storage_service_endpoint.connect()?;
        let client = TeaclaveStorageClient::new(channel)?;
        let service = Self {
            storage_client: Arc::new(Mutex::new(client)),
        };
        #[cfg(test_mode)]
        service.add_mock_data()?;
        Ok(service)
    }

    fn write_to_storage(&self, key: &[u8], value: &[u8]) -> Result<()> {
        let put_request = PutRequest::new(key, value);
        let _put_response = self
            .storage_client
            .clone()
            .lock()
            .map_err(|_| anyhow!("Cannot lock storage client"))?
            .put(put_request)?;
        Ok(())
    }

    fn read_from_storage(&self, key: &[u8]) -> Result<Vec<u8>> {
        let get_request = GetRequest::new(key);
        let get_response = self
            .storage_client
            .clone()
            .lock()
            .map_err(|_| anyhow!("Cannot lock storage client"))?
            .get(get_request)?;
        Ok(get_response.value)
    }
}

#[cfg(feature = "enclave_unit_test")]
pub mod tests {
    use super::*;
    use teaclave_types::{TeaclaveFileCryptoInfo, TeaclaveFileRootKey128};
    use url::Url;

    pub fn handle_input_file() {
        let url = Url::parse("s3://bucket_id/path?token=mock_token").unwrap();
        let hash = "a6d604b5987b693a19d94704532b5d928c2729f24dfd40745f8d03ac9ac75a8b".to_string();
        let user_id = "mock_user".to_string();
        let crypto_info = TeaclaveFileCryptoInfo::TeaclaveFileRootKey128(
            TeaclaveFileRootKey128::new(&[0; 16]).unwrap(),
        );
        let input_file = InputFile::new(url, hash, crypto_info, user_id);
        let key = input_file.get_key_vec();
        let key_str = std::str::from_utf8(&key).unwrap();
        info!("key: {}", key_str);
        assert!(InputFile::is_input_file_id(key_str));
        let value = input_file.to_vec().unwrap();
        let deserialized_file = InputFile::from_slice(&value).unwrap();
        info!("file: {:?}", deserialized_file);
    }

    pub fn handle_output_file() {
        let url = Url::parse("s3://bucket_id/path?token=mock_token").unwrap();
        let user_id = "mock_user".to_string();
        let crypto_info = TeaclaveFileCryptoInfo::TeaclaveFileRootKey128(
            TeaclaveFileRootKey128::new(&[0; 16]).unwrap(),
        );
        let output_file = OutputFile::new(url, crypto_info, user_id);
        let key = output_file.get_key_vec();
        let key_str = std::str::from_utf8(&key).unwrap();
        info!("key: {}", key_str);
        assert!(OutputFile::is_output_file_id(key_str));
        let value = output_file.to_vec().unwrap();
        let deserialized_file = OutputFile::from_slice(&value).unwrap();
        info!("file: {:?}", deserialized_file);
    }

    pub fn handle_fusion_data() {
        let fusion_data =
            FusionData::new(vec!["mock_user_a".to_string(), "mock_user_b".to_string()]).unwrap();
        let key = fusion_data.get_key_vec();
        let key_str = std::str::from_utf8(&key).unwrap();
        info!("key: {}", key_str);
        assert!(FusionData::is_fusion_data_id(key_str));
        let value = fusion_data.to_vec().unwrap();
        let deserialized_data = FusionData::from_slice(&value).unwrap();
        info!("data: {:?}", deserialized_data);
    }
}
