use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataOrganizationExternalIdentitiesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

struct DataOrganizationExternalIdentities_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOrganizationExternalIdentitiesData>,
}

#[derive(Clone)]
pub struct DataOrganizationExternalIdentities(Rc<DataOrganizationExternalIdentities_>);

impl DataOrganizationExternalIdentities {
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

    #[doc= "Get a reference to the value of field `identities` after provisioning.\n"]
    pub fn identities(&self) -> ListRef<DataOrganizationExternalIdentitiesIdentitiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.identities", self.extract_ref()))
    }
}

impl Referable for DataOrganizationExternalIdentities {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataOrganizationExternalIdentities { }

impl ToListMappable for DataOrganizationExternalIdentities {
    type O = ListRef<DataOrganizationExternalIdentitiesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataOrganizationExternalIdentities_ {
    fn extract_datasource_type(&self) -> String {
        "github_organization_external_identities".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOrganizationExternalIdentities {
    pub tf_id: String,
}

impl BuildDataOrganizationExternalIdentities {
    pub fn build(self, stack: &mut Stack) -> DataOrganizationExternalIdentities {
        let out = DataOrganizationExternalIdentities(Rc::new(DataOrganizationExternalIdentities_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataOrganizationExternalIdentitiesData {
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

pub struct DataOrganizationExternalIdentitiesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOrganizationExternalIdentitiesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOrganizationExternalIdentitiesRef {
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

    #[doc= "Get a reference to the value of field `identities` after provisioning.\n"]
    pub fn identities(&self) -> ListRef<DataOrganizationExternalIdentitiesIdentitiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.identities", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOrganizationExternalIdentitiesIdentitiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    login: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    saml_identity: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scim_identity: Option<RecField<PrimField<String>>>,
}

impl DataOrganizationExternalIdentitiesIdentitiesEl {
    #[doc= "Set the field `login`.\n"]
    pub fn set_login(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.login = Some(v.into());
        self
    }

    #[doc= "Set the field `saml_identity`.\n"]
    pub fn set_saml_identity(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.saml_identity = Some(v.into());
        self
    }

    #[doc= "Set the field `scim_identity`.\n"]
    pub fn set_scim_identity(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.scim_identity = Some(v.into());
        self
    }
}

impl ToListMappable for DataOrganizationExternalIdentitiesIdentitiesEl {
    type O = BlockAssignable<DataOrganizationExternalIdentitiesIdentitiesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOrganizationExternalIdentitiesIdentitiesEl {}

impl BuildDataOrganizationExternalIdentitiesIdentitiesEl {
    pub fn build(self) -> DataOrganizationExternalIdentitiesIdentitiesEl {
        DataOrganizationExternalIdentitiesIdentitiesEl {
            login: core::default::Default::default(),
            saml_identity: core::default::Default::default(),
            scim_identity: core::default::Default::default(),
        }
    }
}

pub struct DataOrganizationExternalIdentitiesIdentitiesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOrganizationExternalIdentitiesIdentitiesElRef {
    fn new(shared: StackShared, base: String) -> DataOrganizationExternalIdentitiesIdentitiesElRef {
        DataOrganizationExternalIdentitiesIdentitiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOrganizationExternalIdentitiesIdentitiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `login` after provisioning.\n"]
    pub fn login(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.login", self.base))
    }

    #[doc= "Get a reference to the value of field `saml_identity` after provisioning.\n"]
    pub fn saml_identity(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.saml_identity", self.base))
    }

    #[doc= "Get a reference to the value of field `scim_identity` after provisioning.\n"]
    pub fn scim_identity(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.scim_identity", self.base))
    }
}
