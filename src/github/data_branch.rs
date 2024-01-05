use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataBranchData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    branch: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    repository: PrimField<String>,
}

struct DataBranch_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBranchData>,
}

#[derive(Clone)]
pub struct DataBranch(Rc<DataBranch_>);

impl DataBranch {
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

    #[doc= "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ref_` after provisioning.\n"]
    pub fn ref_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ref", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sha` after provisioning.\n"]
    pub fn sha(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sha", self.extract_ref()))
    }
}

impl Referable for DataBranch {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataBranch { }

impl ToListMappable for DataBranch {
    type O = ListRef<DataBranchRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataBranch_ {
    fn extract_datasource_type(&self) -> String {
        "github_branch".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBranch {
    pub tf_id: String,
    #[doc= ""]
    pub branch: PrimField<String>,
    #[doc= ""]
    pub repository: PrimField<String>,
}

impl BuildDataBranch {
    pub fn build(self, stack: &mut Stack) -> DataBranch {
        let out = DataBranch(Rc::new(DataBranch_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBranchData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                branch: self.branch,
                id: core::default::Default::default(),
                repository: self.repository,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBranchRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBranchRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBranchRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ref_` after provisioning.\n"]
    pub fn ref_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ref", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sha` after provisioning.\n"]
    pub fn sha(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sha", self.extract_ref()))
    }
}
