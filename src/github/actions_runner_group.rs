use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct ActionsRunnerGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allows_public_repositories: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restricted_to_workflows: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selected_repository_ids: Option<SetField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selected_workflows: Option<ListField<PrimField<String>>>,
    visibility: PrimField<String>,
}

struct ActionsRunnerGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ActionsRunnerGroupData>,
}

#[derive(Clone)]
pub struct ActionsRunnerGroup(Rc<ActionsRunnerGroup_>);

impl ActionsRunnerGroup {
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

    #[doc= "Set the field `allows_public_repositories`.\nWhether public repositories can be added to the runner group."]
    pub fn set_allows_public_repositories(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allows_public_repositories = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `restricted_to_workflows`.\nIf 'true', the runner group will be restricted to running only the workflows specified in the 'selected_workflows' array. Defaults to 'false'."]
    pub fn set_restricted_to_workflows(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().restricted_to_workflows = Some(v.into());
        self
    }

    #[doc= "Set the field `selected_repository_ids`.\nList of repository IDs that can access the runner group."]
    pub fn set_selected_repository_ids(self, v: impl Into<SetField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().selected_repository_ids = Some(v.into());
        self
    }

    #[doc= "Set the field `selected_workflows`.\nList of workflows the runner group should be allowed to run. This setting will be ignored unless restricted_to_workflows is set to 'true'."]
    pub fn set_selected_workflows(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().selected_workflows = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `allows_public_repositories` after provisioning.\nWhether public repositories can be added to the runner group."]
    pub fn allows_public_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allows_public_repositories", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default` after provisioning.\nWhether this is the default runner group."]
    pub fn default(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.default", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\nAn etag representing the runner group object"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `inherited` after provisioning.\nWhether the runner group is inherited from the enterprise level"]
    pub fn inherited(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.inherited", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the runner group."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `restricted_to_workflows` after provisioning.\nIf 'true', the runner group will be restricted to running only the workflows specified in the 'selected_workflows' array. Defaults to 'false'."]
    pub fn restricted_to_workflows(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.restricted_to_workflows", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `runners_url` after provisioning.\nThe GitHub API URL for the runner group's runners."]
    pub fn runners_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.runners_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `selected_repositories_url` after provisioning.\nGitHub API URL for the runner group's repositories."]
    pub fn selected_repositories_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.selected_repositories_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `selected_repository_ids` after provisioning.\nList of repository IDs that can access the runner group."]
    pub fn selected_repository_ids(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.selected_repository_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `selected_workflows` after provisioning.\nList of workflows the runner group should be allowed to run. This setting will be ignored unless restricted_to_workflows is set to 'true'."]
    pub fn selected_workflows(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.selected_workflows", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `visibility` after provisioning.\nThe visibility of the runner group."]
    pub fn visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.extract_ref()))
    }
}

impl Referable for ActionsRunnerGroup {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ActionsRunnerGroup { }

impl ToListMappable for ActionsRunnerGroup {
    type O = ListRef<ActionsRunnerGroupRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ActionsRunnerGroup_ {
    fn extract_resource_type(&self) -> String {
        "github_actions_runner_group".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildActionsRunnerGroup {
    pub tf_id: String,
    #[doc= "Name of the runner group."]
    pub name: PrimField<String>,
    #[doc= "The visibility of the runner group."]
    pub visibility: PrimField<String>,
}

impl BuildActionsRunnerGroup {
    pub fn build(self, stack: &mut Stack) -> ActionsRunnerGroup {
        let out = ActionsRunnerGroup(Rc::new(ActionsRunnerGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ActionsRunnerGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                allows_public_repositories: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                restricted_to_workflows: core::default::Default::default(),
                selected_repository_ids: core::default::Default::default(),
                selected_workflows: core::default::Default::default(),
                visibility: self.visibility,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ActionsRunnerGroupRef {
    shared: StackShared,
    base: String,
}

impl Ref for ActionsRunnerGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ActionsRunnerGroupRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allows_public_repositories` after provisioning.\nWhether public repositories can be added to the runner group."]
    pub fn allows_public_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allows_public_repositories", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default` after provisioning.\nWhether this is the default runner group."]
    pub fn default(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.default", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\nAn etag representing the runner group object"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `inherited` after provisioning.\nWhether the runner group is inherited from the enterprise level"]
    pub fn inherited(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.inherited", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the runner group."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `restricted_to_workflows` after provisioning.\nIf 'true', the runner group will be restricted to running only the workflows specified in the 'selected_workflows' array. Defaults to 'false'."]
    pub fn restricted_to_workflows(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.restricted_to_workflows", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `runners_url` after provisioning.\nThe GitHub API URL for the runner group's runners."]
    pub fn runners_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.runners_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `selected_repositories_url` after provisioning.\nGitHub API URL for the runner group's repositories."]
    pub fn selected_repositories_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.selected_repositories_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `selected_repository_ids` after provisioning.\nList of repository IDs that can access the runner group."]
    pub fn selected_repository_ids(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.selected_repository_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `selected_workflows` after provisioning.\nList of workflows the runner group should be allowed to run. This setting will be ignored unless restricted_to_workflows is set to 'true'."]
    pub fn selected_workflows(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.selected_workflows", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `visibility` after provisioning.\nThe visibility of the runner group."]
    pub fn visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.extract_ref()))
    }
}
