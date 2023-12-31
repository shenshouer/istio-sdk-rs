// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -Af -
// kopium version: 0.15.0

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, time::Duration};

use crate::types::PortSelector;

/// Configuration affecting label/content routing, sni routing, etc. See more details at: https://istio.io/docs/reference/config/networking/virtual-service.html
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema, Default)]
#[kube(
    group = "networking.istio.io",
    version = "v1beta1",
    kind = "VirtualService",
    plural = "virtualservices"
)]
#[kube(namespaced)]
pub struct VirtualServiceSpec {
    /// The destination hosts to which traffic is being sent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hosts: Option<Vec<String>>,
    /// The names of gateways and sidecars that should apply these routes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateways: Option<Vec<String>>,
    /// An ordered list of route rules for HTTP traffic.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<Vec<HTTPRoute>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<Vec<TLSRoute>>,
    /// An ordered list of route rules for opaque TCP traffic.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcp: Option<Vec<TCPRoute>>,
    /// A list of namespaces to which this virtual service is exported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "exportTo")]
    pub export_to: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default)]
pub struct HTTPRoute {
    /// The name assigned to the route for debugging purposes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub r#match: Option<Vec<HTTPMatchRequest>>,
    /// A HTTP rule can either return a direct_response, redirect or forward (default) traffic.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route: Option<Vec<HTTPRouteDestination>>,
    /// A HTTP rule can either return a direct_response, redirect or forward (default) traffic.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect: Option<HTTPRedirect>,
    /// A HTTP rule can either return a direct_response, redirect or forward (default) traffic.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "directResponse"
    )]
    pub direct_response: Option<HTTPDirectResponse>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delegate: Option<Delegate>,
    /// Rewrite HTTP URIs and Authority headers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rewrite: Option<HTTPRewrite>,
    /// Timeout for HTTP requests, default is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<Duration>,
    /// Retry policy for HTTP requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retries: Option<HTTPRetry>,
    /// Fault injection policy to apply on HTTP traffic at the client side.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fault: Option<HTTPFaultInjection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirror: Option<Destination>,
    /// Percentage of the traffic to be mirrored by the `mirror` field.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "mirrorPercent"
    )]
    pub mirror_percent: Option<i64>,
    /// Percentage of the traffic to be mirrored by the `mirror` field.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "mirrorPercentage"
    )]
    pub mirror_percentage: Option<Percent>,
    /// Cross-Origin Resource Sharing policy (CORS).
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "corsPolicy"
    )]
    pub cors_policy: Option<CorsPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<Headers>,
}

/// Cross-Origin Resource Sharing policy (CORS).
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct CorsPolicy {
    /// The list of origins that are allowed to perform CORS requests.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "allowOrigin"
    )]
    pub allow_origin: Option<Vec<String>>,
    /// String patterns that match allowed origins.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "allowOrigins"
    )]
    pub allow_origins: Option<Vec<StringMatch>>,
    /// List of HTTP methods allowed to access the resource.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "allowMethods"
    )]
    pub allow_methods: Option<Vec<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "allowHeaders"
    )]
    pub allow_headers: Option<Vec<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "exposeHeaders"
    )]
    pub expose_headers: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxAge")]
    pub max_age: Option<Duration>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "allowCredentials"
    )]
    pub allow_credentials: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct Delegate {
    /// Name specifies the name of the delegate VirtualService.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace specifies the namespace where the delegate VirtualService resides.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// A HTTP rule can either return a direct_response, redirect or forward (default) traffic.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct HTTPDirectResponse {
    /// Specifies the content of the response body.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<HTTPBody>,
    /// Specifies the HTTP response status to be returned.
    pub status: i32,
}

/// Specifies the content of the response body.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct HTTPBody {
    /// response body as base64 encoded bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,
}

/// Fault injection policy to apply on HTTP traffic at the client side.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct HTTPFaultInjection {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abort: Option<Abort>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay: Option<Delay>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct Abort {
    /// HTTP status code to use to abort the Http request.
    #[serde(rename = "httpStatus")]
    pub http_status: i32,
    /// GRPC status code to use to abort the request.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "grpcStatus"
    )]
    pub grpc_status: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "http2Error"
    )]
    pub http2_error: Option<String>,

    /// Percentage of requests to be aborted with the error code provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentage: Option<Percent>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct Delay {
    /// Add a fixed delay before forwarding the request.
    #[serde(rename = "fixedDelay")]
    pub fixed_delay: Duration,
    /// Percentage of requests on which the delay will be injected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentage: Option<Percent>,
    /// Percentage of requests on which the delay will be injected (0-100).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percent: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "exponentialDelay"
    )]
    pub exponential_delay: Option<String>,
}

/// Percentage of requests on which the delay will be injected.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct Percent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default)]
pub struct HTTPMatchRequest {
    /// The name assigned to a match.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<StringMatch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<StringMatch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<StringMatch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority: Option<StringMatch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<BTreeMap<String, StringMatch>>,
    /// Specifies the ports on the host that is being addressed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<u32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sourceLabels"
    )]
    pub source_labels: Option<BTreeMap<String, String>>,
    /// Names of gateways where the rule should be applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateways: Option<Vec<String>>,
    /// Query parameters for matching.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "queryParams"
    )]
    pub query_params: Option<BTreeMap<String, StringMatch>>,
    /// Flag to specify whether the URI matching should be case-insensitive.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "ignoreUriCase"
    )]
    pub ignore_uri_case: Option<bool>,
    /// withoutHeader has the same syntax with the header, but has opposite meaning.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "withoutHeaders"
    )]
    pub without_headers: Option<BTreeMap<String, StringMatch>>,
    /// Source namespace constraining the applicability of a rule to workloads in that namespace.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sourceNamespace"
    )]
    pub source_namespace: Option<String>,
    /// The human readable prefix to use when emitting statistics for this route.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "statPrefix"
    )]
    pub stat_prefix: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default)]
pub struct StringMatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// RE2 style regex-based match (https://github.com/google/re2/wiki/Syntax).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
}

/// A HTTP rule can either return a direct_response, redirect or forward (default) traffic.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct HTTPRedirect {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
    /// On a redirect, overwrite the port portion of the URL with this value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "derivePort"
    )]
    pub derive_port: Option<RedirectPortSelection>,
    /// On a redirect, overwrite the scheme portion of the URL with this value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "redirectCode"
    )]
    pub redirect_code: Option<i64>,
}

/// A HTTP rule can either return a direct_response, redirect or forward (default) traffic.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum RedirectPortSelection {
    #[serde(rename = "FROM_PROTOCOL_DEFAULT")]
    FromProtocolDefault,
    #[serde(rename = "FROM_REQUEST_PORT")]
    FromRequestPort,
}

/// Retry policy for HTTP requests.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct HTTPRetry {
    /// Number of retries to be allowed for a given request.
    pub attempts: i32,
    /// Timeout per attempt for a given request, including the initial call and any retries.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "perTryTimeout"
    )]
    pub per_try_timeout: Option<Duration>,
    /// Specifies the conditions under which retry takes place.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "retryOn")]
    pub retry_on: Option<String>,
    /// Flag to specify whether the retries should retry to other localities.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "retryRemoteLocalities"
    )]
    pub retry_remote_localities: Option<bool>,
}

/// Rewrite HTTP URIs and Authority headers.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct HTTPRewrite {
    /// rewrite the Authority/Host header with this value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default)]
pub struct HTTPRouteDestination {
    pub destination: Destination,
    /// Weight specifies the relative proportion of traffic to be forwarded to the destination.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<Headers>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default)]
pub struct Destination {
    /// The name of a service from the service registry.
    pub host: String,
    /// The name of a subset within the service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subset: Option<String>,
    /// Specifies the port on the host that is being addressed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<PortSelector>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct Headers {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<HeaderOperations>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<HeaderOperations>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct HeaderOperations {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default)]
pub struct TCPRoute {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub r#match: Option<Vec<L4MatchAttributes>>,
    /// The destination to which the connection should be forwarded to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route: Option<Vec<RouteDestination>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct L4MatchAttributes {
    /// IPv4 or IPv6 ip addresses of destination with optional subnet.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "destinationSubnets"
    )]
    pub destination_subnets: Option<Vec<String>>,

    /// Specifies the port on the host that is being addressed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sourceLabels"
    )]
    pub source_labels: Option<BTreeMap<String, String>>,
    /// Names of gateways where the rule should be applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateways: Option<Vec<String>>,
    /// Source namespace constraining the applicability of a rule to workloads in that namespace.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sourceNamespace"
    )]
    pub source_namespace: Option<String>,
    /// IPv4 or IPv6 ip address of source with optional subnet.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sourceSubnet"
    )]
    pub source_subnet: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default)]
pub struct TLSRoute {
    #[serde(rename = "match")]
    pub r#match: Vec<TLSMatchAttributes>,
    /// The destination to which the connection should be forwarded to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route: Option<Vec<RouteDestination>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TLSMatchAttributes {
    /// SNI (server name indicator) to match on.
    #[serde(rename = "sniHosts")]
    pub sni_hosts: Vec<String>,

    /// IPv4 or IPv6 ip addresses of destination with optional subnet.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "destinationSubnets"
    )]
    pub destination_subnets: Option<Vec<String>>,
    /// Specifies the port on the host that is being addressed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<u32>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sourceLabels"
    )]
    pub source_labels: Option<BTreeMap<String, String>>,
    /// Names of gateways where the rule should be applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateways: Option<Vec<String>>,
    /// Source namespace constraining the applicability of a rule to workloads in that namespace.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sourceNamespace"
    )]
    pub source_namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct RouteDestination {
    pub destination: Destination,
    /// Weight specifies the relative proportion of traffic to be forwarded to the destination.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}
