use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataRepositoryFileData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    file: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    repository: PrimField<String>,
}

struct DataRepositoryFile_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRepositoryFileData>,
}

#[derive(Clone)]
pub struct DataRepositoryFile(Rc<DataRepositoryFile_>);

impl DataRepositoryFile {
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

    #[doc= "Set the field `branch`.\nThe branch name, defaults to the repository's default branch"]
    pub fn set_branch(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().branch = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\nThe branch name, defaults to the repository's default branch"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit_author` after provisioning.\nThe commit author name, defaults to the authenticated user's name"]
    pub fn commit_author(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_author", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit_email` after provisioning.\nThe commit author email address, defaults to the authenticated user's email address"]
    pub fn commit_email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit_message` after provisioning.\nThe commit message when creating or updating the file"]
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

impl Referable for DataRepositoryFile {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataRepositoryFile { }

impl ToListMappable for DataRepositoryFile {
    type O = ListRef<DataRepositoryFileRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataRepositoryFile_ {
    fn extract_datasource_type(&self) -> String {
        "github_repository_file".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRepositoryFile {
    pub tf_id: String,
    #[doc= "The file path to manage"]
    pub file: PrimField<String>,
    #[doc= "The repository name"]
    pub repository: PrimField<String>,
}

impl BuildDataRepositoryFile {
    pub fn build(self, stack: &mut Stack) -> DataRepositoryFile {
        let out = DataRepositoryFile(Rc::new(DataRepositoryFile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRepositoryFileData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                branch: core::default::Default::default(),
                file: self.file,
                id: core::default::Default::default(),
                repository: self.repository,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRepositoryFileRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRepositoryFileRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRepositoryFileRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\nThe branch name, defaults to the repository's default branch"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit_author` after provisioning.\nThe commit author name, defaults to the authenticated user's name"]
    pub fn commit_author(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_author", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit_email` after provisioning.\nThe commit author email address, defaults to the authenticated user's email address"]
    pub fn commit_email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `commit_message` after provisioning.\nThe commit message when creating or updating the file"]
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
