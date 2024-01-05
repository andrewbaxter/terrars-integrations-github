use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataActionsOrganizationSecretsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

struct DataActionsOrganizationSecrets_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataActionsOrganizationSecretsData>,
}

#[derive(Clone)]
pub struct DataActionsOrganizationSecrets(Rc<DataActionsOrganizationSecrets_>);

impl DataActionsOrganizationSecrets {
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

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `secrets` after provisioning.\n"]
    pub fn secrets(&self) -> ListRef<DataActionsOrganizationSecretsSecretsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.secrets", self.extract_ref()))
    }
}

impl Referable for DataActionsOrganizationSecrets {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataActionsOrganizationSecrets { }

impl ToListMappable for DataActionsOrganizationSecrets {
    type O = ListRef<DataActionsOrganizationSecretsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataActionsOrganizationSecrets_ {
    fn extract_datasource_type(&self) -> String {
        "github_actions_organization_secrets".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataActionsOrganizationSecrets {
    pub tf_id: String,
}

impl BuildDataActionsOrganizationSecrets {
    pub fn build(self, stack: &mut Stack) -> DataActionsOrganizationSecrets {
        let out = DataActionsOrganizationSecrets(Rc::new(DataActionsOrganizationSecrets_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataActionsOrganizationSecretsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataActionsOrganizationSecretsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataActionsOrganizationSecretsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataActionsOrganizationSecretsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `secrets` after provisioning.\n"]
    pub fn secrets(&self) -> ListRef<DataActionsOrganizationSecretsSecretsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.secrets", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataActionsOrganizationSecretsSecretsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<PrimField<String>>,
}

impl DataActionsOrganizationSecretsSecretsEl {
    #[doc= "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created_at = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `updated_at`.\n"]
    pub fn set_updated_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.updated_at = Some(v.into());
        self
    }

    #[doc= "Set the field `visibility`.\n"]
    pub fn set_visibility(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.visibility = Some(v.into());
        self
    }
}

impl ToListMappable for DataActionsOrganizationSecretsSecretsEl {
    type O = BlockAssignable<DataActionsOrganizationSecretsSecretsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataActionsOrganizationSecretsSecretsEl {}

impl BuildDataActionsOrganizationSecretsSecretsEl {
    pub fn build(self) -> DataActionsOrganizationSecretsSecretsEl {
        DataActionsOrganizationSecretsSecretsEl {
            created_at: core::default::Default::default(),
            name: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            visibility: core::default::Default::default(),
        }
    }
}

pub struct DataActionsOrganizationSecretsSecretsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataActionsOrganizationSecretsSecretsElRef {
    fn new(shared: StackShared, base: String) -> DataActionsOrganizationSecretsSecretsElRef {
        DataActionsOrganizationSecretsSecretsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataActionsOrganizationSecretsSecretsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc= "Get a reference to the value of field `visibility` after provisioning.\n"]
    pub fn visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.base))
    }
}
