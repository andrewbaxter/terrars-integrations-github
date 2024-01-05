use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataRepositoryDeploymentBranchPoliciesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    environment_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    repository: PrimField<String>,
}

struct DataRepositoryDeploymentBranchPolicies_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRepositoryDeploymentBranchPoliciesData>,
}

#[derive(Clone)]
pub struct DataRepositoryDeploymentBranchPolicies(Rc<DataRepositoryDeploymentBranchPolicies_>);

impl DataRepositoryDeploymentBranchPolicies {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderGithub) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `deployment_branch_policies` after provisioning.\n"]
    pub fn deployment_branch_policies(
        &self,
    ) -> ListRef<DataRepositoryDeploymentBranchPoliciesDeploymentBranchPoliciesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deployment_branch_policies", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `environment_name` after provisioning.\nThe target environment name."]
    pub fn environment_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\nThe GitHub repository name."]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }
}

impl Referable for DataRepositoryDeploymentBranchPolicies {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataRepositoryDeploymentBranchPolicies { }

impl ToListMappable for DataRepositoryDeploymentBranchPolicies {
    type O = ListRef<DataRepositoryDeploymentBranchPoliciesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataRepositoryDeploymentBranchPolicies_ {
    fn extract_datasource_type(&self) -> String {
        "github_repository_deployment_branch_policies".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRepositoryDeploymentBranchPolicies {
    pub tf_id: String,
    #[doc= "The target environment name."]
    pub environment_name: PrimField<String>,
    #[doc= "The GitHub repository name."]
    pub repository: PrimField<String>,
}

impl BuildDataRepositoryDeploymentBranchPolicies {
    pub fn build(self, stack: &mut Stack) -> DataRepositoryDeploymentBranchPolicies {
        let out = DataRepositoryDeploymentBranchPolicies(Rc::new(DataRepositoryDeploymentBranchPolicies_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRepositoryDeploymentBranchPoliciesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                environment_name: self.environment_name,
                id: core::default::Default::default(),
                repository: self.repository,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRepositoryDeploymentBranchPoliciesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRepositoryDeploymentBranchPoliciesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRepositoryDeploymentBranchPoliciesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `deployment_branch_policies` after provisioning.\n"]
    pub fn deployment_branch_policies(
        &self,
    ) -> ListRef<DataRepositoryDeploymentBranchPoliciesDeploymentBranchPoliciesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deployment_branch_policies", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `environment_name` after provisioning.\nThe target environment name."]
    pub fn environment_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\nThe GitHub repository name."]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataRepositoryDeploymentBranchPoliciesDeploymentBranchPoliciesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl DataRepositoryDeploymentBranchPoliciesDeploymentBranchPoliciesEl {
    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for DataRepositoryDeploymentBranchPoliciesDeploymentBranchPoliciesEl {
    type O = BlockAssignable<DataRepositoryDeploymentBranchPoliciesDeploymentBranchPoliciesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRepositoryDeploymentBranchPoliciesDeploymentBranchPoliciesEl {}

impl BuildDataRepositoryDeploymentBranchPoliciesDeploymentBranchPoliciesEl {
    pub fn build(self) -> DataRepositoryDeploymentBranchPoliciesDeploymentBranchPoliciesEl {
        DataRepositoryDeploymentBranchPoliciesDeploymentBranchPoliciesEl {
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataRepositoryDeploymentBranchPoliciesDeploymentBranchPoliciesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRepositoryDeploymentBranchPoliciesDeploymentBranchPoliciesElRef {
    fn new(shared: StackShared, base: String) -> DataRepositoryDeploymentBranchPoliciesDeploymentBranchPoliciesElRef {
        DataRepositoryDeploymentBranchPoliciesDeploymentBranchPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRepositoryDeploymentBranchPoliciesDeploymentBranchPoliciesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
