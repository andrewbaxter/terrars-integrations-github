use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataRepositoryPullRequestData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    base_repository: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    number: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<PrimField<String>>,
}

struct DataRepositoryPullRequest_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRepositoryPullRequestData>,
}

#[derive(Clone)]
pub struct DataRepositoryPullRequest(Rc<DataRepositoryPullRequest_>);

impl DataRepositoryPullRequest {
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

    #[doc= "Set the field `owner`.\n"]
    pub fn set_owner(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().owner = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `base_ref` after provisioning.\n"]
    pub fn base_ref(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.base_ref", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `base_repository` after provisioning.\n"]
    pub fn base_repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.base_repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `base_sha` after provisioning.\n"]
    pub fn base_sha(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.base_sha", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `body` after provisioning.\n"]
    pub fn body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.body", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `draft` after provisioning.\n"]
    pub fn draft(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.draft", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `head_owner` after provisioning.\n"]
    pub fn head_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.head_owner", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `head_ref` after provisioning.\n"]
    pub fn head_ref(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.head_ref", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `head_repository` after provisioning.\n"]
    pub fn head_repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.head_repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `head_sha` after provisioning.\n"]
    pub fn head_sha(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.head_sha", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nList of names of labels on the PR"]
    pub fn labels(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maintainer_can_modify` after provisioning.\n"]
    pub fn maintainer_can_modify(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.maintainer_can_modify", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `number` after provisioning.\n"]
    pub fn number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.number", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `opened_at` after provisioning.\n"]
    pub fn opened_at(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.opened_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `opened_by` after provisioning.\nUsername of the PR creator"]
    pub fn opened_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.opened_by", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

impl Referable for DataRepositoryPullRequest {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataRepositoryPullRequest { }

impl ToListMappable for DataRepositoryPullRequest {
    type O = ListRef<DataRepositoryPullRequestRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataRepositoryPullRequest_ {
    fn extract_datasource_type(&self) -> String {
        "github_repository_pull_request".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRepositoryPullRequest {
    pub tf_id: String,
    #[doc= ""]
    pub base_repository: PrimField<String>,
    #[doc= ""]
    pub number: PrimField<f64>,
}

impl BuildDataRepositoryPullRequest {
    pub fn build(self, stack: &mut Stack) -> DataRepositoryPullRequest {
        let out = DataRepositoryPullRequest(Rc::new(DataRepositoryPullRequest_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRepositoryPullRequestData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                base_repository: self.base_repository,
                id: core::default::Default::default(),
                number: self.number,
                owner: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRepositoryPullRequestRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRepositoryPullRequestRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRepositoryPullRequestRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `base_ref` after provisioning.\n"]
    pub fn base_ref(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.base_ref", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `base_repository` after provisioning.\n"]
    pub fn base_repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.base_repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `base_sha` after provisioning.\n"]
    pub fn base_sha(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.base_sha", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `body` after provisioning.\n"]
    pub fn body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.body", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `draft` after provisioning.\n"]
    pub fn draft(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.draft", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `head_owner` after provisioning.\n"]
    pub fn head_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.head_owner", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `head_ref` after provisioning.\n"]
    pub fn head_ref(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.head_ref", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `head_repository` after provisioning.\n"]
    pub fn head_repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.head_repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `head_sha` after provisioning.\n"]
    pub fn head_sha(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.head_sha", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nList of names of labels on the PR"]
    pub fn labels(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maintainer_can_modify` after provisioning.\n"]
    pub fn maintainer_can_modify(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.maintainer_can_modify", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `number` after provisioning.\n"]
    pub fn number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.number", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `opened_at` after provisioning.\n"]
    pub fn opened_at(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.opened_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `opened_by` after provisioning.\nUsername of the PR creator"]
    pub fn opened_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.opened_by", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}
