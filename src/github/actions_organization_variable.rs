use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct ActionsOrganizationVariableData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selected_repository_ids: Option<SetField<PrimField<f64>>>,
    value: PrimField<String>,
    variable_name: PrimField<String>,
    visibility: PrimField<String>,
}

struct ActionsOrganizationVariable_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ActionsOrganizationVariableData>,
}

#[derive(Clone)]
pub struct ActionsOrganizationVariable(Rc<ActionsOrganizationVariable_>);

impl ActionsOrganizationVariable {
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

    #[doc= "Set the field `selected_repository_ids`.\nAn array of repository ids that can access the organization variable."]
    pub fn set_selected_repository_ids(self, v: impl Into<SetField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().selected_repository_ids = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\nDate of 'actions_variable' creation."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `selected_repository_ids` after provisioning.\nAn array of repository ids that can access the organization variable."]
    pub fn selected_repository_ids(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.selected_repository_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\nDate of 'actions_variable' update."]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nValue of the variable."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `variable_name` after provisioning.\nName of the variable."]
    pub fn variable_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.variable_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `visibility` after provisioning.\nConfigures the access that repositories have to the organization variable. Must be one of 'all', 'private', or 'selected'. 'selected_repository_ids' is required if set to 'selected'."]
    pub fn visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.extract_ref()))
    }
}

impl Referable for ActionsOrganizationVariable {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ActionsOrganizationVariable { }

impl ToListMappable for ActionsOrganizationVariable {
    type O = ListRef<ActionsOrganizationVariableRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ActionsOrganizationVariable_ {
    fn extract_resource_type(&self) -> String {
        "github_actions_organization_variable".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildActionsOrganizationVariable {
    pub tf_id: String,
    #[doc= "Value of the variable."]
    pub value: PrimField<String>,
    #[doc= "Name of the variable."]
    pub variable_name: PrimField<String>,
    #[doc= "Configures the access that repositories have to the organization variable. Must be one of 'all', 'private', or 'selected'. 'selected_repository_ids' is required if set to 'selected'."]
    pub visibility: PrimField<String>,
}

impl BuildActionsOrganizationVariable {
    pub fn build(self, stack: &mut Stack) -> ActionsOrganizationVariable {
        let out = ActionsOrganizationVariable(Rc::new(ActionsOrganizationVariable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ActionsOrganizationVariableData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                selected_repository_ids: core::default::Default::default(),
                value: self.value,
                variable_name: self.variable_name,
                visibility: self.visibility,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ActionsOrganizationVariableRef {
    shared: StackShared,
    base: String,
}

impl Ref for ActionsOrganizationVariableRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ActionsOrganizationVariableRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\nDate of 'actions_variable' creation."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `selected_repository_ids` after provisioning.\nAn array of repository ids that can access the organization variable."]
    pub fn selected_repository_ids(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.selected_repository_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\nDate of 'actions_variable' update."]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nValue of the variable."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `variable_name` after provisioning.\nName of the variable."]
    pub fn variable_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.variable_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `visibility` after provisioning.\nConfigures the access that repositories have to the organization variable. Must be one of 'all', 'private', or 'selected'. 'selected_repository_ids' is required if set to 'selected'."]
    pub fn visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.extract_ref()))
    }
}
