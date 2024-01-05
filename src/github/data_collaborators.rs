use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataCollaboratorsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    affiliation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    owner: PrimField<String>,
    repository: PrimField<String>,
}

struct DataCollaborators_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCollaboratorsData>,
}

#[derive(Clone)]
pub struct DataCollaborators(Rc<DataCollaborators_>);

impl DataCollaborators {
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

    #[doc= "Set the field `affiliation`.\n"]
    pub fn set_affiliation(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().affiliation = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `affiliation` after provisioning.\n"]
    pub fn affiliation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.affiliation", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `collaborator` after provisioning.\n"]
    pub fn collaborator(&self) -> ListRef<DataCollaboratorsCollaboratorElRef> {
        ListRef::new(self.shared().clone(), format!("{}.collaborator", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }
}

impl Referable for DataCollaborators {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataCollaborators { }

impl ToListMappable for DataCollaborators {
    type O = ListRef<DataCollaboratorsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataCollaborators_ {
    fn extract_datasource_type(&self) -> String {
        "github_collaborators".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCollaborators {
    pub tf_id: String,
    #[doc= ""]
    pub owner: PrimField<String>,
    #[doc= ""]
    pub repository: PrimField<String>,
}

impl BuildDataCollaborators {
    pub fn build(self, stack: &mut Stack) -> DataCollaborators {
        let out = DataCollaborators(Rc::new(DataCollaborators_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCollaboratorsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                affiliation: core::default::Default::default(),
                id: core::default::Default::default(),
                owner: self.owner,
                repository: self.repository,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCollaboratorsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCollaboratorsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCollaboratorsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `affiliation` after provisioning.\n"]
    pub fn affiliation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.affiliation", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `collaborator` after provisioning.\n"]
    pub fn collaborator(&self) -> ListRef<DataCollaboratorsCollaboratorElRef> {
        ListRef::new(self.shared().clone(), format!("{}.collaborator", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCollaboratorsCollaboratorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    events_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    followers_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    following_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gists_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    html_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    login: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organizations_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permission: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    received_events_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repos_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    site_admin: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starred_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscriptions_url: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<PrimField<String>>,
}

impl DataCollaboratorsCollaboratorEl {
    #[doc= "Set the field `events_url`.\n"]
    pub fn set_events_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.events_url = Some(v.into());
        self
    }

    #[doc= "Set the field `followers_url`.\n"]
    pub fn set_followers_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.followers_url = Some(v.into());
        self
    }

    #[doc= "Set the field `following_url`.\n"]
    pub fn set_following_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.following_url = Some(v.into());
        self
    }

    #[doc= "Set the field `gists_url`.\n"]
    pub fn set_gists_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gists_url = Some(v.into());
        self
    }

    #[doc= "Set the field `html_url`.\n"]
    pub fn set_html_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.html_url = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `login`.\n"]
    pub fn set_login(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.login = Some(v.into());
        self
    }

    #[doc= "Set the field `organizations_url`.\n"]
    pub fn set_organizations_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.organizations_url = Some(v.into());
        self
    }

    #[doc= "Set the field `permission`.\n"]
    pub fn set_permission(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.permission = Some(v.into());
        self
    }

    #[doc= "Set the field `received_events_url`.\n"]
    pub fn set_received_events_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.received_events_url = Some(v.into());
        self
    }

    #[doc= "Set the field `repos_url`.\n"]
    pub fn set_repos_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repos_url = Some(v.into());
        self
    }

    #[doc= "Set the field `site_admin`.\n"]
    pub fn set_site_admin(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.site_admin = Some(v.into());
        self
    }

    #[doc= "Set the field `starred_url`.\n"]
    pub fn set_starred_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.starred_url = Some(v.into());
        self
    }

    #[doc= "Set the field `subscriptions_url`.\n"]
    pub fn set_subscriptions_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subscriptions_url = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url = Some(v.into());
        self
    }
}

impl ToListMappable for DataCollaboratorsCollaboratorEl {
    type O = BlockAssignable<DataCollaboratorsCollaboratorEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCollaboratorsCollaboratorEl {}

impl BuildDataCollaboratorsCollaboratorEl {
    pub fn build(self) -> DataCollaboratorsCollaboratorEl {
        DataCollaboratorsCollaboratorEl {
            events_url: core::default::Default::default(),
            followers_url: core::default::Default::default(),
            following_url: core::default::Default::default(),
            gists_url: core::default::Default::default(),
            html_url: core::default::Default::default(),
            id: core::default::Default::default(),
            login: core::default::Default::default(),
            organizations_url: core::default::Default::default(),
            permission: core::default::Default::default(),
            received_events_url: core::default::Default::default(),
            repos_url: core::default::Default::default(),
            site_admin: core::default::Default::default(),
            starred_url: core::default::Default::default(),
            subscriptions_url: core::default::Default::default(),
            type_: core::default::Default::default(),
            url: core::default::Default::default(),
        }
    }
}

pub struct DataCollaboratorsCollaboratorElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCollaboratorsCollaboratorElRef {
    fn new(shared: StackShared, base: String) -> DataCollaboratorsCollaboratorElRef {
        DataCollaboratorsCollaboratorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCollaboratorsCollaboratorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `events_url` after provisioning.\n"]
    pub fn events_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.events_url", self.base))
    }

    #[doc= "Get a reference to the value of field `followers_url` after provisioning.\n"]
    pub fn followers_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.followers_url", self.base))
    }

    #[doc= "Get a reference to the value of field `following_url` after provisioning.\n"]
    pub fn following_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.following_url", self.base))
    }

    #[doc= "Get a reference to the value of field `gists_url` after provisioning.\n"]
    pub fn gists_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gists_url", self.base))
    }

    #[doc= "Get a reference to the value of field `html_url` after provisioning.\n"]
    pub fn html_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.html_url", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `login` after provisioning.\n"]
    pub fn login(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.login", self.base))
    }

    #[doc= "Get a reference to the value of field `organizations_url` after provisioning.\n"]
    pub fn organizations_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.organizations_url", self.base))
    }

    #[doc= "Get a reference to the value of field `permission` after provisioning.\n"]
    pub fn permission(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.permission", self.base))
    }

    #[doc= "Get a reference to the value of field `received_events_url` after provisioning.\n"]
    pub fn received_events_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.received_events_url", self.base))
    }

    #[doc= "Get a reference to the value of field `repos_url` after provisioning.\n"]
    pub fn repos_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repos_url", self.base))
    }

    #[doc= "Get a reference to the value of field `site_admin` after provisioning.\n"]
    pub fn site_admin(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.site_admin", self.base))
    }

    #[doc= "Get a reference to the value of field `starred_url` after provisioning.\n"]
    pub fn starred_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.starred_url", self.base))
    }

    #[doc= "Get a reference to the value of field `subscriptions_url` after provisioning.\n"]
    pub fn subscriptions_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subscriptions_url", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
