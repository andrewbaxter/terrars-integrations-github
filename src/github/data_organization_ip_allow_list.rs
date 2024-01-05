use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataOrganizationIpAllowListData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

struct DataOrganizationIpAllowList_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOrganizationIpAllowListData>,
}

#[derive(Clone)]
pub struct DataOrganizationIpAllowList(Rc<DataOrganizationIpAllowList_>);

impl DataOrganizationIpAllowList {
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

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip_allow_list` after provisioning.\n"]
    pub fn ip_allow_list(&self) -> ListRef<DataOrganizationIpAllowListIpAllowListElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ip_allow_list", self.extract_ref()))
    }
}

impl Referable for DataOrganizationIpAllowList {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataOrganizationIpAllowList { }

impl ToListMappable for DataOrganizationIpAllowList {
    type O = ListRef<DataOrganizationIpAllowListRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataOrganizationIpAllowList_ {
    fn extract_datasource_type(&self) -> String {
        "github_organization_ip_allow_list".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOrganizationIpAllowList {
    pub tf_id: String,
}

impl BuildDataOrganizationIpAllowList {
    pub fn build(self, stack: &mut Stack) -> DataOrganizationIpAllowList {
        let out = DataOrganizationIpAllowList(Rc::new(DataOrganizationIpAllowList_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataOrganizationIpAllowListData {
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

pub struct DataOrganizationIpAllowListRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOrganizationIpAllowListRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOrganizationIpAllowListRef {
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

    #[doc= "Get a reference to the value of field `ip_allow_list` after provisioning.\n"]
    pub fn ip_allow_list(&self) -> ListRef<DataOrganizationIpAllowListIpAllowListElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ip_allow_list", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOrganizationIpAllowListIpAllowListEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_list_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_active: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<PrimField<String>>,
}

impl DataOrganizationIpAllowListIpAllowListEl {
    #[doc= "Set the field `allow_list_value`.\n"]
    pub fn set_allow_list_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.allow_list_value = Some(v.into());
        self
    }

    #[doc= "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created_at = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `is_active`.\n"]
    pub fn set_is_active(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_active = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `updated_at`.\n"]
    pub fn set_updated_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.updated_at = Some(v.into());
        self
    }
}

impl ToListMappable for DataOrganizationIpAllowListIpAllowListEl {
    type O = BlockAssignable<DataOrganizationIpAllowListIpAllowListEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOrganizationIpAllowListIpAllowListEl {}

impl BuildDataOrganizationIpAllowListIpAllowListEl {
    pub fn build(self) -> DataOrganizationIpAllowListIpAllowListEl {
        DataOrganizationIpAllowListIpAllowListEl {
            allow_list_value: core::default::Default::default(),
            created_at: core::default::Default::default(),
            id: core::default::Default::default(),
            is_active: core::default::Default::default(),
            name: core::default::Default::default(),
            updated_at: core::default::Default::default(),
        }
    }
}

pub struct DataOrganizationIpAllowListIpAllowListElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOrganizationIpAllowListIpAllowListElRef {
    fn new(shared: StackShared, base: String) -> DataOrganizationIpAllowListIpAllowListElRef {
        DataOrganizationIpAllowListIpAllowListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOrganizationIpAllowListIpAllowListElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allow_list_value` after provisioning.\n"]
    pub fn allow_list_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_list_value", self.base))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `is_active` after provisioning.\n"]
    pub fn is_active(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_active", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }
}
