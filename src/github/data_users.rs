use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataUsersData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    usernames: ListField<PrimField<String>>,
}

struct DataUsers_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataUsersData>,
}

#[derive(Clone)]
pub struct DataUsers(Rc<DataUsers_>);

impl DataUsers {
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

    #[doc= "Get a reference to the value of field `emails` after provisioning.\n"]
    pub fn emails(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.emails", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `logins` after provisioning.\n"]
    pub fn logins(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.logins", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_ids` after provisioning.\n"]
    pub fn node_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.node_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `unknown_logins` after provisioning.\n"]
    pub fn unknown_logins(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.unknown_logins", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `usernames` after provisioning.\n"]
    pub fn usernames(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.usernames", self.extract_ref()))
    }
}

impl Referable for DataUsers {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataUsers { }

impl ToListMappable for DataUsers {
    type O = ListRef<DataUsersRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataUsers_ {
    fn extract_datasource_type(&self) -> String {
        "github_users".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataUsers {
    pub tf_id: String,
    #[doc= ""]
    pub usernames: ListField<PrimField<String>>,
}

impl BuildDataUsers {
    pub fn build(self, stack: &mut Stack) -> DataUsers {
        let out = DataUsers(Rc::new(DataUsers_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataUsersData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                usernames: self.usernames,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataUsersRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataUsersRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataUsersRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `emails` after provisioning.\n"]
    pub fn emails(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.emails", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `logins` after provisioning.\n"]
    pub fn logins(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.logins", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_ids` after provisioning.\n"]
    pub fn node_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.node_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `unknown_logins` after provisioning.\n"]
    pub fn unknown_logins(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.unknown_logins", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `usernames` after provisioning.\n"]
    pub fn usernames(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.usernames", self.extract_ref()))
    }
}
