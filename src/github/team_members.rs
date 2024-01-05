use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct TeamMembersData {
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
    team_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    members: Option<Vec<TeamMembersMembersEl>>,
    dynamic: TeamMembersDynamic,
}

struct TeamMembers_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<TeamMembersData>,
}

#[derive(Clone)]
pub struct TeamMembers(Rc<TeamMembers_>);

impl TeamMembers {
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

    #[doc= "Set the field `members`.\n"]
    pub fn set_members(self, v: impl Into<BlockAssignable<TeamMembersMembersEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().members = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.members = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `team_id` after provisioning.\nThe GitHub team id or slug"]
    pub fn team_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.team_id", self.extract_ref()))
    }
}

impl Referable for TeamMembers {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for TeamMembers { }

impl ToListMappable for TeamMembers {
    type O = ListRef<TeamMembersRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for TeamMembers_ {
    fn extract_resource_type(&self) -> String {
        "github_team_members".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTeamMembers {
    pub tf_id: String,
    #[doc= "The GitHub team id or slug"]
    pub team_id: PrimField<String>,
}

impl BuildTeamMembers {
    pub fn build(self, stack: &mut Stack) -> TeamMembers {
        let out = TeamMembers(Rc::new(TeamMembers_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(TeamMembersData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                team_id: self.team_id,
                members: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TeamMembersRef {
    shared: StackShared,
    base: String,
}

impl Ref for TeamMembersRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TeamMembersRef {
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

    #[doc= "Get a reference to the value of field `team_id` after provisioning.\nThe GitHub team id or slug"]
    pub fn team_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.team_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct TeamMembersMembersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<PrimField<String>>,
    username: PrimField<String>,
}

impl TeamMembersMembersEl {
    #[doc= "Set the field `role`.\nThe role of the user within the team. Must be one of 'member' or 'maintainer'."]
    pub fn set_role(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role = Some(v.into());
        self
    }
}

impl ToListMappable for TeamMembersMembersEl {
    type O = BlockAssignable<TeamMembersMembersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTeamMembersMembersEl {
    #[doc= "The user to add to the team."]
    pub username: PrimField<String>,
}

impl BuildTeamMembersMembersEl {
    pub fn build(self) -> TeamMembersMembersEl {
        TeamMembersMembersEl {
            role: core::default::Default::default(),
            username: self.username,
        }
    }
}

pub struct TeamMembersMembersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TeamMembersMembersElRef {
    fn new(shared: StackShared, base: String) -> TeamMembersMembersElRef {
        TeamMembersMembersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TeamMembersMembersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `role` after provisioning.\nThe role of the user within the team. Must be one of 'member' or 'maintainer'."]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }

    #[doc= "Get a reference to the value of field `username` after provisioning.\nThe user to add to the team."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize, Default)]
struct TeamMembersDynamic {
    members: Option<DynamicBlock<TeamMembersMembersEl>>,
}
