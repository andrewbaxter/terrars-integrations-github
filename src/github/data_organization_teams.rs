use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataOrganizationTeamsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    results_per_page: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_teams_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary_only: Option<PrimField<bool>>,
}

struct DataOrganizationTeams_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOrganizationTeamsData>,
}

#[derive(Clone)]
pub struct DataOrganizationTeams(Rc<DataOrganizationTeams_>);

impl DataOrganizationTeams {
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

    #[doc= "Set the field `results_per_page`.\n"]
    pub fn set_results_per_page(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().results_per_page = Some(v.into());
        self
    }

    #[doc= "Set the field `root_teams_only`.\n"]
    pub fn set_root_teams_only(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().root_teams_only = Some(v.into());
        self
    }

    #[doc= "Set the field `summary_only`.\n"]
    pub fn set_summary_only(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().summary_only = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `results_per_page` after provisioning.\n"]
    pub fn results_per_page(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.results_per_page", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `root_teams_only` after provisioning.\n"]
    pub fn root_teams_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.root_teams_only", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `summary_only` after provisioning.\n"]
    pub fn summary_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.summary_only", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `teams` after provisioning.\n"]
    pub fn teams(&self) -> ListRef<DataOrganizationTeamsTeamsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.teams", self.extract_ref()))
    }
}

impl Referable for DataOrganizationTeams {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataOrganizationTeams { }

impl ToListMappable for DataOrganizationTeams {
    type O = ListRef<DataOrganizationTeamsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataOrganizationTeams_ {
    fn extract_datasource_type(&self) -> String {
        "github_organization_teams".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOrganizationTeams {
    pub tf_id: String,
}

impl BuildDataOrganizationTeams {
    pub fn build(self, stack: &mut Stack) -> DataOrganizationTeams {
        let out = DataOrganizationTeams(Rc::new(DataOrganizationTeams_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataOrganizationTeamsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                results_per_page: core::default::Default::default(),
                root_teams_only: core::default::Default::default(),
                summary_only: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOrganizationTeamsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOrganizationTeamsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOrganizationTeamsRef {
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

    #[doc= "Get a reference to the value of field `results_per_page` after provisioning.\n"]
    pub fn results_per_page(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.results_per_page", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `root_teams_only` after provisioning.\n"]
    pub fn root_teams_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.root_teams_only", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `summary_only` after provisioning.\n"]
    pub fn summary_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.summary_only", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `teams` after provisioning.\n"]
    pub fn teams(&self) -> ListRef<DataOrganizationTeamsTeamsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.teams", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOrganizationTeamsTeamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    members: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    privacy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repositories: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slug: Option<PrimField<String>>,
}

impl DataOrganizationTeamsTeamsEl {
    #[doc= "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `members`.\n"]
    pub fn set_members(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.members = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `node_id`.\n"]
    pub fn set_node_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_id = Some(v.into());
        self
    }

    #[doc= "Set the field `parent`.\n"]
    pub fn set_parent(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.parent = Some(v.into());
        self
    }

    #[doc= "Set the field `privacy`.\n"]
    pub fn set_privacy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.privacy = Some(v.into());
        self
    }

    #[doc= "Set the field `repositories`.\n"]
    pub fn set_repositories(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.repositories = Some(v.into());
        self
    }

    #[doc= "Set the field `slug`.\n"]
    pub fn set_slug(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.slug = Some(v.into());
        self
    }
}

impl ToListMappable for DataOrganizationTeamsTeamsEl {
    type O = BlockAssignable<DataOrganizationTeamsTeamsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOrganizationTeamsTeamsEl {}

impl BuildDataOrganizationTeamsTeamsEl {
    pub fn build(self) -> DataOrganizationTeamsTeamsEl {
        DataOrganizationTeamsTeamsEl {
            description: core::default::Default::default(),
            id: core::default::Default::default(),
            members: core::default::Default::default(),
            name: core::default::Default::default(),
            node_id: core::default::Default::default(),
            parent: core::default::Default::default(),
            privacy: core::default::Default::default(),
            repositories: core::default::Default::default(),
            slug: core::default::Default::default(),
        }
    }
}

pub struct DataOrganizationTeamsTeamsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOrganizationTeamsTeamsElRef {
    fn new(shared: StackShared, base: String) -> DataOrganizationTeamsTeamsElRef {
        DataOrganizationTeamsTeamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOrganizationTeamsTeamsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `members` after provisioning.\n"]
    pub fn members(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.members", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `node_id` after provisioning.\n"]
    pub fn node_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_id", self.base))
    }

    #[doc= "Get a reference to the value of field `parent` after provisioning.\n"]
    pub fn parent(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.parent", self.base))
    }

    #[doc= "Get a reference to the value of field `privacy` after provisioning.\n"]
    pub fn privacy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.privacy", self.base))
    }

    #[doc= "Get a reference to the value of field `repositories` after provisioning.\n"]
    pub fn repositories(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.repositories", self.base))
    }

    #[doc= "Get a reference to the value of field `slug` after provisioning.\n"]
    pub fn slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slug", self.base))
    }
}
