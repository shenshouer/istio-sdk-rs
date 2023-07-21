# istio-sdk-rs
A collection of CRDs used in Istio, generated by [`kopium`](https://github.com/kube-rs/kopium) directly from [`istio CRDs`](https://github.com/istio/istio/blob/master/manifests/charts/base/crds/crd-all.gen.yaml).

## Quick Start

`istio-sdk-rs` is built on top of [`kube-rs`](https://github.com/kube-rs/kube-rs) as a set of CRDs, which means it can be easily used under `kube_rs` like below:

```rust
use istio_api_rs::networking::v1beta1::destination_rule::*;
use istio_api_rs::networking::v1beta1::gateway::*;
use istio_api_rs::networking::v1beta1::virtual_service::*;
use kube::{
    api::{Api, ListParams, ResourceExt},
    client::ConfigExt,
    config::{KubeConfigOptions, Kubeconfig},
    Client, Config,
};
use tower::ServiceBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let kube_config_file = std::fs::File::open("./your/kube/config.yaml")?;
    let kube_config: Kubeconfig = serde_yaml::from_reader(kube_config_file)?;
    let kube_config_opt = KubeConfigOptions::default();
    let kube_config = Config::from_custom_kubeconfig(kube_config, &kube_config_opt).await?;
    let https = kube_config.openssl_https_connector()?;

    tracing_subscriber::fmt::init();

    let service = ServiceBuilder::new()
        .layer(kube_config.base_uri_layer())
        .option_layer(kube_config.auth_layer()?)
        .service(hyper::Client::builder().build(https));
    let client = Client::new(service, kube_config.default_namespace);
    let list_opt = ListParams::default();

    let gws: Api<Gateway> = Api::namespaced(client.clone(), "my-ns");
    for gw in gws.list(&list_opt).await? {
        println!("Found Gateway: {}", gw.name());
    }

    let drs: Api<DestinationRule> = Api::namespaced(client.clone(), "my-ns");
    for dr in drs.list(&list_opt).await? {
        println!("Found Destination Rule: {}", dr.name());
    }

    let vss: Api<VirtualService> = Api::namespaced(client.clone(), "my-ns");
    for vs in vss.list(&list_opt).await? {
        let content = serde_yaml::to_string(&vs).unwrap();
        println!("Found Virtual Service with YAML content: {}", content);
    }

    Ok(())
}
```

And in `cargo.toml`, you should specify the API version for both `k8s` & `istio` like:

```toml
[dependencies]
# ...
kube = { version = "0.74", features = ["runtime", "derive"] }
k8s-openapi = { version = "0.15", features = ["v1_18"] }
istio-api-rs = { version = "0.1.0", features = ["v1_11"] }
# ...
```

## Other

`istio-api-rs` is currently developed and tested on istio/api since v1.10, the lower api version is out of this repository's concern.

The repository is using [`istio-api-rs-codegen`](https://github.com/BlankZhu/istio-api-rs-codegen) as code generator, go check that repository if you want to know more about how the codes are generated.

For release package, see [crate.io](https://crates.io/crates/istio-api-rs).