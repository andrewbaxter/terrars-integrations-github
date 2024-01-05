use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataCodespacesSecretsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    full_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

struct DataCodespacesSecrets_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCodespacesSecretsData>,
}

#[derive(Clone)]
pub struct DataCodespacesSecrets(Rc<DataCodespacesSecrets_>);

impl DataCodespacesSecrets {
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

    #[doc= "Set the field `full_name`.\nFull name of the repository (in `org/name` format)."]
    pub fn set_full_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().full_name = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\nThe name of the repository."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `full_name` after provisioning.\nFull name of the repository (in `org/name` format)."]
    pub fn full_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.full_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the repository."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `secrets` after provisioning.\n"]
    pub fn secrets(&self) -> ListRef<DataCodespacesSecretsSecretsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.secrets", self.extract_ref()))
    }
}

impl Referable for DataCodespacesSecrets {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataCodespacesSecrets { }

impl ToListMappable for DataCodespacesSecrets {
    type O = ListRef<DataCodespacesSecretsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataCodespacesSecrets_ {
    fn extract_datasource_type(&self) -> String {
        "github_codespaces_secrets".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCodespacesSecrets {
    pub tf_id: String,
}

impl BuildDataCodespacesSecrets {
    pub fn build(self, stack: &mut Stack) -> DataCodespacesSecrets {
        let out = DataCodespacesSecrets(Rc::new(DataCodespacesSecrets_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCodespacesSecretsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                full_name: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCodespacesSecretsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCodespacesSecretsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCodespacesSecretsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `full_name` after provisioning.\nFull name of the repository (in `org/name` format)."]
    pub fn full_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.full_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the repository."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `secrets` after provisioning.\n"]
    pub fn secrets(&self) -> ListRef<DataCodespacesSecretsSecretsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.secrets", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCodespacesSecretsSecretsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<PrimField<String>>,
}

impl DataCodespacesSecretsSecretsEl {
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
}

impl ToListMappable for DataCodespacesSecretsSecretsEl {
    type O = BlockAssignable<DataCodespacesSecretsSecretsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCodespacesSecretsSecretsEl {}

impl BuildDataCodespacesSecretsSecretsEl {
    pub fn build(self) -> DataCodespacesSecretsSecretsEl {
        DataCodespacesSecretsSecretsEl {
            created_at: core::default::Default::default(),
            name: core::default::Default::default(),
            updated_at: core::default::Default::default(),
        }
    }
}

pub struct DataCodespacesSecretsSecretsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCodespacesSecretsSecretsElRef {
    fn new(shared: StackShared, base: String) -> DataCodespacesSecretsSecretsElRef {
        DataCodespacesSecretsSecretsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCodespacesSecretsSecretsElRef {
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
}
