use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataActionsOrganizationVariablesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

struct DataActionsOrganizationVariables_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataActionsOrganizationVariablesData>,
}

#[derive(Clone)]
pub struct DataActionsOrganizationVariables(Rc<DataActionsOrganizationVariables_>);

impl DataActionsOrganizationVariables {
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

    #[doc= "Get a reference to the value of field `variables` after provisioning.\n"]
    pub fn variables(&self) -> ListRef<DataActionsOrganizationVariablesVariablesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.variables", self.extract_ref()))
    }
}

impl Referable for DataActionsOrganizationVariables {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataActionsOrganizationVariables { }

impl ToListMappable for DataActionsOrganizationVariables {
    type O = ListRef<DataActionsOrganizationVariablesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataActionsOrganizationVariables_ {
    fn extract_datasource_type(&self) -> String {
        "github_actions_organization_variables".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataActionsOrganizationVariables {
    pub tf_id: String,
}

impl BuildDataActionsOrganizationVariables {
    pub fn build(self, stack: &mut Stack) -> DataActionsOrganizationVariables {
        let out = DataActionsOrganizationVariables(Rc::new(DataActionsOrganizationVariables_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataActionsOrganizationVariablesData {
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

pub struct DataActionsOrganizationVariablesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataActionsOrganizationVariablesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataActionsOrganizationVariablesRef {
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

    #[doc= "Get a reference to the value of field `variables` after provisioning.\n"]
    pub fn variables(&self) -> ListRef<DataActionsOrganizationVariablesVariablesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.variables", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataActionsOrganizationVariablesVariablesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<PrimField<String>>,
}

impl DataActionsOrganizationVariablesVariablesEl {
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

    #[doc= "Set the field `visibility`.\n"]
    pub fn set_visibility(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.visibility = Some(v.into());
        self
    }
}

impl ToListMappable for DataActionsOrganizationVariablesVariablesEl {
    type O = BlockAssignable<DataActionsOrganizationVariablesVariablesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataActionsOrganizationVariablesVariablesEl {}

impl BuildDataActionsOrganizationVariablesVariablesEl {
    pub fn build(self) -> DataActionsOrganizationVariablesVariablesEl {
        DataActionsOrganizationVariablesVariablesEl {
            created_at: core::default::Default::default(),
            name: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            value: core::default::Default::default(),
            visibility: core::default::Default::default(),
        }
    }
}

pub struct DataActionsOrganizationVariablesVariablesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataActionsOrganizationVariablesVariablesElRef {
    fn new(shared: StackShared, base: String) -> DataActionsOrganizationVariablesVariablesElRef {
        DataActionsOrganizationVariablesVariablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataActionsOrganizationVariablesVariablesElRef {
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

    #[doc= "Get a reference to the value of field `visibility` after provisioning.\n"]
    pub fn visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.base))
    }
}
