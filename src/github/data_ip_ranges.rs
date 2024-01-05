use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataIpRangesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

struct DataIpRanges_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataIpRangesData>,
}

#[derive(Clone)]
pub struct DataIpRanges(Rc<DataIpRanges_>);

impl DataIpRanges {
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

    #[doc= "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.actions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `actions_ipv4` after provisioning.\n"]
    pub fn actions_ipv4(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.actions_ipv4", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `actions_ipv6` after provisioning.\n"]
    pub fn actions_ipv6(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.actions_ipv6", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `api` after provisioning.\n"]
    pub fn api(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.api", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `api_ipv4` after provisioning.\n"]
    pub fn api_ipv4(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.api_ipv4", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `api_ipv6` after provisioning.\n"]
    pub fn api_ipv6(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.api_ipv6", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dependabot` after provisioning.\n"]
    pub fn dependabot(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.dependabot", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dependabot_ipv4` after provisioning.\n"]
    pub fn dependabot_ipv4(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.dependabot_ipv4", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dependabot_ipv6` after provisioning.\n"]
    pub fn dependabot_ipv6(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.dependabot_ipv6", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.git", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `git_ipv4` after provisioning.\n"]
    pub fn git_ipv4(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.git_ipv4", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `git_ipv6` after provisioning.\n"]
    pub fn git_ipv6(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.git_ipv6", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hooks` after provisioning.\n"]
    pub fn hooks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.hooks", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hooks_ipv4` after provisioning.\n"]
    pub fn hooks_ipv4(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.hooks_ipv4", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hooks_ipv6` after provisioning.\n"]
    pub fn hooks_ipv6(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.hooks_ipv6", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `importer` after provisioning.\n"]
    pub fn importer(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.importer", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `importer_ipv4` after provisioning.\n"]
    pub fn importer_ipv4(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.importer_ipv4", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `importer_ipv6` after provisioning.\n"]
    pub fn importer_ipv6(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.importer_ipv6", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pages` after provisioning.\n"]
    pub fn pages(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.pages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pages_ipv4` after provisioning.\n"]
    pub fn pages_ipv4(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.pages_ipv4", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pages_ipv6` after provisioning.\n"]
    pub fn pages_ipv6(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.pages_ipv6", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `web` after provisioning.\n"]
    pub fn web(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.web", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `web_ipv4` after provisioning.\n"]
    pub fn web_ipv4(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.web_ipv4", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `web_ipv6` after provisioning.\n"]
    pub fn web_ipv6(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.web_ipv6", self.extract_ref()))
    }
}

impl Referable for DataIpRanges {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataIpRanges { }

impl ToListMappable for DataIpRanges {
    type O = ListRef<DataIpRangesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataIpRanges_ {
    fn extract_datasource_type(&self) -> String {
        "github_ip_ranges".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataIpRanges {
    pub tf_id: String,
}

impl BuildDataIpRanges {
    pub fn build(self, stack: &mut Stack) -> DataIpRanges {
        let out = DataIpRanges(Rc::new(DataIpRanges_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataIpRangesData {
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

pub struct DataIpRangesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataIpRangesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataIpRangesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.actions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `actions_ipv4` after provisioning.\n"]
    pub fn actions_ipv4(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.actions_ipv4", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `actions_ipv6` after provisioning.\n"]
    pub fn actions_ipv6(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.actions_ipv6", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `api` after provisioning.\n"]
    pub fn api(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.api", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `api_ipv4` after provisioning.\n"]
    pub fn api_ipv4(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.api_ipv4", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `api_ipv6` after provisioning.\n"]
    pub fn api_ipv6(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.api_ipv6", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dependabot` after provisioning.\n"]
    pub fn dependabot(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.dependabot", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dependabot_ipv4` after provisioning.\n"]
    pub fn dependabot_ipv4(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.dependabot_ipv4", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dependabot_ipv6` after provisioning.\n"]
    pub fn dependabot_ipv6(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.dependabot_ipv6", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.git", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `git_ipv4` after provisioning.\n"]
    pub fn git_ipv4(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.git_ipv4", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `git_ipv6` after provisioning.\n"]
    pub fn git_ipv6(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.git_ipv6", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hooks` after provisioning.\n"]
    pub fn hooks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.hooks", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hooks_ipv4` after provisioning.\n"]
    pub fn hooks_ipv4(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.hooks_ipv4", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hooks_ipv6` after provisioning.\n"]
    pub fn hooks_ipv6(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.hooks_ipv6", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `importer` after provisioning.\n"]
    pub fn importer(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.importer", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `importer_ipv4` after provisioning.\n"]
    pub fn importer_ipv4(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.importer_ipv4", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `importer_ipv6` after provisioning.\n"]
    pub fn importer_ipv6(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.importer_ipv6", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pages` after provisioning.\n"]
    pub fn pages(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.pages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pages_ipv4` after provisioning.\n"]
    pub fn pages_ipv4(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.pages_ipv4", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pages_ipv6` after provisioning.\n"]
    pub fn pages_ipv6(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.pages_ipv6", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `web` after provisioning.\n"]
    pub fn web(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.web", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `web_ipv4` after provisioning.\n"]
    pub fn web_ipv4(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.web_ipv4", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `web_ipv6` after provisioning.\n"]
    pub fn web_ipv6(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.web_ipv6", self.extract_ref()))
    }
}
