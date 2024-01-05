use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct RepositoryFileData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_author: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_message: Option<PrimField<String>>,
    content: PrimField<String>,
    file: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overwrite_on_create: Option<PrimField<bool>>,
    repository: PrimField<String>,
}

struct RepositoryFile_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RepositoryFileData>,
}

#[derive(Clone)]
pub struct RepositoryFile(Rc<RepositoryFile_>);

impl RepositoryFile {
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

    #[doc= "Set the field `branch`.\nThe branch name, defaults to the repository's default branch"]
    pub fn set_branch(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().branch = Some(v.into());
        self
    }

    #[doc= "Set the field `commit_author`.\nThe commit author name, defaults to the authenticated user's name. GitHub app users may omit author and email information so GitHub can verify commits as the GitHub App. "]
    pub fn set_commit_author(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().commit_author = Some(v.into());
        self
    }

    #[doc= "Set the field `commit_email`.\nThe commit author email address, defaults to the authenticated user's email address. GitHub app users may omit author and email information so GitHub can verify commits as the GitHub App."]
    pub fn set_commit_email(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().commit_email = Some(v.into());
        self
    }

    #[doc= "Set the field `commit_message`.\nThe commit message when creating, updating or deleting the file"]
    pub fn set_commit_message(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().commit_message = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `overwrite_on_create`.\nEnable overwriting existing files, defaults to \"false\""]
    pub fn set_overwrite_on_create(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().overwrite_on_create = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\nThe branch name, defaults to the repository's default branch"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit_author` after provisioning.\nThe commit author name, defaults to the authenticated user's name. GitHub app users may omit author and email information so GitHub can verify commits as the GitHub App. "]
    pub fn commit_author(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_author", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit_email` after provisioning.\nThe commit author email address, defaults to the authenticated user's email address. GitHub app users may omit author and email information so GitHub can verify commits as the GitHub App."]
    pub fn commit_email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit_message` after provisioning.\nThe commit message when creating, updating or deleting the file"]
    pub fn commit_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit_sha` after provisioning.\nThe SHA of the commit that modified the file"]
    pub fn commit_sha(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_sha", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content` after provisioning.\nThe file's content"]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `file` after provisioning.\nThe file path to manage"]
    pub fn file(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `overwrite_on_create` after provisioning.\nEnable overwriting existing files, defaults to \"false\""]
    pub fn overwrite_on_create(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.overwrite_on_create", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ref_` after provisioning.\nThe name of the commit/branch/tag"]
    pub fn ref_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ref", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\nThe repository name"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sha` after provisioning.\nThe blob SHA of the file"]
    pub fn sha(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sha", self.extract_ref()))
    }
}

impl Referable for RepositoryFile {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for RepositoryFile { }

impl ToListMappable for RepositoryFile {
    type O = ListRef<RepositoryFileRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for RepositoryFile_ {
    fn extract_resource_type(&self) -> String {
        "github_repository_file".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRepositoryFile {
    pub tf_id: String,
    #[doc= "The file's content"]
    pub content: PrimField<String>,
    #[doc= "The file path to manage"]
    pub file: PrimField<String>,
    #[doc= "The repository name"]
    pub repository: PrimField<String>,
}

impl BuildRepositoryFile {
    pub fn build(self, stack: &mut Stack) -> RepositoryFile {
        let out = RepositoryFile(Rc::new(RepositoryFile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RepositoryFileData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                branch: core::default::Default::default(),
                commit_author: core::default::Default::default(),
                commit_email: core::default::Default::default(),
                commit_message: core::default::Default::default(),
                content: self.content,
                file: self.file,
                id: core::default::Default::default(),
                overwrite_on_create: core::default::Default::default(),
                repository: self.repository,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RepositoryFileRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositoryFileRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RepositoryFileRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\nThe branch name, defaults to the repository's default branch"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit_author` after provisioning.\nThe commit author name, defaults to the authenticated user's name. GitHub app users may omit author and email information so GitHub can verify commits as the GitHub App. "]
    pub fn commit_author(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_author", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit_email` after provisioning.\nThe commit author email address, defaults to the authenticated user's email address. GitHub app users may omit author and email information so GitHub can verify commits as the GitHub App."]
    pub fn commit_email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit_message` after provisioning.\nThe commit message when creating, updating or deleting the file"]
    pub fn commit_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit_sha` after provisioning.\nThe SHA of the commit that modified the file"]
    pub fn commit_sha(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_sha", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content` after provisioning.\nThe file's content"]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `file` after provisioning.\nThe file path to manage"]
    pub fn file(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `overwrite_on_create` after provisioning.\nEnable overwriting existing files, defaults to \"false\""]
    pub fn overwrite_on_create(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.overwrite_on_create", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ref_` after provisioning.\nThe name of the commit/branch/tag"]
    pub fn ref_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ref", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\nThe repository name"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sha` after provisioning.\nThe blob SHA of the file"]
    pub fn sha(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sha", self.extract_ref()))
    }
}
