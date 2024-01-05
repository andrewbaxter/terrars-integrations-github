use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct ActionsRepositoryPermissionsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_actions: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    repository: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_actions_config: Option<Vec<ActionsRepositoryPermissionsAllowedActionsConfigEl>>,
    dynamic: ActionsRepositoryPermissionsDynamic,
}

struct ActionsRepositoryPermissions_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ActionsRepositoryPermissionsData>,
}

#[derive(Clone)]
pub struct ActionsRepositoryPermissions(Rc<ActionsRepositoryPermissions_>);

impl ActionsRepositoryPermissions {
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

    #[doc= "Set the field `allowed_actions`.\nThe permissions policy that controls the actions that are allowed to run. Can be one of: 'all', 'local_only', or 'selected'."]
    pub fn set_allowed_actions(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().allowed_actions = Some(v.into());
        self
    }

    #[doc= "Set the field `enabled`.\nShould GitHub actions be enabled on this repository."]
    pub fn set_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `allowed_actions_config`.\n"]
    pub fn set_allowed_actions_config(
        self,
        v: impl Into<BlockAssignable<ActionsRepositoryPermissionsAllowedActionsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().allowed_actions_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.allowed_actions_config = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `allowed_actions` after provisioning.\nThe permissions policy that controls the actions that are allowed to run. Can be one of: 'all', 'local_only', or 'selected'."]
    pub fn allowed_actions(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.allowed_actions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nShould GitHub actions be enabled on this repository."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\nThe GitHub repository."]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allowed_actions_config` after provisioning.\n"]
    pub fn allowed_actions_config(&self) -> ListRef<ActionsRepositoryPermissionsAllowedActionsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.allowed_actions_config", self.extract_ref()))
    }
}

impl Referable for ActionsRepositoryPermissions {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ActionsRepositoryPermissions { }

impl ToListMappable for ActionsRepositoryPermissions {
    type O = ListRef<ActionsRepositoryPermissionsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ActionsRepositoryPermissions_ {
    fn extract_resource_type(&self) -> String {
        "github_actions_repository_permissions".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildActionsRepositoryPermissions {
    pub tf_id: String,
    #[doc= "The GitHub repository."]
    pub repository: PrimField<String>,
}

impl BuildActionsRepositoryPermissions {
    pub fn build(self, stack: &mut Stack) -> ActionsRepositoryPermissions {
        let out = ActionsRepositoryPermissions(Rc::new(ActionsRepositoryPermissions_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ActionsRepositoryPermissionsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                allowed_actions: core::default::Default::default(),
                enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                repository: self.repository,
                allowed_actions_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ActionsRepositoryPermissionsRef {
    shared: StackShared,
    base: String,
}

impl Ref for ActionsRepositoryPermissionsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ActionsRepositoryPermissionsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allowed_actions` after provisioning.\nThe permissions policy that controls the actions that are allowed to run. Can be one of: 'all', 'local_only', or 'selected'."]
    pub fn allowed_actions(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.allowed_actions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nShould GitHub actions be enabled on this repository."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\nThe GitHub repository."]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allowed_actions_config` after provisioning.\n"]
    pub fn allowed_actions_config(&self) -> ListRef<ActionsRepositoryPermissionsAllowedActionsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.allowed_actions_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ActionsRepositoryPermissionsAllowedActionsConfigEl {
    github_owned_allowed: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    patterns_allowed: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verified_allowed: Option<PrimField<bool>>,
}

impl ActionsRepositoryPermissionsAllowedActionsConfigEl {
    #[doc= "Set the field `patterns_allowed`.\nSpecifies a list of string-matching patterns to allow specific action(s). Wildcards, tags, and SHAs are allowed. For example, 'monalisa/octocat@', 'monalisa/octocat@v2', 'monalisa/'."]
    pub fn set_patterns_allowed(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.patterns_allowed = Some(v.into());
        self
    }

    #[doc= "Set the field `verified_allowed`.\nWhether actions in GitHub Marketplace from verified creators are allowed. Set to 'true' to allow all GitHub Marketplace actions by verified creators."]
    pub fn set_verified_allowed(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.verified_allowed = Some(v.into());
        self
    }
}

impl ToListMappable for ActionsRepositoryPermissionsAllowedActionsConfigEl {
    type O = BlockAssignable<ActionsRepositoryPermissionsAllowedActionsConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildActionsRepositoryPermissionsAllowedActionsConfigEl {
    #[doc= "Whether GitHub-owned actions are allowed in the repository."]
    pub github_owned_allowed: PrimField<bool>,
}

impl BuildActionsRepositoryPermissionsAllowedActionsConfigEl {
    pub fn build(self) -> ActionsRepositoryPermissionsAllowedActionsConfigEl {
        ActionsRepositoryPermissionsAllowedActionsConfigEl {
            github_owned_allowed: self.github_owned_allowed,
            patterns_allowed: core::default::Default::default(),
            verified_allowed: core::default::Default::default(),
        }
    }
}

pub struct ActionsRepositoryPermissionsAllowedActionsConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ActionsRepositoryPermissionsAllowedActionsConfigElRef {
    fn new(shared: StackShared, base: String) -> ActionsRepositoryPermissionsAllowedActionsConfigElRef {
        ActionsRepositoryPermissionsAllowedActionsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ActionsRepositoryPermissionsAllowedActionsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `github_owned_allowed` after provisioning.\nWhether GitHub-owned actions are allowed in the repository."]
    pub fn github_owned_allowed(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.github_owned_allowed", self.base))
    }

    #[doc= "Get a reference to the value of field `patterns_allowed` after provisioning.\nSpecifies a list of string-matching patterns to allow specific action(s). Wildcards, tags, and SHAs are allowed. For example, 'monalisa/octocat@', 'monalisa/octocat@v2', 'monalisa/'."]
    pub fn patterns_allowed(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.patterns_allowed", self.base))
    }

    #[doc= "Get a reference to the value of field `verified_allowed` after provisioning.\nWhether actions in GitHub Marketplace from verified creators are allowed. Set to 'true' to allow all GitHub Marketplace actions by verified creators."]
    pub fn verified_allowed(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.verified_allowed", self.base))
    }
}

#[derive(Serialize, Default)]
struct ActionsRepositoryPermissionsDynamic {
    allowed_actions_config: Option<DynamicBlock<ActionsRepositoryPermissionsAllowedActionsConfigEl>>,
}
