use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataRepositoryAutolinkReferencesData {
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

struct DataRepositoryAutolinkReferences_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRepositoryAutolinkReferencesData>,
}

#[derive(Clone)]
pub struct DataRepositoryAutolinkReferences(Rc<DataRepositoryAutolinkReferences_>);

impl DataRepositoryAutolinkReferences {
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

    #[doc= "Get a reference to the value of field `autolink_references` after provisioning.\n"]
    pub fn autolink_references(&self) -> ListRef<DataRepositoryAutolinkReferencesAutolinkReferencesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.autolink_references", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }
}

impl Referable for DataRepositoryAutolinkReferences {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataRepositoryAutolinkReferences { }

impl ToListMappable for DataRepositoryAutolinkReferences {
    type O = ListRef<DataRepositoryAutolinkReferencesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataRepositoryAutolinkReferences_ {
    fn extract_datasource_type(&self) -> String {
        "github_repository_autolink_references".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRepositoryAutolinkReferences {
    pub tf_id: String,
    #[doc= ""]
    pub repository: PrimField<String>,
}

impl BuildDataRepositoryAutolinkReferences {
    pub fn build(self, stack: &mut Stack) -> DataRepositoryAutolinkReferences {
        let out = DataRepositoryAutolinkReferences(Rc::new(DataRepositoryAutolinkReferences_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRepositoryAutolinkReferencesData {
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

pub struct DataRepositoryAutolinkReferencesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRepositoryAutolinkReferencesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRepositoryAutolinkReferencesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `autolink_references` after provisioning.\n"]
    pub fn autolink_references(&self) -> ListRef<DataRepositoryAutolinkReferencesAutolinkReferencesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.autolink_references", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataRepositoryAutolinkReferencesAutolinkReferencesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    is_alphanumeric: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_url_template: Option<PrimField<String>>,
}

impl DataRepositoryAutolinkReferencesAutolinkReferencesEl {
    #[doc= "Set the field `is_alphanumeric`.\n"]
    pub fn set_is_alphanumeric(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_alphanumeric = Some(v.into());
        self
    }

    #[doc= "Set the field `key_prefix`.\n"]
    pub fn set_key_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_prefix = Some(v.into());
        self
    }

    #[doc= "Set the field `target_url_template`.\n"]
    pub fn set_target_url_template(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_url_template = Some(v.into());
        self
    }
}

impl ToListMappable for DataRepositoryAutolinkReferencesAutolinkReferencesEl {
    type O = BlockAssignable<DataRepositoryAutolinkReferencesAutolinkReferencesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRepositoryAutolinkReferencesAutolinkReferencesEl {}

impl BuildDataRepositoryAutolinkReferencesAutolinkReferencesEl {
    pub fn build(self) -> DataRepositoryAutolinkReferencesAutolinkReferencesEl {
        DataRepositoryAutolinkReferencesAutolinkReferencesEl {
            is_alphanumeric: core::default::Default::default(),
            key_prefix: core::default::Default::default(),
            target_url_template: core::default::Default::default(),
        }
    }
}

pub struct DataRepositoryAutolinkReferencesAutolinkReferencesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRepositoryAutolinkReferencesAutolinkReferencesElRef {
    fn new(shared: StackShared, base: String) -> DataRepositoryAutolinkReferencesAutolinkReferencesElRef {
        DataRepositoryAutolinkReferencesAutolinkReferencesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRepositoryAutolinkReferencesAutolinkReferencesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `is_alphanumeric` after provisioning.\n"]
    pub fn is_alphanumeric(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_alphanumeric", self.base))
    }

    #[doc= "Get a reference to the value of field `key_prefix` after provisioning.\n"]
    pub fn key_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_prefix", self.base))
    }

    #[doc= "Get a reference to the value of field `target_url_template` after provisioning.\n"]
    pub fn target_url_template(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_url_template", self.base))
    }
}
