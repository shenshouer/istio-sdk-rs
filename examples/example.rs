use istio_sdk::networking::v1beta1::destination_rule::*;
use istio_sdk::networking::v1beta1::gateway::*;
use istio_sdk::networking::v1beta1::virtual_service::*;
use kube::{
    api::{Api, ListParams},
    ResourceExt,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = kube::Client::try_default().await?;

    tracing_subscriber::fmt::init();

    let list_opt = ListParams::default();

    let gws: Api<Gateway> = Api::namespaced(client.clone(), "default");
    for gw in gws.list(&list_opt).await? {
        println!("Found Gateway: {}", gw.name_any());
    }

    let drs: Api<DestinationRule> = Api::namespaced(client.clone(), "default");
    for dr in drs.list(&list_opt).await? {
        println!("Found Destination Rule: {}", dr.name_any());
    }

    let vss: Api<VirtualService> = Api::namespaced(client.clone(), "default");
    for vs in vss.list(&list_opt).await? {
        let content = serde_yaml::to_string(&vs).unwrap();
        println!("Found Virtual Service with YAML content: {}", content);
    }

    Ok(())
}
