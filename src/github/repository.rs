use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct RepositoryData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_auto_merge: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_merge_commit: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_rebase_merge: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_squash_merge: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_update_branch: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    archive_on_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    archived: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_init: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_branch_on_merge: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gitignore_template: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_discussions: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_downloads: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_issues: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_projects: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_wiki: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    homepage_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_vulnerability_alerts_during_read: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_template: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    license_template: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_commit_message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_commit_title: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    squash_merge_commit_message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    squash_merge_commit_title: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topics: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vulnerability_alerts: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_commit_signoff_required: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pages: Option<Vec<RepositoryPagesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_and_analysis: Option<Vec<RepositorySecurityAndAnalysisEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template: Option<Vec<RepositoryTemplateEl>>,
    dynamic: RepositoryDynamic,
}

struct Repository_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RepositoryData>,
}

#[derive(Clone)]
pub struct Repository(Rc<Repository_>);

impl Repository {
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

    #[doc= "Set the field `allow_auto_merge`.\nSet to 'true' to allow auto-merging pull requests on the repository."]
    pub fn set_allow_auto_merge(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_auto_merge = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_merge_commit`.\nSet to 'false' to disable merge commits on the repository."]
    pub fn set_allow_merge_commit(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_merge_commit = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_rebase_merge`.\nSet to 'false' to disable rebase merges on the repository."]
    pub fn set_allow_rebase_merge(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_rebase_merge = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_squash_merge`.\nSet to 'false' to disable squash merges on the repository."]
    pub fn set_allow_squash_merge(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_squash_merge = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_update_branch`.\n Set to 'true' to always suggest updating pull request branches."]
    pub fn set_allow_update_branch(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_update_branch = Some(v.into());
        self
    }

    #[doc= "Set the field `archive_on_destroy`.\nSet to 'true' to archive the repository instead of deleting on destroy."]
    pub fn set_archive_on_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().archive_on_destroy = Some(v.into());
        self
    }

    #[doc= "Set the field `archived`.\nSpecifies if the repository should be archived. Defaults to 'false'. NOTE Currently, the API does not support unarchiving."]
    pub fn set_archived(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().archived = Some(v.into());
        self
    }

    #[doc= "Set the field `auto_init`.\nSet to 'true' to produce an initial commit in the repository."]
    pub fn set_auto_init(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_init = Some(v.into());
        self
    }

    #[doc= "Set the field `default_branch`.\nCan only be set after initial repository creation, and only if the target branch exists"]
    pub fn set_default_branch(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_branch = Some(v.into());
        self
    }

    #[doc= "Set the field `delete_branch_on_merge`.\nAutomatically delete head branch after a pull request is merged. Defaults to 'false'."]
    pub fn set_delete_branch_on_merge(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_branch_on_merge = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nA description of the repository."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `gitignore_template`.\nUse the name of the template without the extension. For example, 'Haskell'."]
    pub fn set_gitignore_template(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().gitignore_template = Some(v.into());
        self
    }

    #[doc= "Set the field `has_discussions`.\nSet to 'true' to enable GitHub Discussions on the repository. Defaults to 'false'."]
    pub fn set_has_discussions(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().has_discussions = Some(v.into());
        self
    }

    #[doc= "Set the field `has_downloads`.\nSet to 'true' to enable the (deprecated) downloads features on the repository."]
    pub fn set_has_downloads(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().has_downloads = Some(v.into());
        self
    }

    #[doc= "Set the field `has_issues`.\nSet to 'true' to enable the GitHub Issues features on the repository"]
    pub fn set_has_issues(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().has_issues = Some(v.into());
        self
    }

    #[doc= "Set the field `has_projects`.\nSet to 'true' to enable the GitHub Projects features on the repository. Per the GitHub documentation when in an organization that has disabled repository projects it will default to 'false' and will otherwise default to 'true'. If you specify 'true' when it has been disabled it will return an error."]
    pub fn set_has_projects(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().has_projects = Some(v.into());
        self
    }

    #[doc= "Set the field `has_wiki`.\nSet to 'true' to enable the GitHub Wiki features on the repository."]
    pub fn set_has_wiki(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().has_wiki = Some(v.into());
        self
    }

    #[doc= "Set the field `homepage_url`.\nURL of a page describing the project."]
    pub fn set_homepage_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().homepage_url = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `ignore_vulnerability_alerts_during_read`.\nSet to true to not call the vulnerability alerts endpoint so the resource can also be used without admin permissions during read."]
    pub fn set_ignore_vulnerability_alerts_during_read(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ignore_vulnerability_alerts_during_read = Some(v.into());
        self
    }

    #[doc= "Set the field `is_template`.\nSet to 'true' to tell GitHub that this is a template repository."]
    pub fn set_is_template(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_template = Some(v.into());
        self
    }

    #[doc= "Set the field `license_template`.\nUse the name of the template without the extension. For example, 'mit' or 'mpl-2.0'."]
    pub fn set_license_template(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().license_template = Some(v.into());
        self
    }

    #[doc= "Set the field `merge_commit_message`.\nCan be 'PR_BODY', 'PR_TITLE', or 'BLANK' for a default merge commit message."]
    pub fn set_merge_commit_message(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().merge_commit_message = Some(v.into());
        self
    }

    #[doc= "Set the field `merge_commit_title`.\nCan be 'PR_TITLE' or 'MERGE_MESSAGE' for a default merge commit title."]
    pub fn set_merge_commit_title(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().merge_commit_title = Some(v.into());
        self
    }

    #[doc= "Set the field `private`.\n"]
    pub fn set_private(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().private = Some(v.into());
        self
    }

    #[doc= "Set the field `squash_merge_commit_message`.\nCan be 'PR_BODY', 'COMMIT_MESSAGES', or 'BLANK' for a default squash merge commit message."]
    pub fn set_squash_merge_commit_message(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().squash_merge_commit_message = Some(v.into());
        self
    }

    #[doc= "Set the field `squash_merge_commit_title`.\nCan be 'PR_TITLE' or 'COMMIT_OR_PR_TITLE' for a default squash merge commit title."]
    pub fn set_squash_merge_commit_title(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().squash_merge_commit_title = Some(v.into());
        self
    }

    #[doc= "Set the field `topics`.\nThe list of topics of the repository."]
    pub fn set_topics(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().topics = Some(v.into());
        self
    }

    #[doc= "Set the field `visibility`.\nCan be 'public' or 'private'. If your organization is associated with an enterprise account using GitHub Enterprise Cloud or GitHub Enterprise Server 2.20+, visibility can also be 'internal'."]
    pub fn set_visibility(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().visibility = Some(v.into());
        self
    }

    #[doc= "Set the field `vulnerability_alerts`.\nSet to 'true' to enable security alerts for vulnerable dependencies. Enabling requires alerts to be enabled on the owner level. (Note for importing: GitHub enables the alerts on public repos but disables them on private repos by default). Note that vulnerability alerts have not been successfully tested on any GitHub Enterprise instance and may be unavailable in those settings."]
    pub fn set_vulnerability_alerts(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().vulnerability_alerts = Some(v.into());
        self
    }

    #[doc= "Set the field `web_commit_signoff_required`.\nRequire contributors to sign off on web-based commits. Defaults to 'false'."]
    pub fn set_web_commit_signoff_required(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().web_commit_signoff_required = Some(v.into());
        self
    }

    #[doc= "Set the field `pages`.\n"]
    pub fn set_pages(self, v: impl Into<BlockAssignable<RepositoryPagesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pages = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pages = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `security_and_analysis`.\n"]
    pub fn set_security_and_analysis(self, v: impl Into<BlockAssignable<RepositorySecurityAndAnalysisEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().security_and_analysis = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.security_and_analysis = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `template`.\n"]
    pub fn set_template(self, v: impl Into<BlockAssignable<RepositoryTemplateEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().template = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.template = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `allow_auto_merge` after provisioning.\nSet to 'true' to allow auto-merging pull requests on the repository."]
    pub fn allow_auto_merge(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_auto_merge", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allow_merge_commit` after provisioning.\nSet to 'false' to disable merge commits on the repository."]
    pub fn allow_merge_commit(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_merge_commit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allow_rebase_merge` after provisioning.\nSet to 'false' to disable rebase merges on the repository."]
    pub fn allow_rebase_merge(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_rebase_merge", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allow_squash_merge` after provisioning.\nSet to 'false' to disable squash merges on the repository."]
    pub fn allow_squash_merge(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_squash_merge", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allow_update_branch` after provisioning.\n Set to 'true' to always suggest updating pull request branches."]
    pub fn allow_update_branch(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_update_branch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `archive_on_destroy` after provisioning.\nSet to 'true' to archive the repository instead of deleting on destroy."]
    pub fn archive_on_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.archive_on_destroy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `archived` after provisioning.\nSpecifies if the repository should be archived. Defaults to 'false'. NOTE Currently, the API does not support unarchiving."]
    pub fn archived(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.archived", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_init` after provisioning.\nSet to 'true' to produce an initial commit in the repository."]
    pub fn auto_init(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_init", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_branch` after provisioning.\nCan only be set after initial repository creation, and only if the target branch exists"]
    pub fn default_branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_branch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delete_branch_on_merge` after provisioning.\nAutomatically delete head branch after a pull request is merged. Defaults to 'false'."]
    pub fn delete_branch_on_merge(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_branch_on_merge", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA description of the repository."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `full_name` after provisioning.\nA string of the form 'orgname/reponame'."]
    pub fn full_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.full_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `git_clone_url` after provisioning.\nURL that can be provided to 'git clone' to clone the repository anonymously via the git protocol."]
    pub fn git_clone_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.git_clone_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `gitignore_template` after provisioning.\nUse the name of the template without the extension. For example, 'Haskell'."]
    pub fn gitignore_template(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gitignore_template", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `has_discussions` after provisioning.\nSet to 'true' to enable GitHub Discussions on the repository. Defaults to 'false'."]
    pub fn has_discussions(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.has_discussions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `has_downloads` after provisioning.\nSet to 'true' to enable the (deprecated) downloads features on the repository."]
    pub fn has_downloads(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.has_downloads", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `has_issues` after provisioning.\nSet to 'true' to enable the GitHub Issues features on the repository"]
    pub fn has_issues(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.has_issues", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `has_projects` after provisioning.\nSet to 'true' to enable the GitHub Projects features on the repository. Per the GitHub documentation when in an organization that has disabled repository projects it will default to 'false' and will otherwise default to 'true'. If you specify 'true' when it has been disabled it will return an error."]
    pub fn has_projects(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.has_projects", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `has_wiki` after provisioning.\nSet to 'true' to enable the GitHub Wiki features on the repository."]
    pub fn has_wiki(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.has_wiki", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `homepage_url` after provisioning.\nURL of a page describing the project."]
    pub fn homepage_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.homepage_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `html_url` after provisioning.\nURL to the repository on the web."]
    pub fn html_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.html_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `http_clone_url` after provisioning.\nURL that can be provided to 'git clone' to clone the repository via HTTPS."]
    pub fn http_clone_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_clone_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ignore_vulnerability_alerts_during_read` after provisioning.\nSet to true to not call the vulnerability alerts endpoint so the resource can also be used without admin permissions during read."]
    pub fn ignore_vulnerability_alerts_during_read(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_vulnerability_alerts_during_read", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `is_template` after provisioning.\nSet to 'true' to tell GitHub that this is a template repository."]
    pub fn is_template(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_template", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `license_template` after provisioning.\nUse the name of the template without the extension. For example, 'mit' or 'mpl-2.0'."]
    pub fn license_template(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.license_template", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_commit_message` after provisioning.\nCan be 'PR_BODY', 'PR_TITLE', or 'BLANK' for a default merge commit message."]
    pub fn merge_commit_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_commit_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_commit_title` after provisioning.\nCan be 'PR_TITLE' or 'MERGE_MESSAGE' for a default merge commit title."]
    pub fn merge_commit_title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_commit_title", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the repository."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_id` after provisioning.\nGraphQL global node id for use with v4 API."]
    pub fn node_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `primary_language` after provisioning.\n"]
    pub fn primary_language(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.primary_language", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private` after provisioning.\n"]
    pub fn private(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.private", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repo_id` after provisioning.\nGitHub ID for the repository."]
    pub fn repo_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `squash_merge_commit_message` after provisioning.\nCan be 'PR_BODY', 'COMMIT_MESSAGES', or 'BLANK' for a default squash merge commit message."]
    pub fn squash_merge_commit_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.squash_merge_commit_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `squash_merge_commit_title` after provisioning.\nCan be 'PR_TITLE' or 'COMMIT_OR_PR_TITLE' for a default squash merge commit title."]
    pub fn squash_merge_commit_title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.squash_merge_commit_title", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssh_clone_url` after provisioning.\nURL that can be provided to 'git clone' to clone the repository via SSH."]
    pub fn ssh_clone_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssh_clone_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `svn_url` after provisioning.\nURL that can be provided to 'svn checkout' to check out the repository via GitHub's Subversion protocol emulation."]
    pub fn svn_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.svn_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `topics` after provisioning.\nThe list of topics of the repository."]
    pub fn topics(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.topics", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `visibility` after provisioning.\nCan be 'public' or 'private'. If your organization is associated with an enterprise account using GitHub Enterprise Cloud or GitHub Enterprise Server 2.20+, visibility can also be 'internal'."]
    pub fn visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vulnerability_alerts` after provisioning.\nSet to 'true' to enable security alerts for vulnerable dependencies. Enabling requires alerts to be enabled on the owner level. (Note for importing: GitHub enables the alerts on public repos but disables them on private repos by default). Note that vulnerability alerts have not been successfully tested on any GitHub Enterprise instance and may be unavailable in those settings."]
    pub fn vulnerability_alerts(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.vulnerability_alerts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `web_commit_signoff_required` after provisioning.\nRequire contributors to sign off on web-based commits. Defaults to 'false'."]
    pub fn web_commit_signoff_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_commit_signoff_required", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pages` after provisioning.\n"]
    pub fn pages(&self) -> ListRef<RepositoryPagesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `security_and_analysis` after provisioning.\n"]
    pub fn security_and_analysis(&self) -> ListRef<RepositorySecurityAndAnalysisElRef> {
        ListRef::new(self.shared().clone(), format!("{}.security_and_analysis", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `template` after provisioning.\n"]
    pub fn template(&self) -> ListRef<RepositoryTemplateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.template", self.extract_ref()))
    }
}

impl Referable for Repository {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Repository { }

impl ToListMappable for Repository {
    type O = ListRef<RepositoryRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Repository_ {
    fn extract_resource_type(&self) -> String {
        "github_repository".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRepository {
    pub tf_id: String,
    #[doc= "The name of the repository."]
    pub name: PrimField<String>,
}

impl BuildRepository {
    pub fn build(self, stack: &mut Stack) -> Repository {
        let out = Repository(Rc::new(Repository_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RepositoryData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                allow_auto_merge: core::default::Default::default(),
                allow_merge_commit: core::default::Default::default(),
                allow_rebase_merge: core::default::Default::default(),
                allow_squash_merge: core::default::Default::default(),
                allow_update_branch: core::default::Default::default(),
                archive_on_destroy: core::default::Default::default(),
                archived: core::default::Default::default(),
                auto_init: core::default::Default::default(),
                default_branch: core::default::Default::default(),
                delete_branch_on_merge: core::default::Default::default(),
                description: core::default::Default::default(),
                gitignore_template: core::default::Default::default(),
                has_discussions: core::default::Default::default(),
                has_downloads: core::default::Default::default(),
                has_issues: core::default::Default::default(),
                has_projects: core::default::Default::default(),
                has_wiki: core::default::Default::default(),
                homepage_url: core::default::Default::default(),
                id: core::default::Default::default(),
                ignore_vulnerability_alerts_during_read: core::default::Default::default(),
                is_template: core::default::Default::default(),
                license_template: core::default::Default::default(),
                merge_commit_message: core::default::Default::default(),
                merge_commit_title: core::default::Default::default(),
                name: self.name,
                private: core::default::Default::default(),
                squash_merge_commit_message: core::default::Default::default(),
                squash_merge_commit_title: core::default::Default::default(),
                topics: core::default::Default::default(),
                visibility: core::default::Default::default(),
                vulnerability_alerts: core::default::Default::default(),
                web_commit_signoff_required: core::default::Default::default(),
                pages: core::default::Default::default(),
                security_and_analysis: core::default::Default::default(),
                template: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RepositoryRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositoryRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RepositoryRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allow_auto_merge` after provisioning.\nSet to 'true' to allow auto-merging pull requests on the repository."]
    pub fn allow_auto_merge(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_auto_merge", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allow_merge_commit` after provisioning.\nSet to 'false' to disable merge commits on the repository."]
    pub fn allow_merge_commit(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_merge_commit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allow_rebase_merge` after provisioning.\nSet to 'false' to disable rebase merges on the repository."]
    pub fn allow_rebase_merge(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_rebase_merge", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allow_squash_merge` after provisioning.\nSet to 'false' to disable squash merges on the repository."]
    pub fn allow_squash_merge(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_squash_merge", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allow_update_branch` after provisioning.\n Set to 'true' to always suggest updating pull request branches."]
    pub fn allow_update_branch(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_update_branch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `archive_on_destroy` after provisioning.\nSet to 'true' to archive the repository instead of deleting on destroy."]
    pub fn archive_on_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.archive_on_destroy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `archived` after provisioning.\nSpecifies if the repository should be archived. Defaults to 'false'. NOTE Currently, the API does not support unarchiving."]
    pub fn archived(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.archived", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_init` after provisioning.\nSet to 'true' to produce an initial commit in the repository."]
    pub fn auto_init(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_init", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_branch` after provisioning.\nCan only be set after initial repository creation, and only if the target branch exists"]
    pub fn default_branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_branch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delete_branch_on_merge` after provisioning.\nAutomatically delete head branch after a pull request is merged. Defaults to 'false'."]
    pub fn delete_branch_on_merge(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_branch_on_merge", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA description of the repository."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `full_name` after provisioning.\nA string of the form 'orgname/reponame'."]
    pub fn full_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.full_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `git_clone_url` after provisioning.\nURL that can be provided to 'git clone' to clone the repository anonymously via the git protocol."]
    pub fn git_clone_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.git_clone_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `gitignore_template` after provisioning.\nUse the name of the template without the extension. For example, 'Haskell'."]
    pub fn gitignore_template(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gitignore_template", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `has_discussions` after provisioning.\nSet to 'true' to enable GitHub Discussions on the repository. Defaults to 'false'."]
    pub fn has_discussions(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.has_discussions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `has_downloads` after provisioning.\nSet to 'true' to enable the (deprecated) downloads features on the repository."]
    pub fn has_downloads(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.has_downloads", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `has_issues` after provisioning.\nSet to 'true' to enable the GitHub Issues features on the repository"]
    pub fn has_issues(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.has_issues", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `has_projects` after provisioning.\nSet to 'true' to enable the GitHub Projects features on the repository. Per the GitHub documentation when in an organization that has disabled repository projects it will default to 'false' and will otherwise default to 'true'. If you specify 'true' when it has been disabled it will return an error."]
    pub fn has_projects(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.has_projects", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `has_wiki` after provisioning.\nSet to 'true' to enable the GitHub Wiki features on the repository."]
    pub fn has_wiki(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.has_wiki", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `homepage_url` after provisioning.\nURL of a page describing the project."]
    pub fn homepage_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.homepage_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `html_url` after provisioning.\nURL to the repository on the web."]
    pub fn html_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.html_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `http_clone_url` after provisioning.\nURL that can be provided to 'git clone' to clone the repository via HTTPS."]
    pub fn http_clone_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_clone_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ignore_vulnerability_alerts_during_read` after provisioning.\nSet to true to not call the vulnerability alerts endpoint so the resource can also be used without admin permissions during read."]
    pub fn ignore_vulnerability_alerts_during_read(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_vulnerability_alerts_during_read", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `is_template` after provisioning.\nSet to 'true' to tell GitHub that this is a template repository."]
    pub fn is_template(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_template", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `license_template` after provisioning.\nUse the name of the template without the extension. For example, 'mit' or 'mpl-2.0'."]
    pub fn license_template(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.license_template", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_commit_message` after provisioning.\nCan be 'PR_BODY', 'PR_TITLE', or 'BLANK' for a default merge commit message."]
    pub fn merge_commit_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_commit_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `merge_commit_title` after provisioning.\nCan be 'PR_TITLE' or 'MERGE_MESSAGE' for a default merge commit title."]
    pub fn merge_commit_title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_commit_title", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the repository."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_id` after provisioning.\nGraphQL global node id for use with v4 API."]
    pub fn node_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `primary_language` after provisioning.\n"]
    pub fn primary_language(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.primary_language", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private` after provisioning.\n"]
    pub fn private(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.private", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repo_id` after provisioning.\nGitHub ID for the repository."]
    pub fn repo_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `squash_merge_commit_message` after provisioning.\nCan be 'PR_BODY', 'COMMIT_MESSAGES', or 'BLANK' for a default squash merge commit message."]
    pub fn squash_merge_commit_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.squash_merge_commit_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `squash_merge_commit_title` after provisioning.\nCan be 'PR_TITLE' or 'COMMIT_OR_PR_TITLE' for a default squash merge commit title."]
    pub fn squash_merge_commit_title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.squash_merge_commit_title", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssh_clone_url` after provisioning.\nURL that can be provided to 'git clone' to clone the repository via SSH."]
    pub fn ssh_clone_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssh_clone_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `svn_url` after provisioning.\nURL that can be provided to 'svn checkout' to check out the repository via GitHub's Subversion protocol emulation."]
    pub fn svn_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.svn_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `topics` after provisioning.\nThe list of topics of the repository."]
    pub fn topics(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.topics", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `visibility` after provisioning.\nCan be 'public' or 'private'. If your organization is associated with an enterprise account using GitHub Enterprise Cloud or GitHub Enterprise Server 2.20+, visibility can also be 'internal'."]
    pub fn visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vulnerability_alerts` after provisioning.\nSet to 'true' to enable security alerts for vulnerable dependencies. Enabling requires alerts to be enabled on the owner level. (Note for importing: GitHub enables the alerts on public repos but disables them on private repos by default). Note that vulnerability alerts have not been successfully tested on any GitHub Enterprise instance and may be unavailable in those settings."]
    pub fn vulnerability_alerts(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.vulnerability_alerts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `web_commit_signoff_required` after provisioning.\nRequire contributors to sign off on web-based commits. Defaults to 'false'."]
    pub fn web_commit_signoff_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_commit_signoff_required", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pages` after provisioning.\n"]
    pub fn pages(&self) -> ListRef<RepositoryPagesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `security_and_analysis` after provisioning.\n"]
    pub fn security_and_analysis(&self) -> ListRef<RepositorySecurityAndAnalysisElRef> {
        ListRef::new(self.shared().clone(), format!("{}.security_and_analysis", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `template` after provisioning.\n"]
    pub fn template(&self) -> ListRef<RepositoryTemplateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.template", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct RepositoryPagesElSourceEl {
    branch: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
}

impl RepositoryPagesElSourceEl {
    #[doc= "Set the field `path`.\nThe repository directory from which the site publishes (Default: '/')"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
}

impl ToListMappable for RepositoryPagesElSourceEl {
    type O = BlockAssignable<RepositoryPagesElSourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRepositoryPagesElSourceEl {
    #[doc= "The repository branch used to publish the site's source files. (i.e. 'main' or 'gh-pages')"]
    pub branch: PrimField<String>,
}

impl BuildRepositoryPagesElSourceEl {
    pub fn build(self) -> RepositoryPagesElSourceEl {
        RepositoryPagesElSourceEl {
            branch: self.branch,
            path: core::default::Default::default(),
        }
    }
}

pub struct RepositoryPagesElSourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositoryPagesElSourceElRef {
    fn new(shared: StackShared, base: String) -> RepositoryPagesElSourceElRef {
        RepositoryPagesElSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RepositoryPagesElSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\nThe repository branch used to publish the site's source files. (i.e. 'main' or 'gh-pages')"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\nThe repository directory from which the site publishes (Default: '/')"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize, Default)]
struct RepositoryPagesElDynamic {
    source: Option<DynamicBlock<RepositoryPagesElSourceEl>>,
}

#[derive(Serialize)]
pub struct RepositoryPagesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    build_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cname: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<Vec<RepositoryPagesElSourceEl>>,
    dynamic: RepositoryPagesElDynamic,
}

impl RepositoryPagesEl {
    #[doc= "Set the field `build_type`.\nThe type the page should be sourced."]
    pub fn set_build_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.build_type = Some(v.into());
        self
    }

    #[doc= "Set the field `cname`.\nThe custom domain for the repository. This can only be set after the repository has been created."]
    pub fn set_cname(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cname = Some(v.into());
        self
    }

    #[doc= "Set the field `source`.\n"]
    pub fn set_source(mut self, v: impl Into<BlockAssignable<RepositoryPagesElSourceEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for RepositoryPagesEl {
    type O = BlockAssignable<RepositoryPagesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRepositoryPagesEl {}

impl BuildRepositoryPagesEl {
    pub fn build(self) -> RepositoryPagesEl {
        RepositoryPagesEl {
            build_type: core::default::Default::default(),
            cname: core::default::Default::default(),
            source: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct RepositoryPagesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositoryPagesElRef {
    fn new(shared: StackShared, base: String) -> RepositoryPagesElRef {
        RepositoryPagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RepositoryPagesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `build_type` after provisioning.\nThe type the page should be sourced."]
    pub fn build_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.build_type", self.base))
    }

    #[doc= "Get a reference to the value of field `cname` after provisioning.\nThe custom domain for the repository. This can only be set after the repository has been created."]
    pub fn cname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cname", self.base))
    }

    #[doc= "Get a reference to the value of field `custom_404` after provisioning.\nWhether the rendered GitHub Pages site has a custom 404 page"]
    pub fn custom_404(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_404", self.base))
    }

    #[doc= "Get a reference to the value of field `html_url` after provisioning.\nURL to the repository on the web."]
    pub fn html_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.html_url", self.base))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nThe GitHub Pages site's build status e.g. building or built."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc= "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<RepositoryPagesElSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct RepositorySecurityAndAnalysisElAdvancedSecurityEl {
    status: PrimField<String>,
}

impl RepositorySecurityAndAnalysisElAdvancedSecurityEl { }

impl ToListMappable for RepositorySecurityAndAnalysisElAdvancedSecurityEl {
    type O = BlockAssignable<RepositorySecurityAndAnalysisElAdvancedSecurityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRepositorySecurityAndAnalysisElAdvancedSecurityEl {
    #[doc= "Set to 'enabled' to enable advanced security features on the repository. Can be 'enabled' or 'disabled'."]
    pub status: PrimField<String>,
}

impl BuildRepositorySecurityAndAnalysisElAdvancedSecurityEl {
    pub fn build(self) -> RepositorySecurityAndAnalysisElAdvancedSecurityEl {
        RepositorySecurityAndAnalysisElAdvancedSecurityEl { status: self.status }
    }
}

pub struct RepositorySecurityAndAnalysisElAdvancedSecurityElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositorySecurityAndAnalysisElAdvancedSecurityElRef {
    fn new(shared: StackShared, base: String) -> RepositorySecurityAndAnalysisElAdvancedSecurityElRef {
        RepositorySecurityAndAnalysisElAdvancedSecurityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RepositorySecurityAndAnalysisElAdvancedSecurityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nSet to 'enabled' to enable advanced security features on the repository. Can be 'enabled' or 'disabled'."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct RepositorySecurityAndAnalysisElSecretScanningEl {
    status: PrimField<String>,
}

impl RepositorySecurityAndAnalysisElSecretScanningEl { }

impl ToListMappable for RepositorySecurityAndAnalysisElSecretScanningEl {
    type O = BlockAssignable<RepositorySecurityAndAnalysisElSecretScanningEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRepositorySecurityAndAnalysisElSecretScanningEl {
    #[doc= "Set to 'enabled' to enable secret scanning on the repository. Can be 'enabled' or 'disabled'. If set to 'enabled', the repository's visibility must be 'public' or 'security_and_analysis[0].advanced_security[0].status' must also be set to 'enabled'."]
    pub status: PrimField<String>,
}

impl BuildRepositorySecurityAndAnalysisElSecretScanningEl {
    pub fn build(self) -> RepositorySecurityAndAnalysisElSecretScanningEl {
        RepositorySecurityAndAnalysisElSecretScanningEl { status: self.status }
    }
}

pub struct RepositorySecurityAndAnalysisElSecretScanningElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositorySecurityAndAnalysisElSecretScanningElRef {
    fn new(shared: StackShared, base: String) -> RepositorySecurityAndAnalysisElSecretScanningElRef {
        RepositorySecurityAndAnalysisElSecretScanningElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RepositorySecurityAndAnalysisElSecretScanningElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nSet to 'enabled' to enable secret scanning on the repository. Can be 'enabled' or 'disabled'. If set to 'enabled', the repository's visibility must be 'public' or 'security_and_analysis[0].advanced_security[0].status' must also be set to 'enabled'."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct RepositorySecurityAndAnalysisElSecretScanningPushProtectionEl {
    status: PrimField<String>,
}

impl RepositorySecurityAndAnalysisElSecretScanningPushProtectionEl { }

impl ToListMappable for RepositorySecurityAndAnalysisElSecretScanningPushProtectionEl {
    type O = BlockAssignable<RepositorySecurityAndAnalysisElSecretScanningPushProtectionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRepositorySecurityAndAnalysisElSecretScanningPushProtectionEl {
    #[doc= "Set to 'enabled' to enable secret scanning push protection on the repository. Can be 'enabled' or 'disabled'. If set to 'enabled', the repository's visibility must be 'public' or 'security_and_analysis[0].advanced_security[0].status' must also be set to 'enabled'."]
    pub status: PrimField<String>,
}

impl BuildRepositorySecurityAndAnalysisElSecretScanningPushProtectionEl {
    pub fn build(self) -> RepositorySecurityAndAnalysisElSecretScanningPushProtectionEl {
        RepositorySecurityAndAnalysisElSecretScanningPushProtectionEl { status: self.status }
    }
}

pub struct RepositorySecurityAndAnalysisElSecretScanningPushProtectionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositorySecurityAndAnalysisElSecretScanningPushProtectionElRef {
    fn new(shared: StackShared, base: String) -> RepositorySecurityAndAnalysisElSecretScanningPushProtectionElRef {
        RepositorySecurityAndAnalysisElSecretScanningPushProtectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RepositorySecurityAndAnalysisElSecretScanningPushProtectionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nSet to 'enabled' to enable secret scanning push protection on the repository. Can be 'enabled' or 'disabled'. If set to 'enabled', the repository's visibility must be 'public' or 'security_and_analysis[0].advanced_security[0].status' must also be set to 'enabled'."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize, Default)]
struct RepositorySecurityAndAnalysisElDynamic {
    advanced_security: Option<DynamicBlock<RepositorySecurityAndAnalysisElAdvancedSecurityEl>>,
    secret_scanning: Option<DynamicBlock<RepositorySecurityAndAnalysisElSecretScanningEl>>,
    secret_scanning_push_protection: Option<
        DynamicBlock<RepositorySecurityAndAnalysisElSecretScanningPushProtectionEl>,
    >,
}

#[derive(Serialize)]
pub struct RepositorySecurityAndAnalysisEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_security: Option<Vec<RepositorySecurityAndAnalysisElAdvancedSecurityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_scanning: Option<Vec<RepositorySecurityAndAnalysisElSecretScanningEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_scanning_push_protection: Option<Vec<RepositorySecurityAndAnalysisElSecretScanningPushProtectionEl>>,
    dynamic: RepositorySecurityAndAnalysisElDynamic,
}

impl RepositorySecurityAndAnalysisEl {
    #[doc= "Set the field `advanced_security`.\n"]
    pub fn set_advanced_security(
        mut self,
        v: impl Into<BlockAssignable<RepositorySecurityAndAnalysisElAdvancedSecurityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.advanced_security = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_security = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `secret_scanning`.\n"]
    pub fn set_secret_scanning(
        mut self,
        v: impl Into<BlockAssignable<RepositorySecurityAndAnalysisElSecretScanningEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.secret_scanning = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.secret_scanning = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `secret_scanning_push_protection`.\n"]
    pub fn set_secret_scanning_push_protection(
        mut self,
        v: impl Into<BlockAssignable<RepositorySecurityAndAnalysisElSecretScanningPushProtectionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.secret_scanning_push_protection = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.secret_scanning_push_protection = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for RepositorySecurityAndAnalysisEl {
    type O = BlockAssignable<RepositorySecurityAndAnalysisEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRepositorySecurityAndAnalysisEl {}

impl BuildRepositorySecurityAndAnalysisEl {
    pub fn build(self) -> RepositorySecurityAndAnalysisEl {
        RepositorySecurityAndAnalysisEl {
            advanced_security: core::default::Default::default(),
            secret_scanning: core::default::Default::default(),
            secret_scanning_push_protection: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct RepositorySecurityAndAnalysisElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositorySecurityAndAnalysisElRef {
    fn new(shared: StackShared, base: String) -> RepositorySecurityAndAnalysisElRef {
        RepositorySecurityAndAnalysisElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RepositorySecurityAndAnalysisElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `advanced_security` after provisioning.\n"]
    pub fn advanced_security(&self) -> ListRef<RepositorySecurityAndAnalysisElAdvancedSecurityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.advanced_security", self.base))
    }

    #[doc= "Get a reference to the value of field `secret_scanning` after provisioning.\n"]
    pub fn secret_scanning(&self) -> ListRef<RepositorySecurityAndAnalysisElSecretScanningElRef> {
        ListRef::new(self.shared().clone(), format!("{}.secret_scanning", self.base))
    }

    #[doc= "Get a reference to the value of field `secret_scanning_push_protection` after provisioning.\n"]
    pub fn secret_scanning_push_protection(
        &self,
    ) -> ListRef<RepositorySecurityAndAnalysisElSecretScanningPushProtectionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.secret_scanning_push_protection", self.base))
    }
}

#[derive(Serialize)]
pub struct RepositoryTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    include_all_branches: Option<PrimField<bool>>,
    owner: PrimField<String>,
    repository: PrimField<String>,
}

impl RepositoryTemplateEl {
    #[doc= "Set the field `include_all_branches`.\nWhether the new repository should include all the branches from the template repository (defaults to 'false', which includes only the default branch from the template)."]
    pub fn set_include_all_branches(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_all_branches = Some(v.into());
        self
    }
}

impl ToListMappable for RepositoryTemplateEl {
    type O = BlockAssignable<RepositoryTemplateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRepositoryTemplateEl {
    #[doc= "The GitHub organization or user the template repository is owned by."]
    pub owner: PrimField<String>,
    #[doc= "The name of the template repository."]
    pub repository: PrimField<String>,
}

impl BuildRepositoryTemplateEl {
    pub fn build(self) -> RepositoryTemplateEl {
        RepositoryTemplateEl {
            include_all_branches: core::default::Default::default(),
            owner: self.owner,
            repository: self.repository,
        }
    }
}

pub struct RepositoryTemplateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositoryTemplateElRef {
    fn new(shared: StackShared, base: String) -> RepositoryTemplateElRef {
        RepositoryTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RepositoryTemplateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `include_all_branches` after provisioning.\nWhether the new repository should include all the branches from the template repository (defaults to 'false', which includes only the default branch from the template)."]
    pub fn include_all_branches(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_all_branches", self.base))
    }

    #[doc= "Get a reference to the value of field `owner` after provisioning.\nThe GitHub organization or user the template repository is owned by."]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.base))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\nThe name of the template repository."]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }
}

#[derive(Serialize, Default)]
struct RepositoryDynamic {
    pages: Option<DynamicBlock<RepositoryPagesEl>>,
    security_and_analysis: Option<DynamicBlock<RepositorySecurityAndAnalysisEl>>,
    template: Option<DynamicBlock<RepositoryTemplateEl>>,
}
