use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataUserData {
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

struct DataUser_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataUserData>,
}

#[derive(Clone)]
pub struct DataUser(Rc<DataUser_>);

impl DataUser {
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

    #[doc= "Get a reference to the value of field `avatar_url` after provisioning.\n"]
    pub fn avatar_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.avatar_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bio` after provisioning.\n"]
    pub fn bio(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bio", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `blog` after provisioning.\n"]
    pub fn blog(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.blog", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `company` after provisioning.\n"]
    pub fn company(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.company", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `followers` after provisioning.\n"]
    pub fn followers(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.followers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `following` after provisioning.\n"]
    pub fn following(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.following", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `gpg_keys` after provisioning.\n"]
    pub fn gpg_keys(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.gpg_keys", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `gravatar_id` after provisioning.\n"]
    pub fn gravatar_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gravatar_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `login` after provisioning.\n"]
    pub fn login(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.login", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_id` after provisioning.\n"]
    pub fn node_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `public_gists` after provisioning.\n"]
    pub fn public_gists(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_gists", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `public_repos` after provisioning.\n"]
    pub fn public_repos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_repos", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `site_admin` after provisioning.\n"]
    pub fn site_admin(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.site_admin", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssh_keys` after provisioning.\n"]
    pub fn ssh_keys(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ssh_keys", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `suspended_at` after provisioning.\n"]
    pub fn suspended_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suspended_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.extract_ref()))
    }
}

impl Referable for DataUser {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataUser { }

impl ToListMappable for DataUser {
    type O = ListRef<DataUserRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataUser_ {
    fn extract_datasource_type(&self) -> String {
        "github_user".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataUser {
    pub tf_id: String,
    #[doc= ""]
    pub username: PrimField<String>,
}

impl BuildDataUser {
    pub fn build(self, stack: &mut Stack) -> DataUser {
        let out = DataUser(Rc::new(DataUser_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataUserData {
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

pub struct DataUserRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataUserRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataUserRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `avatar_url` after provisioning.\n"]
    pub fn avatar_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.avatar_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bio` after provisioning.\n"]
    pub fn bio(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bio", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `blog` after provisioning.\n"]
    pub fn blog(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.blog", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `company` after provisioning.\n"]
    pub fn company(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.company", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `followers` after provisioning.\n"]
    pub fn followers(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.followers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `following` after provisioning.\n"]
    pub fn following(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.following", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `gpg_keys` after provisioning.\n"]
    pub fn gpg_keys(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.gpg_keys", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `gravatar_id` after provisioning.\n"]
    pub fn gravatar_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gravatar_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `login` after provisioning.\n"]
    pub fn login(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.login", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_id` after provisioning.\n"]
    pub fn node_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `public_gists` after provisioning.\n"]
    pub fn public_gists(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_gists", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `public_repos` after provisioning.\n"]
    pub fn public_repos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_repos", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `site_admin` after provisioning.\n"]
    pub fn site_admin(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.site_admin", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssh_keys` after provisioning.\n"]
    pub fn ssh_keys(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ssh_keys", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `suspended_at` after provisioning.\n"]
    pub fn suspended_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suspended_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.extract_ref()))
    }
}
