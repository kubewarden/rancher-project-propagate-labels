use k8s_openapi::apimachinery::pkg::api::resource::Quantity;
use k8s_openapi::schemars;

#[derive(Clone, Debug, PartialEq, schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NamespaceResourceQuota {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<ResourceQuotaLimit>,
}

impl k8s_openapi::DeepMerge for NamespaceResourceQuota {
    fn merge_from(&mut self, other: Self)
    where
        Self: Sized,
    {
        self.limit.merge_from(other.limit);
    }
}

#[derive(Clone, Debug, PartialEq, schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectResourceQuota {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<ResourceQuotaLimit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_limit: Option<ResourceQuotaLimit>,
}

impl k8s_openapi::DeepMerge for ProjectResourceQuota {
    fn merge_from(&mut self, other: Self)
    where
        Self: Sized,
    {
        self.limit.merge_from(other.limit);
        self.used_limit.merge_from(other.used_limit);
    }
}

#[derive(Clone, Debug, PartialEq, schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceQuotaLimit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pods: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_controllers: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_maps: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_volume_claims: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services_node_ports: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services_load_balancers: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_cpu: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_memory: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_storage: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits_cpu: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits_memory: Option<Quantity>,
}

impl Default for ResourceQuotaLimit {
    fn default() -> Self {
        ResourceQuotaLimit {
            pods: Some(Quantity("0".to_string())),
            services: Some(Quantity("0".to_string())),
            replication_controllers: Some(Quantity("0".to_string())),
            secrets: Some(Quantity("0".to_string())),
            config_maps: Some(Quantity("0".to_string())),
            persistent_volume_claims: Some(Quantity("0".to_string())),
            services_node_ports: Some(Quantity("0".to_string())),
            services_load_balancers: Some(Quantity("0".to_string())),
            requests_cpu: Some(Quantity("0".to_string())),
            requests_memory: Some(Quantity("0".to_string())),
            requests_storage: Some(Quantity("0".to_string())),
            limits_cpu: Some(Quantity("0".to_string())),
            limits_memory: Some(Quantity("0".to_string())),
        }
    }
}

impl k8s_openapi::DeepMerge for ResourceQuotaLimit {
    fn merge_from(&mut self, other: Self)
    where
        Self: Sized,
    {
        self.pods.merge_from(other.pods);
        self.services.merge_from(other.services);
        self.replication_controllers
            .merge_from(other.replication_controllers);
        self.secrets.merge_from(other.secrets);
        self.config_maps.merge_from(other.config_maps);
        self.persistent_volume_claims
            .merge_from(other.persistent_volume_claims);
        self.services_node_ports
            .merge_from(other.services_node_ports);
        self.services_load_balancers
            .merge_from(other.services_load_balancers);
        self.requests_cpu.merge_from(other.requests_cpu);
        self.requests_memory.merge_from(other.requests_memory);
        self.requests_storage.merge_from(other.requests_storage);
        self.limits_cpu.merge_from(other.limits_cpu);
        self.limits_memory.merge_from(other.limits_memory);
    }
}

#[derive(Clone, Debug, PartialEq, schemars::JsonSchema, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerResourceLimit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_cpu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_memory: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits_cpu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits_memory: Option<String>,
}

impl k8s_openapi::DeepMerge for ContainerResourceLimit {
    fn merge_from(&mut self, other: Self)
    where
        Self: Sized,
    {
        self.requests_cpu.merge_from(other.requests_cpu);
        self.requests_memory.merge_from(other.requests_memory);
        self.limits_cpu.merge_from(other.limits_cpu);
        self.limits_memory.merge_from(other.limits_memory);
    }
}

#[derive(
    Clone,
    Debug,
    PartialEq,
    k8s_openapi_derive::CustomResourceDefinition,
    schemars::JsonSchema,
    serde::Deserialize,
    serde::Serialize,
)]
#[custom_resource_definition(
    group = "management.cattle.io",
    version = "v3",
    plural = "projects",
    generate_schema,
    namespaced,
    impl_deep_merge
)]
#[serde(rename_all = "camelCase")]
pub struct ProjectSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_quota: Option<ProjectResourceQuota>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_default_resource_quota: Option<NamespaceResourceQuota>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_default_resource_limit: Option<ContainerResourceLimit>,
    pub enable_project_monitoring: bool,
}

impl k8s_openapi::DeepMerge for ProjectSpec {
    fn merge_from(&mut self, other: Self)
    where
        Self: Sized,
    {
        self.display_name.merge_from(other.display_name);
        self.description.merge_from(other.description);
        self.cluster_name.merge_from(other.cluster_name);
        self.resource_quota.merge_from(other.resource_quota);
        self.namespace_default_resource_quota
            .merge_from(other.namespace_default_resource_quota);
        self.container_default_resource_limit
            .merge_from(other.container_default_resource_limit);
        self.enable_project_monitoring
            .merge_from(other.enable_project_monitoring);
    }
}
