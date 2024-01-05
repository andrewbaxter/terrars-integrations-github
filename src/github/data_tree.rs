use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataTreeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recursive: Option<PrimField<bool>>,
    repository: PrimField<String>,
    tree_sha: PrimField<String>,
}

struct DataTree_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataTreeData>,
}

#[derive(Clone)]
pub struct DataTree(Rc<DataTree_>);

impl DataTree {
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

    #[doc= "Set the field `recursive`.\n"]
    pub fn set_recursive(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().recursive = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `entries` after provisioning.\n"]
    pub fn entries(&self) -> ListRef<DataTreeEntriesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.entries", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `recursive` after provisioning.\n"]
    pub fn recursive(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.recursive", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tree_sha` after provisioning.\n"]
    pub fn tree_sha(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tree_sha", self.extract_ref()))
    }
}

impl Referable for DataTree {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataTree { }

impl ToListMappable for DataTree {
    type O = ListRef<DataTreeRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataTree_ {
    fn extract_datasource_type(&self) -> String {
        "github_tree".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataTree {
    pub tf_id: String,
    #[doc= ""]
    pub repository: PrimField<String>,
    #[doc= ""]
    pub tree_sha: PrimField<String>,
}

impl BuildDataTree {
    pub fn build(self, stack: &mut Stack) -> DataTree {
        let out = DataTree(Rc::new(DataTree_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataTreeData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                recursive: core::default::Default::default(),
                repository: self.repository,
                tree_sha: self.tree_sha,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataTreeRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataTreeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataTreeRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `entries` after provisioning.\n"]
    pub fn entries(&self) -> ListRef<DataTreeEntriesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.entries", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `recursive` after provisioning.\n"]
    pub fn recursive(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.recursive", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tree_sha` after provisioning.\n"]
    pub fn tree_sha(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tree_sha", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataTreeEntriesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sha: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataTreeEntriesEl {
    #[doc= "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }

    #[doc= "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }

    #[doc= "Set the field `sha`.\n"]
    pub fn set_sha(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sha = Some(v.into());
        self
    }

    #[doc= "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataTreeEntriesEl {
    type O = BlockAssignable<DataTreeEntriesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataTreeEntriesEl {}

impl BuildDataTreeEntriesEl {
    pub fn build(self) -> DataTreeEntriesEl {
        DataTreeEntriesEl {
            mode: core::default::Default::default(),
            path: core::default::Default::default(),
            sha: core::default::Default::default(),
            size: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataTreeEntriesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataTreeEntriesElRef {
    fn new(shared: StackShared, base: String) -> DataTreeEntriesElRef {
        DataTreeEntriesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataTreeEntriesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc= "Get a reference to the value of field `sha` after provisioning.\n"]
    pub fn sha(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sha", self.base))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
