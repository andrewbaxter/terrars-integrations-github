use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataUserExternalIdentityData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    username: PrimField<String>,
}

struct DataUserExternalIdentity_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataUserExternalIdentityData>,
}

#[derive(Clone)]
pub struct DataUserExternalIdentity(Rc<DataUserExternalIdentity_>);

impl DataUserExternalIdentity {
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

    #[doc= "Get a reference to the value of field `login` after provisioning.\n"]
    pub fn login(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.login", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `saml_identity` after provisioning.\n"]
    pub fn saml_identity(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.saml_identity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `scim_identity` after provisioning.\n"]
    pub fn scim_identity(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.scim_identity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.extract_ref()))
    }
}

impl Referable for DataUserExternalIdentity {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataUserExternalIdentity { }

impl ToListMappable for DataUserExternalIdentity {
    type O = ListRef<DataUserExternalIdentityRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataUserExternalIdentity_ {
    fn extract_datasource_type(&self) -> String {
        "github_user_external_identity".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataUserExternalIdentity {
    pub tf_id: String,
    #[doc= ""]
    pub username: PrimField<String>,
}

impl BuildDataUserExternalIdentity {
    pub fn build(self, stack: &mut Stack) -> DataUserExternalIdentity {
        let out = DataUserExternalIdentity(Rc::new(DataUserExternalIdentity_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataUserExternalIdentityData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                username: self.username,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataUserExternalIdentityRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataUserExternalIdentityRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataUserExternalIdentityRef {
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

    #[doc= "Get a reference to the value of field `login` after provisioning.\n"]
    pub fn login(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.login", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `saml_identity` after provisioning.\n"]
    pub fn saml_identity(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.saml_identity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `scim_identity` after provisioning.\n"]
    pub fn scim_identity(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.scim_identity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.extract_ref()))
    }
}
