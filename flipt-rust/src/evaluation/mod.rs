pub mod models;

use crate::{error::UpstreamError, util::deserialize};
use models::{
    BatchEvaluationRequest, BatchEvaluationResponse, BooleanEvaluationResponse, EvaluationFlagType,
    EvaluationNamespaceSnapshot, EvaluationNamespaceSnapshotRequest, EvaluationRequest, Flag,
    VariantEvaluationResponse,
};
use reqwest::Client;
use url::Url;

pub struct Evaluation {
    client: Client,
    url: Url,
}

impl Evaluation {
    pub fn new(client: Client, url: Url) -> Self {
        Self { client, url }
    }

    pub async fn boolean(
        &self,
        request: &EvaluationRequest,
    ) -> Result<BooleanEvaluationResponse, UpstreamError> {
        let endpoint = format!("{}evaluate/v1/boolean", self.url.as_str());

        let response = match self.client.post(endpoint).json(request).send().await {
            Ok(r) => r,
            Err(e) => {
                return Err(UpstreamError::default_with_message(e.to_string()));
            }
        };

        deserialize(response).await
    }

    pub async fn variant(
        &self,
        request: &EvaluationRequest,
    ) -> Result<VariantEvaluationResponse, UpstreamError> {
        let endpoint = format!("{}evaluate/v1/variant", self.url.as_str());

        let response = match self.client.post(endpoint).json(request).send().await {
            Ok(r) => r,
            Err(e) => {
                return Err(UpstreamError::default_with_message(e.to_string()));
            }
        };

        deserialize(response).await
    }

    pub async fn batch(
        &self,
        batch: &BatchEvaluationRequest,
    ) -> Result<BatchEvaluationResponse, UpstreamError> {
        let endpoint = format!("{}evaluate/v1/batch", self.url.as_str());

        let response = match self.client.post(endpoint).json(batch).send().await {
            Ok(r) => r,
            Err(e) => {
                return Err(UpstreamError::default_with_message(e.to_string()));
            }
        };

        deserialize(response).await
    }

    pub async fn list_flags(
        &self,
        request: &EvaluationNamespaceSnapshotRequest,
    ) -> Result<Vec<Flag>, UpstreamError> {
        let result = self.inner_list_flags(request).await?;
        let flags: Vec<Flag> = result
            .flags
            .into_iter()
            .map(|f| Flag {
                key: f.key,
                enabled: f.enabled,
                r#type: EvaluationFlagType::try_from(f.r#type).unwrap_or_default(),
            })
            .collect::<Vec<Flag>>();
        Ok(flags)
    }

    async fn inner_list_flags(
        &self,
        request: &EvaluationNamespaceSnapshotRequest,
    ) -> Result<EvaluationNamespaceSnapshot, UpstreamError> {
        let endpoint = self.url(&request.key, request.reference.clone());

        let response = match self.client.get(endpoint).send().await {
            Ok(r) => r,
            Err(e) => {
                return Err(UpstreamError::default_with_message(e.to_string()));
            }
        };

        deserialize(response).await
    }

    fn url(&self, namespace: &str, reference: String) -> String {
        match reference.is_empty() {
            true => {
                format!(
                    "{}internal/v1/evaluation/snapshot/namespace/{}",
                    self.url.as_str(),
                    namespace
                )
            }
            false => {
                format!(
                    "{}internal/v1/evaluation/snapshot/namespace/{}?reference={}",
                    self.url.as_str(),
                    namespace,
                    reference,
                )
            }
        }
    }
}
