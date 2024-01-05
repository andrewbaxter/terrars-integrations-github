use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct RepositoryCollaboratorsData {
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
    repository: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    team: Option<Vec<RepositoryCollaboratorsTeamEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<Vec<RepositoryCollaboratorsUserEl>>,
    dynamic: RepositoryCollaboratorsDynamic,
}

struct RepositoryCollaborators_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RepositoryCollaboratorsData>,
}

#[derive(Clone)]
pub struct RepositoryCollaborators(Rc<RepositoryCollaborators_>);

impl RepositoryCollaborators {
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

    #[doc= "Set the field `team`.\n"]
    pub fn set_team(self, v: impl Into<BlockAssignable<RepositoryCollaboratorsTeamEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().team = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.team = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `user`.\n"]
    pub fn set_user(self, v: impl Into<BlockAssignable<RepositoryCollaboratorsUserEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().user = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.user = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `invitation_ids` after provisioning.\nMap of usernames to invitation ID for any users added"]
    pub fn invitation_ids(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.invitation_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }
}

impl Referable for RepositoryCollaborators {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for RepositoryCollaborators { }

impl ToListMappable for RepositoryCollaborators {
    type O = ListRef<RepositoryCollaboratorsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for RepositoryCollaborators_ {
    fn extract_resource_type(&self) -> String {
        "github_repository_collaborators".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRepositoryCollaborators {
    pub tf_id: String,
    #[doc= ""]
    pub repository: PrimField<String>,
}

impl BuildRepositoryCollaborators {
    pub fn build(self, stack: &mut Stack) -> RepositoryCollaborators {
        let out = RepositoryCollaborators(Rc::new(RepositoryCollaborators_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RepositoryCollaboratorsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                repository: self.repository,
                team: core::default::Default::default(),
                user: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RepositoryCollaboratorsRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositoryCollaboratorsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RepositoryCollaboratorsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `invitation_ids` after provisioning.\nMap of usernames to invitation ID for any users added"]
    pub fn invitation_ids(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.invitation_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct RepositoryCollaboratorsTeamEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    permission: Option<PrimField<String>>,
    team_id: PrimField<String>,
}

impl RepositoryCollaboratorsTeamEl {
    #[doc= "Set the field `permission`.\n"]
    pub fn set_permission(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.permission = Some(v.into());
        self
    }
}

impl ToListMappable for RepositoryCollaboratorsTeamEl {
    type O = BlockAssignable<RepositoryCollaboratorsTeamEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRepositoryCollaboratorsTeamEl {
    #[doc= "Team ID or slug to add to the repository as a collaborator."]
    pub team_id: PrimField<String>,
}

impl BuildRepositoryCollaboratorsTeamEl {
    pub fn build(self) -> RepositoryCollaboratorsTeamEl {
        RepositoryCollaboratorsTeamEl {
            permission: core::default::Default::default(),
            team_id: self.team_id,
        }
    }
}

pub struct RepositoryCollaboratorsTeamElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositoryCollaboratorsTeamElRef {
    fn new(shared: StackShared, base: String) -> RepositoryCollaboratorsTeamElRef {
        RepositoryCollaboratorsTeamElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RepositoryCollaboratorsTeamElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `permission` after provisioning.\n"]
    pub fn permission(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.permission", self.base))
    }

    #[doc= "Get a reference to the value of field `team_id` after provisioning.\nTeam ID or slug to add to the repository as a collaborator."]
    pub fn team_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.team_id", self.base))
    }
}

#[derive(Serialize)]
pub struct RepositoryCollaboratorsUserEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    permission: Option<PrimField<String>>,
    username: PrimField<String>,
}

impl RepositoryCollaboratorsUserEl {
    #[doc= "Set the field `permission`.\n"]
    pub fn set_permission(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.permission = Some(v.into());
        self
    }
}

impl ToListMappable for RepositoryCollaboratorsUserEl {
    type O = BlockAssignable<RepositoryCollaboratorsUserEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRepositoryCollaboratorsUserEl {
    #[doc= "(Required) The user to add to the repository as a collaborator."]
    pub username: PrimField<String>,
}

impl BuildRepositoryCollaboratorsUserEl {
    pub fn build(self) -> RepositoryCollaboratorsUserEl {
        RepositoryCollaboratorsUserEl {
            permission: core::default::Default::default(),
            username: self.username,
        }
    }
}

pub struct RepositoryCollaboratorsUserElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositoryCollaboratorsUserElRef {
    fn new(shared: StackShared, base: String) -> RepositoryCollaboratorsUserElRef {
        RepositoryCollaboratorsUserElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RepositoryCollaboratorsUserElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `permission` after provisioning.\n"]
    pub fn permission(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.permission", self.base))
    }

    #[doc= "Get a reference to the value of field `username` after provisioning.\n(Required) The user to add to the repository as a collaborator."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize, Default)]
struct RepositoryCollaboratorsDynamic {
    team: Option<DynamicBlock<RepositoryCollaboratorsTeamEl>>,
    user: Option<DynamicBlock<RepositoryCollaboratorsUserEl>>,
}
