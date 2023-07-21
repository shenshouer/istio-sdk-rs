// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -Af -
// kopium version: 0.15.0

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// Provides configuration for individual workloads. See more details at: https://istio.io/docs/reference/config/networking/proxy-config.html
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "networking.istio.io", version = "v1beta1", kind = "ProxyConfig", plural = "proxyconfigs")]
#[kube(namespaced)]
#[kube(status = "ProxyConfigStatus")]
pub struct ProxyConfigSpec {
    /// The number of worker threads to run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<i64>,
    /// Additional environment variables for the proxy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "environmentVariables")]
    pub environment_variables: Option<BTreeMap<String, String>>,
    /// Specifies the details of the proxy image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<ProxyConfigImage>,
    /// Optional.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<ProxyConfigSelector>,
}

/// Specifies the details of the proxy image.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ProxyConfigImage {
    /// The image type of the image.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageType")]
    pub image_type: Option<String>,
}

/// Optional.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ProxyConfigSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

