use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataActionsOrganizationRegistrationTokenData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

struct DataActionsOrganizationRegistrationToken_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataActionsOrganizationRegistrationTokenData>,
}

#[derive(Clone)]
pub struct DataActionsOrganizationRegistrationToken(Rc<DataActionsOrganizationRegistrationToken_>);

impl DataActionsOrganizationRegistrationToken {
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

    #[doc= "Get a reference to the value of field `expires_at` after provisioning.\n"]
    pub fn expires_at(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.expires_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `token` after provisioning.\n"]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.extract_ref()))
    }
}

impl Referable for DataActionsOrganizationRegistrationToken {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataActionsOrganizationRegistrationToken { }

impl ToListMappable for DataActionsOrganizationRegistrationToken {
    type O = ListRef<DataActionsOrganizationRegistrationTokenRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataActionsOrganizationRegistrationToken_ {
    fn extract_datasource_type(&self) -> String {
        "github_actions_organization_registration_token".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataActionsOrganizationRegistrationToken {
    pub tf_id: String,
}

impl BuildDataActionsOrganizationRegistrationToken {
    pub fn build(self, stack: &mut Stack) -> DataActionsOrganizationRegistrationToken {
        let out = DataActionsOrganizationRegistrationToken(Rc::new(DataActionsOrganizationRegistrationToken_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataActionsOrganizationRegistrationTokenData {
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

pub struct DataActionsOrganizationRegistrationTokenRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataActionsOrganizationRegistrationTokenRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataActionsOrganizationRegistrationTokenRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `expires_at` after provisioning.\n"]
    pub fn expires_at(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.expires_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `token` after provisioning.\n"]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.extract_ref()))
    }
}
