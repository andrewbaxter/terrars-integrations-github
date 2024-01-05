use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataOrganizationTeamSyncGroupsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

struct DataOrganizationTeamSyncGroups_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOrganizationTeamSyncGroupsData>,
}

#[derive(Clone)]
pub struct DataOrganizationTeamSyncGroups(Rc<DataOrganizationTeamSyncGroups_>);

impl DataOrganizationTeamSyncGroups {
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

    #[doc= "Get a reference to the value of field `groups` after provisioning.\n"]
    pub fn groups(&self) -> ListRef<DataOrganizationTeamSyncGroupsGroupsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.groups", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

impl Referable for DataOrganizationTeamSyncGroups {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataOrganizationTeamSyncGroups { }

impl ToListMappable for DataOrganizationTeamSyncGroups {
    type O = ListRef<DataOrganizationTeamSyncGroupsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataOrganizationTeamSyncGroups_ {
    fn extract_datasource_type(&self) -> String {
        "github_organization_team_sync_groups".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOrganizationTeamSyncGroups {
    pub tf_id: String,
}

impl BuildDataOrganizationTeamSyncGroups {
    pub fn build(self, stack: &mut Stack) -> DataOrganizationTeamSyncGroups {
        let out = DataOrganizationTeamSyncGroups(Rc::new(DataOrganizationTeamSyncGroups_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataOrganizationTeamSyncGroupsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOrganizationTeamSyncGroupsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOrganizationTeamSyncGroupsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOrganizationTeamSyncGroupsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `groups` after provisioning.\n"]
    pub fn groups(&self) -> ListRef<DataOrganizationTeamSyncGroupsGroupsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.groups", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOrganizationTeamSyncGroupsGroupsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    group_description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_name: Option<PrimField<String>>,
}

impl DataOrganizationTeamSyncGroupsGroupsEl {
    #[doc= "Set the field `group_description`.\n"]
    pub fn set_group_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group_description = Some(v.into());
        self
    }

    #[doc= "Set the field `group_id`.\n"]
    pub fn set_group_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group_id = Some(v.into());
        self
    }

    #[doc= "Set the field `group_name`.\n"]
    pub fn set_group_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataOrganizationTeamSyncGroupsGroupsEl {
    type O = BlockAssignable<DataOrganizationTeamSyncGroupsGroupsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOrganizationTeamSyncGroupsGroupsEl {}

impl BuildDataOrganizationTeamSyncGroupsGroupsEl {
    pub fn build(self) -> DataOrganizationTeamSyncGroupsGroupsEl {
        DataOrganizationTeamSyncGroupsGroupsEl {
            group_description: core::default::Default::default(),
            group_id: core::default::Default::default(),
            group_name: core::default::Default::default(),
        }
    }
}

pub struct DataOrganizationTeamSyncGroupsGroupsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOrganizationTeamSyncGroupsGroupsElRef {
    fn new(shared: StackShared, base: String) -> DataOrganizationTeamSyncGroupsGroupsElRef {
        DataOrganizationTeamSyncGroupsGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOrganizationTeamSyncGroupsGroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `group_description` after provisioning.\n"]
    pub fn group_description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_description", self.base))
    }

    #[doc= "Get a reference to the value of field `group_id` after provisioning.\n"]
    pub fn group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.base))
    }

    #[doc= "Get a reference to the value of field `group_name` after provisioning.\n"]
    pub fn group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_name", self.base))
    }
}
