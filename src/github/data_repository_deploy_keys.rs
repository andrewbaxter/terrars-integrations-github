use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataRepositoryDeployKeysData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    repository: PrimField<String>,
}

struct DataRepositoryDeployKeys_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRepositoryDeployKeysData>,
}

#[derive(Clone)]
pub struct DataRepositoryDeployKeys(Rc<DataRepositoryDeployKeys_>);

impl DataRepositoryDeployKeys {
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

    #[doc= "Get a reference to the value of field `keys` after provisioning.\n"]
    pub fn keys(&self) -> ListRef<DataRepositoryDeployKeysKeysElRef> {
        ListRef::new(self.shared().clone(), format!("{}.keys", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }
}

impl Referable for DataRepositoryDeployKeys {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataRepositoryDeployKeys { }

impl ToListMappable for DataRepositoryDeployKeys {
    type O = ListRef<DataRepositoryDeployKeysRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataRepositoryDeployKeys_ {
    fn extract_datasource_type(&self) -> String {
        "github_repository_deploy_keys".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRepositoryDeployKeys {
    pub tf_id: String,
    #[doc= ""]
    pub repository: PrimField<String>,
}

impl BuildDataRepositoryDeployKeys {
    pub fn build(self, stack: &mut Stack) -> DataRepositoryDeployKeys {
        let out = DataRepositoryDeployKeys(Rc::new(DataRepositoryDeployKeys_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRepositoryDeployKeysData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                repository: self.repository,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRepositoryDeployKeysRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRepositoryDeployKeysRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRepositoryDeployKeysRef {
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

    #[doc= "Get a reference to the value of field `keys` after provisioning.\n"]
    pub fn keys(&self) -> ListRef<DataRepositoryDeployKeysKeysElRef> {
        ListRef::new(self.shared().clone(), format!("{}.keys", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataRepositoryDeployKeysKeysEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verified: Option<PrimField<bool>>,
}

impl DataRepositoryDeployKeysKeysEl {
    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `title`.\n"]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }

    #[doc= "Set the field `verified`.\n"]
    pub fn set_verified(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.verified = Some(v.into());
        self
    }
}

impl ToListMappable for DataRepositoryDeployKeysKeysEl {
    type O = BlockAssignable<DataRepositoryDeployKeysKeysEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRepositoryDeployKeysKeysEl {}

impl BuildDataRepositoryDeployKeysKeysEl {
    pub fn build(self) -> DataRepositoryDeployKeysKeysEl {
        DataRepositoryDeployKeysKeysEl {
            id: core::default::Default::default(),
            key: core::default::Default::default(),
            title: core::default::Default::default(),
            verified: core::default::Default::default(),
        }
    }
}

pub struct DataRepositoryDeployKeysKeysElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRepositoryDeployKeysKeysElRef {
    fn new(shared: StackShared, base: String) -> DataRepositoryDeployKeysKeysElRef {
        DataRepositoryDeployKeysKeysElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRepositoryDeployKeysKeysElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }

    #[doc= "Get a reference to the value of field `verified` after provisioning.\n"]
    pub fn verified(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.verified", self.base))
    }
}
