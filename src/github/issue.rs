use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct IssueData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assignees: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    milestone_number: Option<PrimField<f64>>,
    repository: PrimField<String>,
    title: PrimField<String>,
}

struct Issue_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IssueData>,
}

#[derive(Clone)]
pub struct Issue(Rc<Issue_>);

impl Issue {
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

    #[doc= "Set the field `assignees`.\nList of Logins to assign to the issue."]
    pub fn set_assignees(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().assignees = Some(v.into());
        self
    }

    #[doc= "Set the field `body`.\nBody of the issue."]
    pub fn set_body(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().body = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\nList of labels to attach to the issue."]
    pub fn set_labels(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }

    #[doc= "Set the field `milestone_number`.\nMilestone number to assign to the issue."]
    pub fn set_milestone_number(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().milestone_number = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `assignees` after provisioning.\nList of Logins to assign to the issue."]
    pub fn assignees(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.assignees", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `body` after provisioning.\nBody of the issue."]
    pub fn body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.body", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issue_id` after provisioning.\nThe issue id."]
    pub fn issue_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.issue_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nList of labels to attach to the issue."]
    pub fn labels(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `milestone_number` after provisioning.\nMilestone number to assign to the issue."]
    pub fn milestone_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.milestone_number", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `number` after provisioning.\nThe issue number."]
    pub fn number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.number", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\nThe GitHub repository name."]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `title` after provisioning.\nTitle of the issue."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }
}

impl Referable for Issue {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Issue { }

impl ToListMappable for Issue {
    type O = ListRef<IssueRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Issue_ {
    fn extract_resource_type(&self) -> String {
        "github_issue".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIssue {
    pub tf_id: String,
    #[doc= "The GitHub repository name."]
    pub repository: PrimField<String>,
    #[doc= "Title of the issue."]
    pub title: PrimField<String>,
}

impl BuildIssue {
    pub fn build(self, stack: &mut Stack) -> Issue {
        let out = Issue(Rc::new(Issue_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IssueData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                assignees: core::default::Default::default(),
                body: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                milestone_number: core::default::Default::default(),
                repository: self.repository,
                title: self.title,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IssueRef {
    shared: StackShared,
    base: String,
}

impl Ref for IssueRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IssueRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `assignees` after provisioning.\nList of Logins to assign to the issue."]
    pub fn assignees(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.assignees", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `body` after provisioning.\nBody of the issue."]
    pub fn body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.body", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `issue_id` after provisioning.\nThe issue id."]
    pub fn issue_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.issue_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nList of labels to attach to the issue."]
    pub fn labels(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `milestone_number` after provisioning.\nMilestone number to assign to the issue."]
    pub fn milestone_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.milestone_number", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `number` after provisioning.\nThe issue number."]
    pub fn number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.number", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\nThe GitHub repository name."]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `title` after provisioning.\nTitle of the issue."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }
}
