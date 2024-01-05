use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct ProjectCardData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    column_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<PrimField<String>>,
}

struct ProjectCard_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ProjectCardData>,
}

#[derive(Clone)]
pub struct ProjectCard(Rc<ProjectCard_>);

impl ProjectCard {
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

    #[doc= "Set the field `content_id`.\n'github_issue.issue_id'."]
    pub fn set_content_id(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().content_id = Some(v.into());
        self
    }

    #[doc= "Set the field `content_type`.\nMust be either 'Issue' or 'PullRequest'."]
    pub fn set_content_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().content_type = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `note`.\nThe note contents of the card. Markdown supported."]
    pub fn set_note(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().note = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `card_id` after provisioning.\nThe ID of the card."]
    pub fn card_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.card_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `column_id` after provisioning.\nThe ID of the project column."]
    pub fn column_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.column_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content_id` after provisioning.\n'github_issue.issue_id'."]
    pub fn content_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content_type` after provisioning.\nMust be either 'Issue' or 'PullRequest'."]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `note` after provisioning.\nThe note contents of the card. Markdown supported."]
    pub fn note(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.note", self.extract_ref()))
    }
}

impl Referable for ProjectCard {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ProjectCard { }

impl ToListMappable for ProjectCard {
    type O = ListRef<ProjectCardRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ProjectCard_ {
    fn extract_resource_type(&self) -> String {
        "github_project_card".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProjectCard {
    pub tf_id: String,
    #[doc= "The ID of the project column."]
    pub column_id: PrimField<String>,
}

impl BuildProjectCard {
    pub fn build(self, stack: &mut Stack) -> ProjectCard {
        let out = ProjectCard(Rc::new(ProjectCard_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ProjectCardData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                column_id: self.column_id,
                content_id: core::default::Default::default(),
                content_type: core::default::Default::default(),
                id: core::default::Default::default(),
                note: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ProjectCardRef {
    shared: StackShared,
    base: String,
}

impl Ref for ProjectCardRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ProjectCardRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `card_id` after provisioning.\nThe ID of the card."]
    pub fn card_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.card_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `column_id` after provisioning.\nThe ID of the project column."]
    pub fn column_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.column_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content_id` after provisioning.\n'github_issue.issue_id'."]
    pub fn content_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content_type` after provisioning.\nMust be either 'Issue' or 'PullRequest'."]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `note` after provisioning.\nThe note contents of the card. Markdown supported."]
    pub fn note(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.note", self.extract_ref()))
    }
}
