use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct TeamSyncGroupMappingData {
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
    team_slug: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<Vec<TeamSyncGroupMappingGroupEl>>,
    dynamic: TeamSyncGroupMappingDynamic,
}

struct TeamSyncGroupMapping_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<TeamSyncGroupMappingData>,
}

#[derive(Clone)]
pub struct TeamSyncGroupMapping(Rc<TeamSyncGroupMapping_>);

impl TeamSyncGroupMapping {
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

    #[doc= "Set the field `group`.\n"]
    pub fn set_group(self, v: impl Into<BlockAssignable<TeamSyncGroupMappingGroupEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().group = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.group = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `team_slug` after provisioning.\nSlug of the team."]
    pub fn team_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.team_slug", self.extract_ref()))
    }
}

impl Referable for TeamSyncGroupMapping {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for TeamSyncGroupMapping { }

impl ToListMappable for TeamSyncGroupMapping {
    type O = ListRef<TeamSyncGroupMappingRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for TeamSyncGroupMapping_ {
    fn extract_resource_type(&self) -> String {
        "github_team_sync_group_mapping".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTeamSyncGroupMapping {
    pub tf_id: String,
    #[doc= "Slug of the team."]
    pub team_slug: PrimField<String>,
}

impl BuildTeamSyncGroupMapping {
    pub fn build(self, stack: &mut Stack) -> TeamSyncGroupMapping {
        let out = TeamSyncGroupMapping(Rc::new(TeamSyncGroupMapping_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(TeamSyncGroupMappingData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                team_slug: self.team_slug,
                group: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TeamSyncGroupMappingRef {
    shared: StackShared,
    base: String,
}

impl Ref for TeamSyncGroupMappingRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TeamSyncGroupMappingRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `team_slug` after provisioning.\nSlug of the team."]
    pub fn team_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.team_slug", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct TeamSyncGroupMappingGroupEl {
    group_description: PrimField<String>,
    group_id: PrimField<String>,
    group_name: PrimField<String>,
}

impl TeamSyncGroupMappingGroupEl { }

impl ToListMappable for TeamSyncGroupMappingGroupEl {
    type O = BlockAssignable<TeamSyncGroupMappingGroupEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTeamSyncGroupMappingGroupEl {
    #[doc= "The description of the IdP group."]
    pub group_description: PrimField<String>,
    #[doc= "The ID of the IdP group."]
    pub group_id: PrimField<String>,
    #[doc= "The name of the IdP group."]
    pub group_name: PrimField<String>,
}

impl BuildTeamSyncGroupMappingGroupEl {
    pub fn build(self) -> TeamSyncGroupMappingGroupEl {
        TeamSyncGroupMappingGroupEl {
            group_description: self.group_description,
            group_id: self.group_id,
            group_name: self.group_name,
        }
    }
}

pub struct TeamSyncGroupMappingGroupElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TeamSyncGroupMappingGroupElRef {
    fn new(shared: StackShared, base: String) -> TeamSyncGroupMappingGroupElRef {
        TeamSyncGroupMappingGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TeamSyncGroupMappingGroupElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `group_description` after provisioning.\nThe description of the IdP group."]
    pub fn group_description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_description", self.base))
    }

    #[doc= "Get a reference to the value of field `group_id` after provisioning.\nThe ID of the IdP group."]
    pub fn group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.base))
    }

    #[doc= "Get a reference to the value of field `group_name` after provisioning.\nThe name of the IdP group."]
    pub fn group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct TeamSyncGroupMappingDynamic {
    group: Option<DynamicBlock<TeamSyncGroupMappingGroupEl>>,
}
