use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct BranchData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    branch: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    repository: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_sha: Option<PrimField<String>>,
}

struct Branch_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BranchData>,
}

#[derive(Clone)]
pub struct Branch(Rc<Branch_>);

impl Branch {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderGithub) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `source_branch`.\nThe branch name to start from. Defaults to 'main'."]
    pub fn set_source_branch(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_branch = Some(v.into());
        self
    }

    #[doc= "Set the field `source_sha`.\nThe commit hash to start from. Defaults to the tip of 'source_branch'. If provided, 'source_branch' is ignored."]
    pub fn set_source_sha(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_sha = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\nThe repository branch to create."]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\nAn etag representing the Branch object."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ref_` after provisioning.\nA string representing a branch reference, in the form of 'refs/heads/<branch>'."]
    pub fn ref_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ref", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\nThe GitHub repository name."]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sha` after provisioning.\nA string storing the reference's HEAD commit's SHA1."]
    pub fn sha(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sha", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source_branch` after provisioning.\nThe branch name to start from. Defaults to 'main'."]
    pub fn source_branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_branch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source_sha` after provisioning.\nThe commit hash to start from. Defaults to the tip of 'source_branch'. If provided, 'source_branch' is ignored."]
    pub fn source_sha(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_sha", self.extract_ref()))
    }
}

impl Referable for Branch {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Branch { }

impl ToListMappable for Branch {
    type O = ListRef<BranchRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Branch_ {
    fn extract_resource_type(&self) -> String {
        "github_branch".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBranch {
    pub tf_id: String,
    #[doc= "The repository branch to create."]
    pub branch: PrimField<String>,
    #[doc= "The GitHub repository name."]
    pub repository: PrimField<String>,
}

impl BuildBranch {
    pub fn build(self, stack: &mut Stack) -> Branch {
        let out = Branch(Rc::new(Branch_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BranchData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                branch: self.branch,
                id: core::default::Default::default(),
                repository: self.repository,
                source_branch: core::default::Default::default(),
                source_sha: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BranchRef {
    shared: StackShared,
    base: String,
}

impl Ref for BranchRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BranchRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\nThe repository branch to create."]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\nAn etag representing the Branch object."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ref_` after provisioning.\nA string representing a branch reference, in the form of 'refs/heads/<branch>'."]
    pub fn ref_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ref", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\nThe GitHub repository name."]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sha` after provisioning.\nA string storing the reference's HEAD commit's SHA1."]
    pub fn sha(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sha", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source_branch` after provisioning.\nThe branch name to start from. Defaults to 'main'."]
    pub fn source_branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_branch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source_sha` after provisioning.\nThe commit hash to start from. Defaults to the tip of 'source_branch'. If provided, 'source_branch' is ignored."]
    pub fn source_sha(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_sha", self.extract_ref()))
    }
}
