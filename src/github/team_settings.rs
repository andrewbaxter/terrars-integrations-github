use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct TeamSettingsData {
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
    review_request_delegation: Option<Vec<TeamSettingsReviewRequestDelegationEl>>,
    dynamic: TeamSettingsDynamic,
}

struct TeamSettings_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<TeamSettingsData>,
}

#[derive(Clone)]
pub struct TeamSettings(Rc<TeamSettings_>);

impl TeamSettings {
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

    #[doc= "Set the field `review_request_delegation`.\n"]
    pub fn set_review_request_delegation(
        self,
        v: impl Into<BlockAssignable<TeamSettingsReviewRequestDelegationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().review_request_delegation = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.review_request_delegation = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `team_id` after provisioning.\nThe GitHub team id or the GitHub team slug."]
    pub fn team_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.team_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `team_slug` after provisioning.\nThe slug of the Team within the Organization."]
    pub fn team_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.team_slug", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `team_uid` after provisioning.\nThe unique ID of the Team on GitHub. Corresponds to the ID of the 'github_team_settings' resource."]
    pub fn team_uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.team_uid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `review_request_delegation` after provisioning.\n"]
    pub fn review_request_delegation(&self) -> ListRef<TeamSettingsReviewRequestDelegationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.review_request_delegation", self.extract_ref()))
    }
}

impl Referable for TeamSettings {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for TeamSettings { }

impl ToListMappable for TeamSettings {
    type O = ListRef<TeamSettingsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for TeamSettings_ {
    fn extract_resource_type(&self) -> String {
        "github_team_settings".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTeamSettings {
    pub tf_id: String,
    #[doc= "The GitHub team id or the GitHub team slug."]
    pub team_id: PrimField<String>,
}

impl BuildTeamSettings {
    pub fn build(self, stack: &mut Stack) -> TeamSettings {
        let out = TeamSettings(Rc::new(TeamSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(TeamSettingsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                team_id: self.team_id,
                review_request_delegation: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TeamSettingsRef {
    shared: StackShared,
    base: String,
}

impl Ref for TeamSettingsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TeamSettingsRef {
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

    #[doc= "Get a reference to the value of field `team_id` after provisioning.\nThe GitHub team id or the GitHub team slug."]
    pub fn team_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.team_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `team_slug` after provisioning.\nThe slug of the Team within the Organization."]
    pub fn team_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.team_slug", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `team_uid` after provisioning.\nThe unique ID of the Team on GitHub. Corresponds to the ID of the 'github_team_settings' resource."]
    pub fn team_uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.team_uid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `review_request_delegation` after provisioning.\n"]
    pub fn review_request_delegation(&self) -> ListRef<TeamSettingsReviewRequestDelegationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.review_request_delegation", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct TeamSettingsReviewRequestDelegationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notify: Option<PrimField<bool>>,
}

impl TeamSettingsReviewRequestDelegationEl {
    #[doc= "Set the field `algorithm`.\nThe algorithm to use when assigning pull requests to team members. Supported values are 'ROUND_ROBIN' and 'LOAD_BALANCE'."]
    pub fn set_algorithm(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.algorithm = Some(v.into());
        self
    }

    #[doc= "Set the field `member_count`.\nThe number of team members to assign to a pull request."]
    pub fn set_member_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.member_count = Some(v.into());
        self
    }

    #[doc= "Set the field `notify`.\nwhether to notify the entire team when at least one member is also assigned to the pull request."]
    pub fn set_notify(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.notify = Some(v.into());
        self
    }
}

impl ToListMappable for TeamSettingsReviewRequestDelegationEl {
    type O = BlockAssignable<TeamSettingsReviewRequestDelegationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTeamSettingsReviewRequestDelegationEl {}

impl BuildTeamSettingsReviewRequestDelegationEl {
    pub fn build(self) -> TeamSettingsReviewRequestDelegationEl {
        TeamSettingsReviewRequestDelegationEl {
            algorithm: core::default::Default::default(),
            member_count: core::default::Default::default(),
            notify: core::default::Default::default(),
        }
    }
}

pub struct TeamSettingsReviewRequestDelegationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TeamSettingsReviewRequestDelegationElRef {
    fn new(shared: StackShared, base: String) -> TeamSettingsReviewRequestDelegationElRef {
        TeamSettingsReviewRequestDelegationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TeamSettingsReviewRequestDelegationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `algorithm` after provisioning.\nThe algorithm to use when assigning pull requests to team members. Supported values are 'ROUND_ROBIN' and 'LOAD_BALANCE'."]
    pub fn algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.base))
    }

    #[doc= "Get a reference to the value of field `member_count` after provisioning.\nThe number of team members to assign to a pull request."]
    pub fn member_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.member_count", self.base))
    }

    #[doc= "Get a reference to the value of field `notify` after provisioning.\nwhether to notify the entire team when at least one member is also assigned to the pull request."]
    pub fn notify(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.notify", self.base))
    }
}

#[derive(Serialize, Default)]
struct TeamSettingsDynamic {
    review_request_delegation: Option<DynamicBlock<TeamSettingsReviewRequestDelegationEl>>,
}
