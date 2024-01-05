use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct ReleaseData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discussion_category_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    draft: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generate_release_notes: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prerelease: Option<PrimField<bool>>,
    repository: PrimField<String>,
    tag_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_commitish: Option<PrimField<String>>,
}

struct Release_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ReleaseData>,
}

#[derive(Clone)]
pub struct Release(Rc<Release_>);

impl Release {
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

    #[doc= "Set the field `body`.\nText describing the contents of the tag."]
    pub fn set_body(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().body = Some(v.into());
        self
    }

    #[doc= "Set the field `discussion_category_name`.\nIf specified, a discussion of the specified category is created and linked to the release. The value must be a category that already exists in the repository."]
    pub fn set_discussion_category_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().discussion_category_name = Some(v.into());
        self
    }

    #[doc= "Set the field `draft`.\nSet to 'false' to create a published release."]
    pub fn set_draft(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().draft = Some(v.into());
        self
    }

    #[doc= "Set the field `generate_release_notes`.\nSet to 'true' to automatically generate the name and body for this release. If 'name' is specified, the specified name will be used; otherwise, a name will be automatically generated. If 'body' is specified, the body will be pre-pended to the automatically generated notes."]
    pub fn set_generate_release_notes(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().generate_release_notes = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\nThe name of the release."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc= "Set the field `prerelease`.\nSet to 'false' to identify the release as a full release."]
    pub fn set_prerelease(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().prerelease = Some(v.into());
        self
    }

    #[doc= "Set the field `target_commitish`.\n The branch name or commit SHA the tag is created from."]
    pub fn set_target_commitish(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().target_commitish = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `body` after provisioning.\nText describing the contents of the tag."]
    pub fn body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.body", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `discussion_category_name` after provisioning.\nIf specified, a discussion of the specified category is created and linked to the release. The value must be a category that already exists in the repository."]
    pub fn discussion_category_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.discussion_category_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `draft` after provisioning.\nSet to 'false' to create a published release."]
    pub fn draft(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.draft", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `generate_release_notes` after provisioning.\nSet to 'true' to automatically generate the name and body for this release. If 'name' is specified, the specified name will be used; otherwise, a name will be automatically generated. If 'body' is specified, the body will be pre-pended to the automatically generated notes."]
    pub fn generate_release_notes(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.generate_release_notes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the release."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `prerelease` after provisioning.\nSet to 'false' to identify the release as a full release."]
    pub fn prerelease(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.prerelease", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\nThe name of the repository."]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag_name` after provisioning.\nThe name of the tag."]
    pub fn tag_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `target_commitish` after provisioning.\n The branch name or commit SHA the tag is created from."]
    pub fn target_commitish(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_commitish", self.extract_ref()))
    }
}

impl Referable for Release {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Release { }

impl ToListMappable for Release {
    type O = ListRef<ReleaseRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Release_ {
    fn extract_resource_type(&self) -> String {
        "github_release".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRelease {
    pub tf_id: String,
    #[doc= "The name of the repository."]
    pub repository: PrimField<String>,
    #[doc= "The name of the tag."]
    pub tag_name: PrimField<String>,
}

impl BuildRelease {
    pub fn build(self, stack: &mut Stack) -> Release {
        let out = Release(Rc::new(Release_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ReleaseData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                body: core::default::Default::default(),
                discussion_category_name: core::default::Default::default(),
                draft: core::default::Default::default(),
                generate_release_notes: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                prerelease: core::default::Default::default(),
                repository: self.repository,
                tag_name: self.tag_name,
                target_commitish: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ReleaseRef {
    shared: StackShared,
    base: String,
}

impl Ref for ReleaseRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ReleaseRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `body` after provisioning.\nText describing the contents of the tag."]
    pub fn body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.body", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `discussion_category_name` after provisioning.\nIf specified, a discussion of the specified category is created and linked to the release. The value must be a category that already exists in the repository."]
    pub fn discussion_category_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.discussion_category_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `draft` after provisioning.\nSet to 'false' to create a published release."]
    pub fn draft(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.draft", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `generate_release_notes` after provisioning.\nSet to 'true' to automatically generate the name and body for this release. If 'name' is specified, the specified name will be used; otherwise, a name will be automatically generated. If 'body' is specified, the body will be pre-pended to the automatically generated notes."]
    pub fn generate_release_notes(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.generate_release_notes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the release."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `prerelease` after provisioning.\nSet to 'false' to identify the release as a full release."]
    pub fn prerelease(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.prerelease", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\nThe name of the repository."]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag_name` after provisioning.\nThe name of the tag."]
    pub fn tag_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `target_commitish` after provisioning.\n The branch name or commit SHA the tag is created from."]
    pub fn target_commitish(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_commitish", self.extract_ref()))
    }
}
