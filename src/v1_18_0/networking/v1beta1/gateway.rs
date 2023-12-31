// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -Af -
// kopium version: 0.15.0

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Configuration affecting edge load balancer. See more details at: https://istio.io/docs/reference/config/networking/gateway.html
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "networking.istio.io",
    version = "v1beta1",
    kind = "Gateway",
    plural = "gateways"
)]
#[kube(namespaced)]
pub struct GatewaySpec {
    /// A list of server specifications.
    pub servers: Vec<Server>,
    pub selector: BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default)]
pub struct Server {
    pub port: Port,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    /// One or more hosts exposed by this gateway.
    pub hosts: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "defaultEndpoint"
    )]
    pub default_endpoint: Option<String>,
    /// Set of TLS related options that govern the server's behavior.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<ServerTLSSettings>,
    /// An optional name of the server, when set must be unique across all servers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default)]
pub struct Port {
    /// Label assigned to the port.
    pub name: String,
    /// A valid non-negative integer port number.
    pub number: u32,
    /// The protocol exposed on the port.
    pub protocol: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "targetPort"
    )]
    pub target_port: Option<i64>,
}

/// Set of TLS related options that govern the server's behavior.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default)]
pub struct ServerTLSSettings {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "httpsRedirect"
    )]
    pub https_redirect: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<TLSmode>,
    /// REQUIRED if mode is `SIMPLE` or `MUTUAL`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "serverCertificate"
    )]
    pub server_certificate: Option<String>,
    /// REQUIRED if mode is `SIMPLE` or `MUTUAL`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "privateKey"
    )]
    pub private_key: Option<String>,
    /// REQUIRED if mode is `MUTUAL`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "caCertificates"
    )]
    pub ca_certificates: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "credentialName"
    )]
    pub credential_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "subjectAltNames"
    )]
    pub subject_alt_names: Option<Vec<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "verifyCertificateSpki"
    )]
    pub verify_certificate_spki: Option<Vec<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "verifyCertificateHash"
    )]
    pub verify_certificate_hash: Option<Vec<String>>,
    /// Optional: Minimum TLS protocol version.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "minProtocolVersion"
    )]
    pub min_protocol_version: Option<TLSProtocol>,
    /// Optional: Maximum TLS protocol version.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "maxProtocolVersion"
    )]
    pub max_protocol_version: Option<TLSProtocol>,
    /// Optional: If specified, only support the specified cipher list.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "cipherSuites"
    )]
    pub cipher_suites: Option<Vec<String>>,
}

/// Set of TLS related options that govern the server's behavior.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum TLSProtocol {
    #[serde(rename = "TLS_AUTO")]
    TlsAuto,
    #[serde(rename = "TLSV1_0")]
    Tlsv10,
    #[serde(rename = "TLSV1_1")]
    Tlsv11,
    #[serde(rename = "TLSV1_2")]
    Tlsv12,
    #[serde(rename = "TLSV1_3")]
    Tlsv13,
}

/// Set of TLS related options that govern the server's behavior.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum TLSmode {
    #[serde(rename = "PASSTHROUGH")]
    Passthrough,
    #[serde(rename = "SIMPLE")]
    Simple,
    #[serde(rename = "MUTUAL")]
    Mutual,
    #[serde(rename = "AUTO_PASSTHROUGH")]
    AutoPassthrough,
    #[serde(rename = "ISTIO_MUTUAL")]
    IstioMutual,
}
