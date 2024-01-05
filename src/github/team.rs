use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct TeamData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_default_maintainer: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ldap_dn: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_team_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_team_read_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_team_read_slug: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    privacy: Option<PrimField<String>>,
}

struct Team_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<TeamData>,
}

#[derive(Clone)]
pub struct Team(Rc<Team_>);

impl Team {
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

    #[doc= "Set the field `create_default_maintainer`.\nAdds a default maintainer to the team. Adds the creating user to the team when 'true'."]
    pub fn set_create_default_maintainer(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().create_default_maintainer = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nA description of the team."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `ldap_dn`.\nThe LDAP Distinguished Name of the group where membership will be synchronized. Only available in GitHub Enterprise Server."]
    pub fn set_ldap_dn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ldap_dn = Some(v.into());
        self
    }

    #[doc= "Set the field `parent_team_id`.\nThe ID or slug of the parent team, if this is a nested team."]
    pub fn set_parent_team_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parent_team_id = Some(v.into());
        self
    }

    #[doc= "Set the field `parent_team_read_id`.\nThe id of the parent team read in Github."]
    pub fn set_parent_team_read_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parent_team_read_id = Some(v.into());
        self
    }

    #[doc= "Set the field `parent_team_read_slug`.\nThe id of the parent team read in Github."]
    pub fn set_parent_team_read_slug(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parent_team_read_slug = Some(v.into());
        self
    }

    #[doc= "Set the field `privacy`.\nThe level of privacy for the team. Must be one of 'secret' or 'closed'."]
    pub fn set_privacy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().privacy = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `create_default_maintainer` after provisioning.\nAdds a default maintainer to the team. Adds the creating user to the team when 'true'."]
    pub fn create_default_maintainer(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_default_maintainer", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA description of the team."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ldap_dn` after provisioning.\nThe LDAP Distinguished Name of the group where membership will be synchronized. Only available in GitHub Enterprise Server."]
    pub fn ldap_dn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ldap_dn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `members_count` after provisioning.\n"]
    pub fn members_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.members_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the team."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_id` after provisioning.\nThe Node ID of the created team."]
    pub fn node_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `parent_team_id` after provisioning.\nThe ID or slug of the parent team, if this is a nested team."]
    pub fn parent_team_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parent_team_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `parent_team_read_id` after provisioning.\nThe id of the parent team read in Github."]
    pub fn parent_team_read_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parent_team_read_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `parent_team_read_slug` after provisioning.\nThe id of the parent team read in Github."]
    pub fn parent_team_read_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parent_team_read_slug", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `privacy` after provisioning.\nThe level of privacy for the team. Must be one of 'secret' or 'closed'."]
    pub fn privacy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.privacy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `slug` after provisioning.\nThe slug of the created team."]
    pub fn slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slug", self.extract_ref()))
    }
}

impl Referable for Team {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Team { }

impl ToListMappable for Team {
    type O = ListRef<TeamRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Team_ {
    fn extract_resource_type(&self) -> String {
        "github_team".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTeam {
    pub tf_id: String,
    #[doc= "The name of the team."]
    pub name: PrimField<String>,
}

impl BuildTeam {
    pub fn build(self, stack: &mut Stack) -> Team {
        let out = Team(Rc::new(Team_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(TeamData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                create_default_maintainer: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                ldap_dn: core::default::Default::default(),
                name: self.name,
                parent_team_id: core::default::Default::default(),
                parent_team_read_id: core::default::Default::default(),
                parent_team_read_slug: core::default::Default::default(),
                privacy: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TeamRef {
    shared: StackShared,
    base: String,
}

impl Ref for TeamRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TeamRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create_default_maintainer` after provisioning.\nAdds a default maintainer to the team. Adds the creating user to the team when 'true'."]
    pub fn create_default_maintainer(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_default_maintainer", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA description of the team."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ldap_dn` after provisioning.\nThe LDAP Distinguished Name of the group where membership will be synchronized. Only available in GitHub Enterprise Server."]
    pub fn ldap_dn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ldap_dn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `members_count` after provisioning.\n"]
    pub fn members_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.members_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the team."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_id` after provisioning.\nThe Node ID of the created team."]
    pub fn node_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `parent_team_id` after provisioning.\nThe ID or slug of the parent team, if this is a nested team."]
    pub fn parent_team_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parent_team_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `parent_team_read_id` after provisioning.\nThe id of the parent team read in Github."]
    pub fn parent_team_read_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parent_team_read_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `parent_team_read_slug` after provisioning.\nThe id of the parent team read in Github."]
    pub fn parent_team_read_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parent_team_read_slug", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `privacy` after provisioning.\nThe level of privacy for the team. Must be one of 'secret' or 'closed'."]
    pub fn privacy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.privacy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `slug` after provisioning.\nThe slug of the created team."]
    pub fn slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slug", self.extract_ref()))
    }
}
