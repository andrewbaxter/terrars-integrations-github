use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataActionsVariablesData {
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

struct DataActionsVariables_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataActionsVariablesData>,
}

#[derive(Clone)]
pub struct DataActionsVariables(Rc<DataActionsVariables_>);

impl DataActionsVariables {
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

    #[doc= "Set the field `full_name`.\n"]
    pub fn set_full_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().full_name = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `full_name` after provisioning.\n"]
    pub fn full_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.full_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `variables` after provisioning.\n"]
    pub fn variables(&self) -> ListRef<DataActionsVariablesVariablesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.variables", self.extract_ref()))
    }
}

impl Referable for DataActionsVariables {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataActionsVariables { }

impl ToListMappable for DataActionsVariables {
    type O = ListRef<DataActionsVariablesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataActionsVariables_ {
    fn extract_datasource_type(&self) -> String {
        "github_actions_variables".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataActionsVariables {
    pub tf_id: String,
}

impl BuildDataActionsVariables {
    pub fn build(self, stack: &mut Stack) -> DataActionsVariables {
        let out = DataActionsVariables(Rc::new(DataActionsVariables_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataActionsVariablesData {
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

pub struct DataActionsVariablesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataActionsVariablesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataActionsVariablesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `full_name` after provisioning.\n"]
    pub fn full_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.full_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `variables` after provisioning.\n"]
    pub fn variables(&self) -> ListRef<DataActionsVariablesVariablesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.variables", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataActionsVariablesVariablesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataActionsVariablesVariablesEl {
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

    #[doc= "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataActionsVariablesVariablesEl {
    type O = BlockAssignable<DataActionsVariablesVariablesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataActionsVariablesVariablesEl {}

impl BuildDataActionsVariablesVariablesEl {
    pub fn build(self) -> DataActionsVariablesVariablesEl {
        DataActionsVariablesVariablesEl {
            created_at: core::default::Default::default(),
            name: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataActionsVariablesVariablesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataActionsVariablesVariablesElRef {
    fn new(shared: StackShared, base: String) -> DataActionsVariablesVariablesElRef {
        DataActionsVariablesVariablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataActionsVariablesVariablesElRef {
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

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
