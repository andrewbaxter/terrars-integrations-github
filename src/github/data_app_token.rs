use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataAppTokenData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    app_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    installation_id: PrimField<String>,
    pem_file: PrimField<String>,
}

struct DataAppToken_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAppTokenData>,
}

#[derive(Clone)]
pub struct DataAppToken(Rc<DataAppToken_>);

impl DataAppToken {
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

    #[doc= "Get a reference to the value of field `app_id` after provisioning.\nThe GitHub App ID."]
    pub fn app_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `installation_id` after provisioning.\nThe GitHub App installation instance ID."]
    pub fn installation_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.installation_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pem_file` after provisioning.\nThe GitHub App PEM file contents."]
    pub fn pem_file(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pem_file", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `token` after provisioning.\nThe generated token from the credentials."]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.extract_ref()))
    }
}

impl Referable for DataAppToken {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataAppToken { }

impl ToListMappable for DataAppToken {
    type O = ListRef<DataAppTokenRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataAppToken_ {
    fn extract_datasource_type(&self) -> String {
        "github_app_token".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAppToken {
    pub tf_id: String,
    #[doc= "The GitHub App ID."]
    pub app_id: PrimField<String>,
    #[doc= "The GitHub App installation instance ID."]
    pub installation_id: PrimField<String>,
    #[doc= "The GitHub App PEM file contents."]
    pub pem_file: PrimField<String>,
}

impl BuildDataAppToken {
    pub fn build(self, stack: &mut Stack) -> DataAppToken {
        let out = DataAppToken(Rc::new(DataAppToken_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataAppTokenData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                app_id: self.app_id,
                id: core::default::Default::default(),
                installation_id: self.installation_id,
                pem_file: self.pem_file,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAppTokenRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppTokenRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAppTokenRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `app_id` after provisioning.\nThe GitHub App ID."]
    pub fn app_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `installation_id` after provisioning.\nThe GitHub App installation instance ID."]
    pub fn installation_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.installation_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pem_file` after provisioning.\nThe GitHub App PEM file contents."]
    pub fn pem_file(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pem_file", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `token` after provisioning.\nThe generated token from the credentials."]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.extract_ref()))
    }
}
