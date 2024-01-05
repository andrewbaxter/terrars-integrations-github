use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataBranchProtectionRulesData {
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

struct DataBranchProtectionRules_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBranchProtectionRulesData>,
}

#[derive(Clone)]
pub struct DataBranchProtectionRules(Rc<DataBranchProtectionRules_>);

impl DataBranchProtectionRules {
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

    #[doc= "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<DataBranchProtectionRulesRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }
}

impl Referable for DataBranchProtectionRules {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataBranchProtectionRules { }

impl ToListMappable for DataBranchProtectionRules {
    type O = ListRef<DataBranchProtectionRulesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataBranchProtectionRules_ {
    fn extract_datasource_type(&self) -> String {
        "github_branch_protection_rules".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBranchProtectionRules {
    pub tf_id: String,
    #[doc= ""]
    pub repository: PrimField<String>,
}

impl BuildDataBranchProtectionRules {
    pub fn build(self, stack: &mut Stack) -> DataBranchProtectionRules {
        let out = DataBranchProtectionRules(Rc::new(DataBranchProtectionRules_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBranchProtectionRulesData {
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

pub struct DataBranchProtectionRulesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBranchProtectionRulesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBranchProtectionRulesRef {
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

    #[doc= "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<DataBranchProtectionRulesRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBranchProtectionRulesRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pattern: Option<PrimField<String>>,
}

impl DataBranchProtectionRulesRulesEl {
    #[doc= "Set the field `pattern`.\n"]
    pub fn set_pattern(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pattern = Some(v.into());
        self
    }
}

impl ToListMappable for DataBranchProtectionRulesRulesEl {
    type O = BlockAssignable<DataBranchProtectionRulesRulesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBranchProtectionRulesRulesEl {}

impl BuildDataBranchProtectionRulesRulesEl {
    pub fn build(self) -> DataBranchProtectionRulesRulesEl {
        DataBranchProtectionRulesRulesEl { pattern: core::default::Default::default() }
    }
}

pub struct DataBranchProtectionRulesRulesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBranchProtectionRulesRulesElRef {
    fn new(shared: StackShared, base: String) -> DataBranchProtectionRulesRulesElRef {
        DataBranchProtectionRulesRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBranchProtectionRulesRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `pattern` after provisioning.\n"]
    pub fn pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }
}
