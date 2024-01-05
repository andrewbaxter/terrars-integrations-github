use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataRepositoryBranchesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    only_non_protected_branches: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    only_protected_branches: Option<PrimField<bool>>,
    repository: PrimField<String>,
}

struct DataRepositoryBranches_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRepositoryBranchesData>,
}

#[derive(Clone)]
pub struct DataRepositoryBranches(Rc<DataRepositoryBranches_>);

impl DataRepositoryBranches {
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

    #[doc= "Set the field `only_non_protected_branches`.\n"]
    pub fn set_only_non_protected_branches(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().only_non_protected_branches = Some(v.into());
        self
    }

    #[doc= "Set the field `only_protected_branches`.\n"]
    pub fn set_only_protected_branches(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().only_protected_branches = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `branches` after provisioning.\n"]
    pub fn branches(&self) -> ListRef<DataRepositoryBranchesBranchesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.branches", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `only_non_protected_branches` after provisioning.\n"]
    pub fn only_non_protected_branches(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.only_non_protected_branches", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `only_protected_branches` after provisioning.\n"]
    pub fn only_protected_branches(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.only_protected_branches", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }
}

impl Referable for DataRepositoryBranches {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataRepositoryBranches { }

impl ToListMappable for DataRepositoryBranches {
    type O = ListRef<DataRepositoryBranchesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataRepositoryBranches_ {
    fn extract_datasource_type(&self) -> String {
        "github_repository_branches".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRepositoryBranches {
    pub tf_id: String,
    #[doc= ""]
    pub repository: PrimField<String>,
}

impl BuildDataRepositoryBranches {
    pub fn build(self, stack: &mut Stack) -> DataRepositoryBranches {
        let out = DataRepositoryBranches(Rc::new(DataRepositoryBranches_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRepositoryBranchesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                only_non_protected_branches: core::default::Default::default(),
                only_protected_branches: core::default::Default::default(),
                repository: self.repository,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRepositoryBranchesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRepositoryBranchesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRepositoryBranchesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `branches` after provisioning.\n"]
    pub fn branches(&self) -> ListRef<DataRepositoryBranchesBranchesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.branches", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `only_non_protected_branches` after provisioning.\n"]
    pub fn only_non_protected_branches(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.only_non_protected_branches", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `only_protected_branches` after provisioning.\n"]
    pub fn only_protected_branches(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.only_protected_branches", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataRepositoryBranchesBranchesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protected: Option<PrimField<bool>>,
}

impl DataRepositoryBranchesBranchesEl {
    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `protected`.\n"]
    pub fn set_protected(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.protected = Some(v.into());
        self
    }
}

impl ToListMappable for DataRepositoryBranchesBranchesEl {
    type O = BlockAssignable<DataRepositoryBranchesBranchesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRepositoryBranchesBranchesEl {}

impl BuildDataRepositoryBranchesBranchesEl {
    pub fn build(self) -> DataRepositoryBranchesBranchesEl {
        DataRepositoryBranchesBranchesEl {
            name: core::default::Default::default(),
            protected: core::default::Default::default(),
        }
    }
}

pub struct DataRepositoryBranchesBranchesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRepositoryBranchesBranchesElRef {
    fn new(shared: StackShared, base: String) -> DataRepositoryBranchesBranchesElRef {
        DataRepositoryBranchesBranchesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRepositoryBranchesBranchesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `protected` after provisioning.\n"]
    pub fn protected(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.protected", self.base))
    }
}
