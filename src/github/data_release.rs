use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataReleaseData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    owner: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    release_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    release_tag: Option<PrimField<String>>,
    repository: PrimField<String>,
    retrieve_by: PrimField<String>,
}

struct DataRelease_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataReleaseData>,
}

#[derive(Clone)]
pub struct DataRelease(Rc<DataRelease_>);

impl DataRelease {
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

    #[doc= "Set the field `release_id`.\n"]
    pub fn set_release_id(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().release_id = Some(v.into());
        self
    }

    #[doc= "Set the field `release_tag`.\n"]
    pub fn set_release_tag(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().release_tag = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `asserts_url` after provisioning.\n"]
    pub fn asserts_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.asserts_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `assets` after provisioning.\n"]
    pub fn assets(&self) -> ListRef<DataReleaseAssetsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.assets", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `assets_url` after provisioning.\n"]
    pub fn assets_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.assets_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `body` after provisioning.\n"]
    pub fn body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.body", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `draft` after provisioning.\n"]
    pub fn draft(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.draft", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `html_url` after provisioning.\n"]
    pub fn html_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.html_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `prerelease` after provisioning.\n"]
    pub fn prerelease(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.prerelease", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `published_at` after provisioning.\n"]
    pub fn published_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.published_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `release_id` after provisioning.\n"]
    pub fn release_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.release_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `release_tag` after provisioning.\n"]
    pub fn release_tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.release_tag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `retrieve_by` after provisioning.\n"]
    pub fn retrieve_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.retrieve_by", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tarball_url` after provisioning.\n"]
    pub fn tarball_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tarball_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `target_commitish` after provisioning.\n"]
    pub fn target_commitish(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_commitish", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `upload_url` after provisioning.\n"]
    pub fn upload_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.upload_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zipball_url` after provisioning.\n"]
    pub fn zipball_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zipball_url", self.extract_ref()))
    }
}

impl Referable for DataRelease {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataRelease { }

impl ToListMappable for DataRelease {
    type O = ListRef<DataReleaseRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataRelease_ {
    fn extract_datasource_type(&self) -> String {
        "github_release".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRelease {
    pub tf_id: String,
    #[doc= ""]
    pub owner: PrimField<String>,
    #[doc= ""]
    pub repository: PrimField<String>,
    #[doc= ""]
    pub retrieve_by: PrimField<String>,
}

impl BuildDataRelease {
    pub fn build(self, stack: &mut Stack) -> DataRelease {
        let out = DataRelease(Rc::new(DataRelease_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataReleaseData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                owner: self.owner,
                release_id: core::default::Default::default(),
                release_tag: core::default::Default::default(),
                repository: self.repository,
                retrieve_by: self.retrieve_by,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataReleaseRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataReleaseRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataReleaseRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `asserts_url` after provisioning.\n"]
    pub fn asserts_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.asserts_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `assets` after provisioning.\n"]
    pub fn assets(&self) -> ListRef<DataReleaseAssetsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.assets", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `assets_url` after provisioning.\n"]
    pub fn assets_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.assets_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `body` after provisioning.\n"]
    pub fn body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.body", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `draft` after provisioning.\n"]
    pub fn draft(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.draft", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `html_url` after provisioning.\n"]
    pub fn html_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.html_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `prerelease` after provisioning.\n"]
    pub fn prerelease(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.prerelease", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `published_at` after provisioning.\n"]
    pub fn published_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.published_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `release_id` after provisioning.\n"]
    pub fn release_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.release_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `release_tag` after provisioning.\n"]
    pub fn release_tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.release_tag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `retrieve_by` after provisioning.\n"]
    pub fn retrieve_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.retrieve_by", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tarball_url` after provisioning.\n"]
    pub fn tarball_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tarball_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `target_commitish` after provisioning.\n"]
    pub fn target_commitish(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_commitish", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `upload_url` after provisioning.\n"]
    pub fn upload_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.upload_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zipball_url` after provisioning.\n"]
    pub fn zipball_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zipball_url", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataReleaseAssetsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    browser_download_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<PrimField<String>>,
}

impl DataReleaseAssetsEl {
    #[doc= "Set the field `browser_download_url`.\n"]
    pub fn set_browser_download_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.browser_download_url = Some(v.into());
        self
    }

    #[doc= "Set the field `content_type`.\n"]
    pub fn set_content_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content_type = Some(v.into());
        self
    }

    #[doc= "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created_at = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `label`.\n"]
    pub fn set_label(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.label = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `node_id`.\n"]
    pub fn set_node_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_id = Some(v.into());
        self
    }

    #[doc= "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size = Some(v.into());
        self
    }

    #[doc= "Set the field `updated_at`.\n"]
    pub fn set_updated_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.updated_at = Some(v.into());
        self
    }

    #[doc= "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url = Some(v.into());
        self
    }
}

impl ToListMappable for DataReleaseAssetsEl {
    type O = BlockAssignable<DataReleaseAssetsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataReleaseAssetsEl {}

impl BuildDataReleaseAssetsEl {
    pub fn build(self) -> DataReleaseAssetsEl {
        DataReleaseAssetsEl {
            browser_download_url: core::default::Default::default(),
            content_type: core::default::Default::default(),
            created_at: core::default::Default::default(),
            id: core::default::Default::default(),
            label: core::default::Default::default(),
            name: core::default::Default::default(),
            node_id: core::default::Default::default(),
            size: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            url: core::default::Default::default(),
        }
    }
}

pub struct DataReleaseAssetsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataReleaseAssetsElRef {
    fn new(shared: StackShared, base: String) -> DataReleaseAssetsElRef {
        DataReleaseAssetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataReleaseAssetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `browser_download_url` after provisioning.\n"]
    pub fn browser_download_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.browser_download_url", self.base))
    }

    #[doc= "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `label` after provisioning.\n"]
    pub fn label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `node_id` after provisioning.\n"]
    pub fn node_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_id", self.base))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
