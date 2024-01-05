use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct RepositoryPullRequestData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    base_ref: PrimField<String>,
    base_repository: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<PrimField<String>>,
    head_ref: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintainer_can_modify: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<PrimField<String>>,
    title: PrimField<String>,
}

struct RepositoryPullRequest_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RepositoryPullRequestData>,
}

#[derive(Clone)]
pub struct RepositoryPullRequest(Rc<RepositoryPullRequest_>);

impl RepositoryPullRequest {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderGithub) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `body`.\nBody of the Pull Request."]
    pub fn set_body(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().body = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `maintainer_can_modify`.\nControls whether the base repository maintainers can modify the Pull Request. Default: 'false'."]
    pub fn set_maintainer_can_modify(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().maintainer_can_modify = Some(v.into());
        self
    }

    #[doc= "Set the field `owner`.\nOwner of the repository. If not provided, the provider's default owner is used."]
    pub fn set_owner(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().owner = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `base_ref` after provisioning.\nName of the branch serving as the base of the Pull Request."]
    pub fn base_ref(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.base_ref", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `base_repository` after provisioning.\nName of the base repository to retrieve the Pull Requests from."]
    pub fn base_repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.base_repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `base_sha` after provisioning.\nHead commit SHA of the Pull Request base."]
    pub fn base_sha(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.base_sha", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `body` after provisioning.\nBody of the Pull Request."]
    pub fn body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.body", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `draft` after provisioning.\nIndicates Whether this Pull Request is a draft."]
    pub fn draft(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.draft", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `head_ref` after provisioning.\nName of the branch serving as the head of the Pull Request."]
    pub fn head_ref(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.head_ref", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `head_sha` after provisioning.\nHead commit SHA of the Pull Request head."]
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

    #[doc= "Get a reference to the value of field `maintainer_can_modify` after provisioning.\nControls whether the base repository maintainers can modify the Pull Request. Default: 'false'."]
    pub fn maintainer_can_modify(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.maintainer_can_modify", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `number` after provisioning.\nThe number of the Pull Request within the repository."]
    pub fn number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.number", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `opened_at` after provisioning.\nUnix timestamp indicating the Pull Request creation time."]
    pub fn opened_at(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.opened_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `opened_by` after provisioning.\nUsername of the PR creator"]
    pub fn opened_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.opened_by", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `owner` after provisioning.\nOwner of the repository. If not provided, the provider's default owner is used."]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nThe current Pull Request state - can be 'open', 'closed' or 'merged'."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `title` after provisioning.\nThe title of the Pull Request."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\nThe timestamp of the last Pull Request update."]
    pub fn updated_at(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

impl Referable for RepositoryPullRequest {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for RepositoryPullRequest { }

impl ToListMappable for RepositoryPullRequest {
    type O = ListRef<RepositoryPullRequestRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for RepositoryPullRequest_ {
    fn extract_resource_type(&self) -> String {
        "github_repository_pull_request".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRepositoryPullRequest {
    pub tf_id: String,
    #[doc= "Name of the branch serving as the base of the Pull Request."]
    pub base_ref: PrimField<String>,
    #[doc= "Name of the base repository to retrieve the Pull Requests from."]
    pub base_repository: PrimField<String>,
    #[doc= "Name of the branch serving as the head of the Pull Request."]
    pub head_ref: PrimField<String>,
    #[doc= "The title of the Pull Request."]
    pub title: PrimField<String>,
}

impl BuildRepositoryPullRequest {
    pub fn build(self, stack: &mut Stack) -> RepositoryPullRequest {
        let out = RepositoryPullRequest(Rc::new(RepositoryPullRequest_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RepositoryPullRequestData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                base_ref: self.base_ref,
                base_repository: self.base_repository,
                body: core::default::Default::default(),
                head_ref: self.head_ref,
                id: core::default::Default::default(),
                maintainer_can_modify: core::default::Default::default(),
                owner: core::default::Default::default(),
                title: self.title,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RepositoryPullRequestRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositoryPullRequestRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RepositoryPullRequestRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `base_ref` after provisioning.\nName of the branch serving as the base of the Pull Request."]
    pub fn base_ref(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.base_ref", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `base_repository` after provisioning.\nName of the base repository to retrieve the Pull Requests from."]
    pub fn base_repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.base_repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `base_sha` after provisioning.\nHead commit SHA of the Pull Request base."]
    pub fn base_sha(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.base_sha", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `body` after provisioning.\nBody of the Pull Request."]
    pub fn body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.body", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `draft` after provisioning.\nIndicates Whether this Pull Request is a draft."]
    pub fn draft(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.draft", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `head_ref` after provisioning.\nName of the branch serving as the head of the Pull Request."]
    pub fn head_ref(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.head_ref", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `head_sha` after provisioning.\nHead commit SHA of the Pull Request head."]
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

    #[doc= "Get a reference to the value of field `maintainer_can_modify` after provisioning.\nControls whether the base repository maintainers can modify the Pull Request. Default: 'false'."]
    pub fn maintainer_can_modify(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.maintainer_can_modify", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `number` after provisioning.\nThe number of the Pull Request within the repository."]
    pub fn number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.number", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `opened_at` after provisioning.\nUnix timestamp indicating the Pull Request creation time."]
    pub fn opened_at(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.opened_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `opened_by` after provisioning.\nUsername of the PR creator"]
    pub fn opened_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.opened_by", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `owner` after provisioning.\nOwner of the repository. If not provided, the provider's default owner is used."]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nThe current Pull Request state - can be 'open', 'closed' or 'merged'."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `title` after provisioning.\nThe title of the Pull Request."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\nThe timestamp of the last Pull Request update."]
    pub fn updated_at(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}
