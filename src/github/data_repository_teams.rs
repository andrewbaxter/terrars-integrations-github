use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataRepositoryTeamsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    full_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

struct DataRepositoryTeams_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRepositoryTeamsData>,
}

#[derive(Clone)]
pub struct DataRepositoryTeams(Rc<DataRepositoryTeams_>);

impl DataRepositoryTeams {
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

    #[doc= "Set the field `full_name`.\n"]
    pub fn set_full_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().full_name = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `full_name` after provisioning.\n"]
    pub fn full_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.full_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `teams` after provisioning.\n"]
    pub fn teams(&self) -> ListRef<DataRepositoryTeamsTeamsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.teams", self.extract_ref()))
    }
}

impl Referable for DataRepositoryTeams {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataRepositoryTeams { }

impl ToListMappable for DataRepositoryTeams {
    type O = ListRef<DataRepositoryTeamsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataRepositoryTeams_ {
    fn extract_datasource_type(&self) -> String {
        "github_repository_teams".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRepositoryTeams {
    pub tf_id: String,
}

impl BuildDataRepositoryTeams {
    pub fn build(self, stack: &mut Stack) -> DataRepositoryTeams {
        let out = DataRepositoryTeams(Rc::new(DataRepositoryTeams_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRepositoryTeamsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                full_name: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRepositoryTeamsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRepositoryTeamsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRepositoryTeamsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `full_name` after provisioning.\n"]
    pub fn full_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.full_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `teams` after provisioning.\n"]
    pub fn teams(&self) -> ListRef<DataRepositoryTeamsTeamsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.teams", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataRepositoryTeamsTeamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permission: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slug: Option<PrimField<String>>,
}

impl DataRepositoryTeamsTeamsEl {
    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `permission`.\n"]
    pub fn set_permission(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.permission = Some(v.into());
        self
    }

    #[doc= "Set the field `slug`.\n"]
    pub fn set_slug(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.slug = Some(v.into());
        self
    }
}

impl ToListMappable for DataRepositoryTeamsTeamsEl {
    type O = BlockAssignable<DataRepositoryTeamsTeamsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRepositoryTeamsTeamsEl {}

impl BuildDataRepositoryTeamsTeamsEl {
    pub fn build(self) -> DataRepositoryTeamsTeamsEl {
        DataRepositoryTeamsTeamsEl {
            name: core::default::Default::default(),
            permission: core::default::Default::default(),
            slug: core::default::Default::default(),
        }
    }
}

pub struct DataRepositoryTeamsTeamsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRepositoryTeamsTeamsElRef {
    fn new(shared: StackShared, base: String) -> DataRepositoryTeamsTeamsElRef {
        DataRepositoryTeamsTeamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRepositoryTeamsTeamsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `permission` after provisioning.\n"]
    pub fn permission(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.permission", self.base))
    }

    #[doc= "Get a reference to the value of field `slug` after provisioning.\n"]
    pub fn slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slug", self.base))
    }
}
