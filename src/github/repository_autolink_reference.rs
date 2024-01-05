use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct RepositoryAutolinkReferenceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_alphanumeric: Option<PrimField<bool>>,
    key_prefix: PrimField<String>,
    repository: PrimField<String>,
    target_url_template: PrimField<String>,
}

struct RepositoryAutolinkReference_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RepositoryAutolinkReferenceData>,
}

#[derive(Clone)]
pub struct RepositoryAutolinkReference(Rc<RepositoryAutolinkReference_>);

impl RepositoryAutolinkReference {
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

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `is_alphanumeric`.\nWhether this autolink reference matches alphanumeric characters. If false, this autolink reference only matches numeric characters."]
    pub fn set_is_alphanumeric(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_alphanumeric = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `is_alphanumeric` after provisioning.\nWhether this autolink reference matches alphanumeric characters. If false, this autolink reference only matches numeric characters."]
    pub fn is_alphanumeric(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_alphanumeric", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key_prefix` after provisioning.\nThis prefix appended by a number will generate a link any time it is found in an issue, pull request, or commit"]
    pub fn key_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_prefix", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\nThe repository name"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `target_url_template` after provisioning.\nThe template of the target URL used for the links; must be a valid URL and contain `<num>` for the reference number"]
    pub fn target_url_template(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_url_template", self.extract_ref()))
    }
}

impl Referable for RepositoryAutolinkReference {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for RepositoryAutolinkReference { }

impl ToListMappable for RepositoryAutolinkReference {
    type O = ListRef<RepositoryAutolinkReferenceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for RepositoryAutolinkReference_ {
    fn extract_resource_type(&self) -> String {
        "github_repository_autolink_reference".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRepositoryAutolinkReference {
    pub tf_id: String,
    #[doc= "This prefix appended by a number will generate a link any time it is found in an issue, pull request, or commit"]
    pub key_prefix: PrimField<String>,
    #[doc= "The repository name"]
    pub repository: PrimField<String>,
    #[doc= "The template of the target URL used for the links; must be a valid URL and contain `<num>` for the reference number"]
    pub target_url_template: PrimField<String>,
}

impl BuildRepositoryAutolinkReference {
    pub fn build(self, stack: &mut Stack) -> RepositoryAutolinkReference {
        let out = RepositoryAutolinkReference(Rc::new(RepositoryAutolinkReference_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RepositoryAutolinkReferenceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                is_alphanumeric: core::default::Default::default(),
                key_prefix: self.key_prefix,
                repository: self.repository,
                target_url_template: self.target_url_template,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RepositoryAutolinkReferenceRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositoryAutolinkReferenceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RepositoryAutolinkReferenceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `is_alphanumeric` after provisioning.\nWhether this autolink reference matches alphanumeric characters. If false, this autolink reference only matches numeric characters."]
    pub fn is_alphanumeric(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_alphanumeric", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key_prefix` after provisioning.\nThis prefix appended by a number will generate a link any time it is found in an issue, pull request, or commit"]
    pub fn key_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_prefix", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\nThe repository name"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `target_url_template` after provisioning.\nThe template of the target URL used for the links; must be a valid URL and contain `<num>` for the reference number"]
    pub fn target_url_template(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_url_template", self.extract_ref()))
    }
}
