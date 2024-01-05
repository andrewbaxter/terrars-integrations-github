use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataRepositoryPullRequestsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base_ref: Option<PrimField<String>>,
    base_repository: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    head_ref: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_by: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_direction: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}

struct DataRepositoryPullRequests_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRepositoryPullRequestsData>,
}

#[derive(Clone)]
pub struct DataRepositoryPullRequests(Rc<DataRepositoryPullRequests_>);

impl DataRepositoryPullRequests {
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

    #[doc= "Set the field `base_ref`.\n"]
    pub fn set_base_ref(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().base_ref = Some(v.into());
        self
    }

    #[doc= "Set the field `head_ref`.\n"]
    pub fn set_head_ref(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().head_ref = Some(v.into());
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

    #[doc= "Set the field `sort_by`.\n"]
    pub fn set_sort_by(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().sort_by = Some(v.into());
        self
    }

    #[doc= "Set the field `sort_direction`.\n"]
    pub fn set_sort_direction(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().sort_direction = Some(v.into());
        self
    }

    #[doc= "Set the field `state`.\n"]
    pub fn set_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().state = Some(v.into());
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

    #[doc= "Get a reference to the value of field `head_ref` after provisioning.\n"]
    pub fn head_ref(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.head_ref", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `results` after provisioning.\n"]
    pub fn results(&self) -> ListRef<DataRepositoryPullRequestsResultsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.results", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort_by` after provisioning.\n"]
    pub fn sort_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sort_by", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort_direction` after provisioning.\n"]
    pub fn sort_direction(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sort_direction", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }
}

impl Referable for DataRepositoryPullRequests {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataRepositoryPullRequests { }

impl ToListMappable for DataRepositoryPullRequests {
    type O = ListRef<DataRepositoryPullRequestsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataRepositoryPullRequests_ {
    fn extract_datasource_type(&self) -> String {
        "github_repository_pull_requests".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRepositoryPullRequests {
    pub tf_id: String,
    #[doc= ""]
    pub base_repository: PrimField<String>,
}

impl BuildDataRepositoryPullRequests {
    pub fn build(self, stack: &mut Stack) -> DataRepositoryPullRequests {
        let out = DataRepositoryPullRequests(Rc::new(DataRepositoryPullRequests_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRepositoryPullRequestsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                base_ref: core::default::Default::default(),
                base_repository: self.base_repository,
                head_ref: core::default::Default::default(),
                id: core::default::Default::default(),
                owner: core::default::Default::default(),
                sort_by: core::default::Default::default(),
                sort_direction: core::default::Default::default(),
                state: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRepositoryPullRequestsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRepositoryPullRequestsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRepositoryPullRequestsRef {
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

    #[doc= "Get a reference to the value of field `head_ref` after provisioning.\n"]
    pub fn head_ref(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.head_ref", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `results` after provisioning.\n"]
    pub fn results(&self) -> ListRef<DataRepositoryPullRequestsResultsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.results", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort_by` after provisioning.\n"]
    pub fn sort_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sort_by", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort_direction` after provisioning.\n"]
    pub fn sort_direction(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sort_direction", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataRepositoryPullRequestsResultsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    base_ref: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base_sha: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    draft: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    head_owner: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    head_ref: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    head_repository: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    head_sha: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintainer_can_modify: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opened_at: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opened_by: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<PrimField<f64>>,
}

impl DataRepositoryPullRequestsResultsEl {
    #[doc= "Set the field `base_ref`.\n"]
    pub fn set_base_ref(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.base_ref = Some(v.into());
        self
    }

    #[doc= "Set the field `base_sha`.\n"]
    pub fn set_base_sha(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.base_sha = Some(v.into());
        self
    }

    #[doc= "Set the field `body`.\n"]
    pub fn set_body(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.body = Some(v.into());
        self
    }

    #[doc= "Set the field `draft`.\n"]
    pub fn set_draft(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.draft = Some(v.into());
        self
    }

    #[doc= "Set the field `head_owner`.\n"]
    pub fn set_head_owner(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.head_owner = Some(v.into());
        self
    }

    #[doc= "Set the field `head_ref`.\n"]
    pub fn set_head_ref(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.head_ref = Some(v.into());
        self
    }

    #[doc= "Set the field `head_repository`.\n"]
    pub fn set_head_repository(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.head_repository = Some(v.into());
        self
    }

    #[doc= "Set the field `head_sha`.\n"]
    pub fn set_head_sha(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.head_sha = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }

    #[doc= "Set the field `maintainer_can_modify`.\n"]
    pub fn set_maintainer_can_modify(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.maintainer_can_modify = Some(v.into());
        self
    }

    #[doc= "Set the field `number`.\n"]
    pub fn set_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.number = Some(v.into());
        self
    }

    #[doc= "Set the field `opened_at`.\n"]
    pub fn set_opened_at(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.opened_at = Some(v.into());
        self
    }

    #[doc= "Set the field `opened_by`.\n"]
    pub fn set_opened_by(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.opened_by = Some(v.into());
        self
    }

    #[doc= "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }

    #[doc= "Set the field `title`.\n"]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }

    #[doc= "Set the field `updated_at`.\n"]
    pub fn set_updated_at(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.updated_at = Some(v.into());
        self
    }
}

impl ToListMappable for DataRepositoryPullRequestsResultsEl {
    type O = BlockAssignable<DataRepositoryPullRequestsResultsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRepositoryPullRequestsResultsEl {}

impl BuildDataRepositoryPullRequestsResultsEl {
    pub fn build(self) -> DataRepositoryPullRequestsResultsEl {
        DataRepositoryPullRequestsResultsEl {
            base_ref: core::default::Default::default(),
            base_sha: core::default::Default::default(),
            body: core::default::Default::default(),
            draft: core::default::Default::default(),
            head_owner: core::default::Default::default(),
            head_ref: core::default::Default::default(),
            head_repository: core::default::Default::default(),
            head_sha: core::default::Default::default(),
            labels: core::default::Default::default(),
            maintainer_can_modify: core::default::Default::default(),
            number: core::default::Default::default(),
            opened_at: core::default::Default::default(),
            opened_by: core::default::Default::default(),
            state: core::default::Default::default(),
            title: core::default::Default::default(),
            updated_at: core::default::Default::default(),
        }
    }
}

pub struct DataRepositoryPullRequestsResultsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRepositoryPullRequestsResultsElRef {
    fn new(shared: StackShared, base: String) -> DataRepositoryPullRequestsResultsElRef {
        DataRepositoryPullRequestsResultsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRepositoryPullRequestsResultsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `base_ref` after provisioning.\n"]
    pub fn base_ref(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.base_ref", self.base))
    }

    #[doc= "Get a reference to the value of field `base_sha` after provisioning.\n"]
    pub fn base_sha(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.base_sha", self.base))
    }

    #[doc= "Get a reference to the value of field `body` after provisioning.\n"]
    pub fn body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.body", self.base))
    }

    #[doc= "Get a reference to the value of field `draft` after provisioning.\n"]
    pub fn draft(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.draft", self.base))
    }

    #[doc= "Get a reference to the value of field `head_owner` after provisioning.\n"]
    pub fn head_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.head_owner", self.base))
    }

    #[doc= "Get a reference to the value of field `head_ref` after provisioning.\n"]
    pub fn head_ref(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.head_ref", self.base))
    }

    #[doc= "Get a reference to the value of field `head_repository` after provisioning.\n"]
    pub fn head_repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.head_repository", self.base))
    }

    #[doc= "Get a reference to the value of field `head_sha` after provisioning.\n"]
    pub fn head_sha(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.head_sha", self.base))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc= "Get a reference to the value of field `maintainer_can_modify` after provisioning.\n"]
    pub fn maintainer_can_modify(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.maintainer_can_modify", self.base))
    }

    #[doc= "Get a reference to the value of field `number` after provisioning.\n"]
    pub fn number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.number", self.base))
    }

    #[doc= "Get a reference to the value of field `opened_at` after provisioning.\n"]
    pub fn opened_at(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.opened_at", self.base))
    }

    #[doc= "Get a reference to the value of field `opened_by` after provisioning.\n"]
    pub fn opened_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.opened_by", self.base))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc= "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }
}
