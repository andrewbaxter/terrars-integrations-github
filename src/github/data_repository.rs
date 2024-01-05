use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataRepositoryData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    full_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    homepage_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

struct DataRepository_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRepositoryData>,
}

#[derive(Clone)]
pub struct DataRepository(Rc<DataRepository_>);

impl DataRepository {
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

    #[doc= "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `full_name`.\n"]
    pub fn set_full_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().full_name = Some(v.into());
        self
    }

    #[doc= "Set the field `homepage_url`.\n"]
    pub fn set_homepage_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().homepage_url = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `allow_auto_merge` after provisioning.\n"]
    pub fn allow_auto_merge(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_auto_merge", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allow_merge_commit` after provisioning.\n"]
    pub fn allow_merge_commit(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_merge_commit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allow_rebase_merge` after provisioning.\n"]
    pub fn allow_rebase_merge(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_rebase_merge", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allow_squash_merge` after provisioning.\n"]
    pub fn allow_squash_merge(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_squash_merge", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `archived` after provisioning.\n"]
    pub fn archived(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.archived", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_branch` after provisioning.\n"]
    pub fn default_branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_branch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fork` after provisioning.\n"]
    pub fn fork(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.fork", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `full_name` after provisioning.\n"]
    pub fn full_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.full_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `git_clone_url` after provisioning.\n"]
    pub fn git_clone_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.git_clone_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `has_discussions` after provisioning.\n"]
    pub fn has_discussions(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.has_discussions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `has_downloads` after provisioning.\n"]
    pub fn has_downloads(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.has_downloads", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `has_issues` after provisioning.\n"]
    pub fn has_issues(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.has_issues", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `has_projects` after provisioning.\n"]
    pub fn has_projects(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.has_projects", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `has_wiki` after provisioning.\n"]
    pub fn has_wiki(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.has_wiki", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `homepage_url` after provisioning.\n"]
    pub fn homepage_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.homepage_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `html_url` after provisioning.\n"]
    pub fn html_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.html_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `http_clone_url` after provisioning.\n"]
    pub fn http_clone_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_clone_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `is_template` after provisioning.\n"]
    pub fn is_template(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_template", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_commit_message` after provisioning.\n"]
    pub fn merge_commit_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_commit_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_commit_title` after provisioning.\n"]
    pub fn merge_commit_title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_commit_title", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_id` after provisioning.\n"]
    pub fn node_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pages` after provisioning.\n"]
    pub fn pages(&self) -> ListRef<DataRepositoryPagesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `primary_language` after provisioning.\n"]
    pub fn primary_language(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.primary_language", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private` after provisioning.\n"]
    pub fn private(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.private", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repo_id` after provisioning.\n"]
    pub fn repo_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `squash_merge_commit_message` after provisioning.\n"]
    pub fn squash_merge_commit_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.squash_merge_commit_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `squash_merge_commit_title` after provisioning.\n"]
    pub fn squash_merge_commit_title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.squash_merge_commit_title", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssh_clone_url` after provisioning.\n"]
    pub fn ssh_clone_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssh_clone_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `svn_url` after provisioning.\n"]
    pub fn svn_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.svn_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `template` after provisioning.\n"]
    pub fn template(&self) -> ListRef<DataRepositoryTemplateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.template", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `topics` after provisioning.\n"]
    pub fn topics(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.topics", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `visibility` after provisioning.\n"]
    pub fn visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.extract_ref()))
    }
}

impl Referable for DataRepository {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataRepository { }

impl ToListMappable for DataRepository {
    type O = ListRef<DataRepositoryRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataRepository_ {
    fn extract_datasource_type(&self) -> String {
        "github_repository".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRepository {
    pub tf_id: String,
}

impl BuildDataRepository {
    pub fn build(self, stack: &mut Stack) -> DataRepository {
        let out = DataRepository(Rc::new(DataRepository_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRepositoryData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                description: core::default::Default::default(),
                full_name: core::default::Default::default(),
                homepage_url: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRepositoryRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRepositoryRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRepositoryRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `allow_auto_merge` after provisioning.\n"]
    pub fn allow_auto_merge(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_auto_merge", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allow_merge_commit` after provisioning.\n"]
    pub fn allow_merge_commit(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_merge_commit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allow_rebase_merge` after provisioning.\n"]
    pub fn allow_rebase_merge(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_rebase_merge", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allow_squash_merge` after provisioning.\n"]
    pub fn allow_squash_merge(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_squash_merge", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `archived` after provisioning.\n"]
    pub fn archived(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.archived", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_branch` after provisioning.\n"]
    pub fn default_branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_branch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fork` after provisioning.\n"]
    pub fn fork(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.fork", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `full_name` after provisioning.\n"]
    pub fn full_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.full_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `git_clone_url` after provisioning.\n"]
    pub fn git_clone_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.git_clone_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `has_discussions` after provisioning.\n"]
    pub fn has_discussions(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.has_discussions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `has_downloads` after provisioning.\n"]
    pub fn has_downloads(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.has_downloads", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `has_issues` after provisioning.\n"]
    pub fn has_issues(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.has_issues", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `has_projects` after provisioning.\n"]
    pub fn has_projects(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.has_projects", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `has_wiki` after provisioning.\n"]
    pub fn has_wiki(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.has_wiki", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `homepage_url` after provisioning.\n"]
    pub fn homepage_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.homepage_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `html_url` after provisioning.\n"]
    pub fn html_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.html_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `http_clone_url` after provisioning.\n"]
    pub fn http_clone_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_clone_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `is_template` after provisioning.\n"]
    pub fn is_template(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_template", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_commit_message` after provisioning.\n"]
    pub fn merge_commit_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_commit_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_commit_title` after provisioning.\n"]
    pub fn merge_commit_title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_commit_title", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_id` after provisioning.\n"]
    pub fn node_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pages` after provisioning.\n"]
    pub fn pages(&self) -> ListRef<DataRepositoryPagesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `primary_language` after provisioning.\n"]
    pub fn primary_language(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.primary_language", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private` after provisioning.\n"]
    pub fn private(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.private", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repo_id` after provisioning.\n"]
    pub fn repo_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `squash_merge_commit_message` after provisioning.\n"]
    pub fn squash_merge_commit_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.squash_merge_commit_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `squash_merge_commit_title` after provisioning.\n"]
    pub fn squash_merge_commit_title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.squash_merge_commit_title", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssh_clone_url` after provisioning.\n"]
    pub fn ssh_clone_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssh_clone_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `svn_url` after provisioning.\n"]
    pub fn svn_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.svn_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `template` after provisioning.\n"]
    pub fn template(&self) -> ListRef<DataRepositoryTemplateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.template", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `topics` after provisioning.\n"]
    pub fn topics(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.topics", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `visibility` after provisioning.\n"]
    pub fn visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataRepositoryPagesElSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
}

impl DataRepositoryPagesElSourceEl {
    #[doc= "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }

    #[doc= "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
}

impl ToListMappable for DataRepositoryPagesElSourceEl {
    type O = BlockAssignable<DataRepositoryPagesElSourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRepositoryPagesElSourceEl {}

impl BuildDataRepositoryPagesElSourceEl {
    pub fn build(self) -> DataRepositoryPagesElSourceEl {
        DataRepositoryPagesElSourceEl {
            branch: core::default::Default::default(),
            path: core::default::Default::default(),
        }
    }
}

pub struct DataRepositoryPagesElSourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRepositoryPagesElSourceElRef {
    fn new(shared: StackShared, base: String) -> DataRepositoryPagesElSourceElRef {
        DataRepositoryPagesElSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRepositoryPagesElSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRepositoryPagesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    build_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cname: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_404: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    html_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<ListField<DataRepositoryPagesElSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<PrimField<String>>,
}

impl DataRepositoryPagesEl {
    #[doc= "Set the field `build_type`.\n"]
    pub fn set_build_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.build_type = Some(v.into());
        self
    }

    #[doc= "Set the field `cname`.\n"]
    pub fn set_cname(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cname = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_404`.\n"]
    pub fn set_custom_404(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.custom_404 = Some(v.into());
        self
    }

    #[doc= "Set the field `html_url`.\n"]
    pub fn set_html_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.html_url = Some(v.into());
        self
    }

    #[doc= "Set the field `source`.\n"]
    pub fn set_source(mut self, v: impl Into<ListField<DataRepositoryPagesElSourceEl>>) -> Self {
        self.source = Some(v.into());
        self
    }

    #[doc= "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }

    #[doc= "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url = Some(v.into());
        self
    }
}

impl ToListMappable for DataRepositoryPagesEl {
    type O = BlockAssignable<DataRepositoryPagesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRepositoryPagesEl {}

impl BuildDataRepositoryPagesEl {
    pub fn build(self) -> DataRepositoryPagesEl {
        DataRepositoryPagesEl {
            build_type: core::default::Default::default(),
            cname: core::default::Default::default(),
            custom_404: core::default::Default::default(),
            html_url: core::default::Default::default(),
            source: core::default::Default::default(),
            status: core::default::Default::default(),
            url: core::default::Default::default(),
        }
    }
}

pub struct DataRepositoryPagesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRepositoryPagesElRef {
    fn new(shared: StackShared, base: String) -> DataRepositoryPagesElRef {
        DataRepositoryPagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRepositoryPagesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `build_type` after provisioning.\n"]
    pub fn build_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.build_type", self.base))
    }

    #[doc= "Get a reference to the value of field `cname` after provisioning.\n"]
    pub fn cname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cname", self.base))
    }

    #[doc= "Get a reference to the value of field `custom_404` after provisioning.\n"]
    pub fn custom_404(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_404", self.base))
    }

    #[doc= "Get a reference to the value of field `html_url` after provisioning.\n"]
    pub fn html_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.html_url", self.base))
    }

    #[doc= "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<DataRepositoryPagesElSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRepositoryTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository: Option<PrimField<String>>,
}

impl DataRepositoryTemplateEl {
    #[doc= "Set the field `owner`.\n"]
    pub fn set_owner(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.owner = Some(v.into());
        self
    }

    #[doc= "Set the field `repository`.\n"]
    pub fn set_repository(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repository = Some(v.into());
        self
    }
}

impl ToListMappable for DataRepositoryTemplateEl {
    type O = BlockAssignable<DataRepositoryTemplateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRepositoryTemplateEl {}

impl BuildDataRepositoryTemplateEl {
    pub fn build(self) -> DataRepositoryTemplateEl {
        DataRepositoryTemplateEl {
            owner: core::default::Default::default(),
            repository: core::default::Default::default(),
        }
    }
}

pub struct DataRepositoryTemplateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRepositoryTemplateElRef {
    fn new(shared: StackShared, base: String) -> DataRepositoryTemplateElRef {
        DataRepositoryTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRepositoryTemplateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.base))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }
}
