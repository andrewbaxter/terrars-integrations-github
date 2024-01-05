use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct BranchProtectionV3Data {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    branch: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_admins: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    repository: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_conversation_resolution: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_signed_commits: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required_pull_request_reviews: Option<Vec<BranchProtectionV3RequiredPullRequestReviewsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required_status_checks: Option<Vec<BranchProtectionV3RequiredStatusChecksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restrictions: Option<Vec<BranchProtectionV3RestrictionsEl>>,
    dynamic: BranchProtectionV3Dynamic,
}

struct BranchProtectionV3_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BranchProtectionV3Data>,
}

#[derive(Clone)]
pub struct BranchProtectionV3(Rc<BranchProtectionV3_>);

impl BranchProtectionV3 {
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

    #[doc= "Set the field `enforce_admins`.\nSetting this to 'true' enforces status checks for repository administrators."]
    pub fn set_enforce_admins(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enforce_admins = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `require_conversation_resolution`.\nSetting this to 'true' requires all conversations on code must be resolved before a pull request can be merged."]
    pub fn set_require_conversation_resolution(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().require_conversation_resolution = Some(v.into());
        self
    }

    #[doc= "Set the field `require_signed_commits`.\nSetting this to 'true' requires all commits to be signed with GPG."]
    pub fn set_require_signed_commits(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().require_signed_commits = Some(v.into());
        self
    }

    #[doc= "Set the field `required_pull_request_reviews`.\n"]
    pub fn set_required_pull_request_reviews(
        self,
        v: impl Into<BlockAssignable<BranchProtectionV3RequiredPullRequestReviewsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().required_pull_request_reviews = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.required_pull_request_reviews = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `required_status_checks`.\n"]
    pub fn set_required_status_checks(
        self,
        v: impl Into<BlockAssignable<BranchProtectionV3RequiredStatusChecksEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().required_status_checks = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.required_status_checks = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `restrictions`.\n"]
    pub fn set_restrictions(self, v: impl Into<BlockAssignable<BranchProtectionV3RestrictionsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().restrictions = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.restrictions = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\nThe Git branch to protect."]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enforce_admins` after provisioning.\nSetting this to 'true' enforces status checks for repository administrators."]
    pub fn enforce_admins(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enforce_admins", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\nThe GitHub repository name."]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `require_conversation_resolution` after provisioning.\nSetting this to 'true' requires all conversations on code must be resolved before a pull request can be merged."]
    pub fn require_conversation_resolution(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_conversation_resolution", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `require_signed_commits` after provisioning.\nSetting this to 'true' requires all commits to be signed with GPG."]
    pub fn require_signed_commits(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_signed_commits", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `required_pull_request_reviews` after provisioning.\n"]
    pub fn required_pull_request_reviews(&self) -> ListRef<BranchProtectionV3RequiredPullRequestReviewsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.required_pull_request_reviews", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `required_status_checks` after provisioning.\n"]
    pub fn required_status_checks(&self) -> ListRef<BranchProtectionV3RequiredStatusChecksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.required_status_checks", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `restrictions` after provisioning.\n"]
    pub fn restrictions(&self) -> ListRef<BranchProtectionV3RestrictionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.restrictions", self.extract_ref()))
    }
}

impl Referable for BranchProtectionV3 {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for BranchProtectionV3 { }

impl ToListMappable for BranchProtectionV3 {
    type O = ListRef<BranchProtectionV3Ref>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for BranchProtectionV3_ {
    fn extract_resource_type(&self) -> String {
        "github_branch_protection_v3".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBranchProtectionV3 {
    pub tf_id: String,
    #[doc= "The Git branch to protect."]
    pub branch: PrimField<String>,
    #[doc= "The GitHub repository name."]
    pub repository: PrimField<String>,
}

impl BuildBranchProtectionV3 {
    pub fn build(self, stack: &mut Stack) -> BranchProtectionV3 {
        let out = BranchProtectionV3(Rc::new(BranchProtectionV3_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BranchProtectionV3Data {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                branch: self.branch,
                enforce_admins: core::default::Default::default(),
                id: core::default::Default::default(),
                repository: self.repository,
                require_conversation_resolution: core::default::Default::default(),
                require_signed_commits: core::default::Default::default(),
                required_pull_request_reviews: core::default::Default::default(),
                required_status_checks: core::default::Default::default(),
                restrictions: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BranchProtectionV3Ref {
    shared: StackShared,
    base: String,
}

impl Ref for BranchProtectionV3Ref {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BranchProtectionV3Ref {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\nThe Git branch to protect."]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enforce_admins` after provisioning.\nSetting this to 'true' enforces status checks for repository administrators."]
    pub fn enforce_admins(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enforce_admins", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\nThe GitHub repository name."]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `require_conversation_resolution` after provisioning.\nSetting this to 'true' requires all conversations on code must be resolved before a pull request can be merged."]
    pub fn require_conversation_resolution(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_conversation_resolution", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `require_signed_commits` after provisioning.\nSetting this to 'true' requires all commits to be signed with GPG."]
    pub fn require_signed_commits(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_signed_commits", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `required_pull_request_reviews` after provisioning.\n"]
    pub fn required_pull_request_reviews(&self) -> ListRef<BranchProtectionV3RequiredPullRequestReviewsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.required_pull_request_reviews", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `required_status_checks` after provisioning.\n"]
    pub fn required_status_checks(&self) -> ListRef<BranchProtectionV3RequiredStatusChecksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.required_status_checks", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `restrictions` after provisioning.\n"]
    pub fn restrictions(&self) -> ListRef<BranchProtectionV3RestrictionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.restrictions", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BranchProtectionV3RequiredPullRequestReviewsElBypassPullRequestAllowancesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    apps: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    teams: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    users: Option<SetField<PrimField<String>>>,
}

impl BranchProtectionV3RequiredPullRequestReviewsElBypassPullRequestAllowancesEl {
    #[doc= "Set the field `apps`.\n"]
    pub fn set_apps(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.apps = Some(v.into());
        self
    }

    #[doc= "Set the field `teams`.\n"]
    pub fn set_teams(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.teams = Some(v.into());
        self
    }

    #[doc= "Set the field `users`.\n"]
    pub fn set_users(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.users = Some(v.into());
        self
    }
}

impl ToListMappable for BranchProtectionV3RequiredPullRequestReviewsElBypassPullRequestAllowancesEl {
    type O = BlockAssignable<BranchProtectionV3RequiredPullRequestReviewsElBypassPullRequestAllowancesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBranchProtectionV3RequiredPullRequestReviewsElBypassPullRequestAllowancesEl {}

impl BuildBranchProtectionV3RequiredPullRequestReviewsElBypassPullRequestAllowancesEl {
    pub fn build(self) -> BranchProtectionV3RequiredPullRequestReviewsElBypassPullRequestAllowancesEl {
        BranchProtectionV3RequiredPullRequestReviewsElBypassPullRequestAllowancesEl {
            apps: core::default::Default::default(),
            teams: core::default::Default::default(),
            users: core::default::Default::default(),
        }
    }
}

pub struct BranchProtectionV3RequiredPullRequestReviewsElBypassPullRequestAllowancesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BranchProtectionV3RequiredPullRequestReviewsElBypassPullRequestAllowancesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BranchProtectionV3RequiredPullRequestReviewsElBypassPullRequestAllowancesElRef {
        BranchProtectionV3RequiredPullRequestReviewsElBypassPullRequestAllowancesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BranchProtectionV3RequiredPullRequestReviewsElBypassPullRequestAllowancesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `apps` after provisioning.\n"]
    pub fn apps(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.apps", self.base))
    }

    #[doc= "Get a reference to the value of field `teams` after provisioning.\n"]
    pub fn teams(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.teams", self.base))
    }

    #[doc= "Get a reference to the value of field `users` after provisioning.\n"]
    pub fn users(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.users", self.base))
    }
}

#[derive(Serialize, Default)]
struct BranchProtectionV3RequiredPullRequestReviewsElDynamic {
    bypass_pull_request_allowances: Option<
        DynamicBlock<BranchProtectionV3RequiredPullRequestReviewsElBypassPullRequestAllowancesEl>,
    >,
}

#[derive(Serialize)]
pub struct BranchProtectionV3RequiredPullRequestReviewsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dismiss_stale_reviews: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dismissal_apps: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dismissal_teams: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dismissal_users: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_admins: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_code_owner_reviews: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required_approving_review_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bypass_pull_request_allowances: Option<
        Vec<BranchProtectionV3RequiredPullRequestReviewsElBypassPullRequestAllowancesEl>,
    >,
    dynamic: BranchProtectionV3RequiredPullRequestReviewsElDynamic,
}

impl BranchProtectionV3RequiredPullRequestReviewsEl {
    #[doc= "Set the field `dismiss_stale_reviews`.\nDismiss approved reviews automatically when a new commit is pushed."]
    pub fn set_dismiss_stale_reviews(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.dismiss_stale_reviews = Some(v.into());
        self
    }

    #[doc= "Set the field `dismissal_apps`.\nThe list of apps slugs with dismissal access. Always use slug of the app, not its name. Each app already has to have access to the repository."]
    pub fn set_dismissal_apps(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.dismissal_apps = Some(v.into());
        self
    }

    #[doc= "Set the field `dismissal_teams`.\nThe list of team slugs with dismissal access. Always use slug of the team, not its name. Each team already has to have access to the repository."]
    pub fn set_dismissal_teams(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.dismissal_teams = Some(v.into());
        self
    }

    #[doc= "Set the field `dismissal_users`.\nThe list of user logins with dismissal access."]
    pub fn set_dismissal_users(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.dismissal_users = Some(v.into());
        self
    }

    #[doc= "Set the field `include_admins`.\n"]
    pub fn set_include_admins(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_admins = Some(v.into());
        self
    }

    #[doc= "Set the field `require_code_owner_reviews`.\nRequire an approved review in pull requests including files with a designated code owner."]
    pub fn set_require_code_owner_reviews(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_code_owner_reviews = Some(v.into());
        self
    }

    #[doc= "Set the field `required_approving_review_count`.\nRequire 'x' number of approvals to satisfy branch protection requirements. If this is specified it must be a number between 0-6."]
    pub fn set_required_approving_review_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.required_approving_review_count = Some(v.into());
        self
    }

    #[doc= "Set the field `bypass_pull_request_allowances`.\n"]
    pub fn set_bypass_pull_request_allowances(
        mut self,
        v: impl Into<BlockAssignable<BranchProtectionV3RequiredPullRequestReviewsElBypassPullRequestAllowancesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.bypass_pull_request_allowances = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.bypass_pull_request_allowances = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BranchProtectionV3RequiredPullRequestReviewsEl {
    type O = BlockAssignable<BranchProtectionV3RequiredPullRequestReviewsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBranchProtectionV3RequiredPullRequestReviewsEl {}

impl BuildBranchProtectionV3RequiredPullRequestReviewsEl {
    pub fn build(self) -> BranchProtectionV3RequiredPullRequestReviewsEl {
        BranchProtectionV3RequiredPullRequestReviewsEl {
            dismiss_stale_reviews: core::default::Default::default(),
            dismissal_apps: core::default::Default::default(),
            dismissal_teams: core::default::Default::default(),
            dismissal_users: core::default::Default::default(),
            include_admins: core::default::Default::default(),
            require_code_owner_reviews: core::default::Default::default(),
            required_approving_review_count: core::default::Default::default(),
            bypass_pull_request_allowances: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BranchProtectionV3RequiredPullRequestReviewsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BranchProtectionV3RequiredPullRequestReviewsElRef {
    fn new(shared: StackShared, base: String) -> BranchProtectionV3RequiredPullRequestReviewsElRef {
        BranchProtectionV3RequiredPullRequestReviewsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BranchProtectionV3RequiredPullRequestReviewsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `dismiss_stale_reviews` after provisioning.\nDismiss approved reviews automatically when a new commit is pushed."]
    pub fn dismiss_stale_reviews(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.dismiss_stale_reviews", self.base))
    }

    #[doc= "Get a reference to the value of field `dismissal_apps` after provisioning.\nThe list of apps slugs with dismissal access. Always use slug of the app, not its name. Each app already has to have access to the repository."]
    pub fn dismissal_apps(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.dismissal_apps", self.base))
    }

    #[doc= "Get a reference to the value of field `dismissal_teams` after provisioning.\nThe list of team slugs with dismissal access. Always use slug of the team, not its name. Each team already has to have access to the repository."]
    pub fn dismissal_teams(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.dismissal_teams", self.base))
    }

    #[doc= "Get a reference to the value of field `dismissal_users` after provisioning.\nThe list of user logins with dismissal access."]
    pub fn dismissal_users(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.dismissal_users", self.base))
    }

    #[doc= "Get a reference to the value of field `include_admins` after provisioning.\n"]
    pub fn include_admins(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_admins", self.base))
    }

    #[doc= "Get a reference to the value of field `require_code_owner_reviews` after provisioning.\nRequire an approved review in pull requests including files with a designated code owner."]
    pub fn require_code_owner_reviews(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_code_owner_reviews", self.base))
    }

    #[doc= "Get a reference to the value of field `required_approving_review_count` after provisioning.\nRequire 'x' number of approvals to satisfy branch protection requirements. If this is specified it must be a number between 0-6."]
    pub fn required_approving_review_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.required_approving_review_count", self.base))
    }

    #[doc= "Get a reference to the value of field `bypass_pull_request_allowances` after provisioning.\n"]
    pub fn bypass_pull_request_allowances(
        &self,
    ) -> ListRef<BranchProtectionV3RequiredPullRequestReviewsElBypassPullRequestAllowancesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bypass_pull_request_allowances", self.base))
    }
}

#[derive(Serialize)]
pub struct BranchProtectionV3RequiredStatusChecksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    checks: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contexts: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_admins: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strict: Option<PrimField<bool>>,
}

impl BranchProtectionV3RequiredStatusChecksEl {
    #[doc= "Set the field `checks`.\nThe list of status checks to require in order to merge into this branch. No status checks are required by default. Checks should be strings containing the 'context' and 'app_id' like so 'context:app_id'"]
    pub fn set_checks(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.checks = Some(v.into());
        self
    }

    #[doc= "Set the field `contexts`.\n"]
    pub fn set_contexts(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.contexts = Some(v.into());
        self
    }

    #[doc= "Set the field `include_admins`.\n"]
    pub fn set_include_admins(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_admins = Some(v.into());
        self
    }

    #[doc= "Set the field `strict`.\nRequire branches to be up to date before merging."]
    pub fn set_strict(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.strict = Some(v.into());
        self
    }
}

impl ToListMappable for BranchProtectionV3RequiredStatusChecksEl {
    type O = BlockAssignable<BranchProtectionV3RequiredStatusChecksEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBranchProtectionV3RequiredStatusChecksEl {}

impl BuildBranchProtectionV3RequiredStatusChecksEl {
    pub fn build(self) -> BranchProtectionV3RequiredStatusChecksEl {
        BranchProtectionV3RequiredStatusChecksEl {
            checks: core::default::Default::default(),
            contexts: core::default::Default::default(),
            include_admins: core::default::Default::default(),
            strict: core::default::Default::default(),
        }
    }
}

pub struct BranchProtectionV3RequiredStatusChecksElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BranchProtectionV3RequiredStatusChecksElRef {
    fn new(shared: StackShared, base: String) -> BranchProtectionV3RequiredStatusChecksElRef {
        BranchProtectionV3RequiredStatusChecksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BranchProtectionV3RequiredStatusChecksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `checks` after provisioning.\nThe list of status checks to require in order to merge into this branch. No status checks are required by default. Checks should be strings containing the 'context' and 'app_id' like so 'context:app_id'"]
    pub fn checks(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.checks", self.base))
    }

    #[doc= "Get a reference to the value of field `contexts` after provisioning.\n"]
    pub fn contexts(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.contexts", self.base))
    }

    #[doc= "Get a reference to the value of field `include_admins` after provisioning.\n"]
    pub fn include_admins(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_admins", self.base))
    }

    #[doc= "Get a reference to the value of field `strict` after provisioning.\nRequire branches to be up to date before merging."]
    pub fn strict(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.strict", self.base))
    }
}

#[derive(Serialize)]
pub struct BranchProtectionV3RestrictionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    apps: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    teams: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    users: Option<SetField<PrimField<String>>>,
}

impl BranchProtectionV3RestrictionsEl {
    #[doc= "Set the field `apps`.\nThe list of app slugs with push access."]
    pub fn set_apps(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.apps = Some(v.into());
        self
    }

    #[doc= "Set the field `teams`.\nThe list of team slugs with push access. Always use slug of the team, not its name. Each team already has to have access to the repository."]
    pub fn set_teams(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.teams = Some(v.into());
        self
    }

    #[doc= "Set the field `users`.\nThe list of user logins with push access."]
    pub fn set_users(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.users = Some(v.into());
        self
    }
}

impl ToListMappable for BranchProtectionV3RestrictionsEl {
    type O = BlockAssignable<BranchProtectionV3RestrictionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBranchProtectionV3RestrictionsEl {}

impl BuildBranchProtectionV3RestrictionsEl {
    pub fn build(self) -> BranchProtectionV3RestrictionsEl {
        BranchProtectionV3RestrictionsEl {
            apps: core::default::Default::default(),
            teams: core::default::Default::default(),
            users: core::default::Default::default(),
        }
    }
}

pub struct BranchProtectionV3RestrictionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BranchProtectionV3RestrictionsElRef {
    fn new(shared: StackShared, base: String) -> BranchProtectionV3RestrictionsElRef {
        BranchProtectionV3RestrictionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BranchProtectionV3RestrictionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `apps` after provisioning.\nThe list of app slugs with push access."]
    pub fn apps(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.apps", self.base))
    }

    #[doc= "Get a reference to the value of field `teams` after provisioning.\nThe list of team slugs with push access. Always use slug of the team, not its name. Each team already has to have access to the repository."]
    pub fn teams(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.teams", self.base))
    }

    #[doc= "Get a reference to the value of field `users` after provisioning.\nThe list of user logins with push access."]
    pub fn users(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.users", self.base))
    }
}

#[derive(Serialize, Default)]
struct BranchProtectionV3Dynamic {
    required_pull_request_reviews: Option<DynamicBlock<BranchProtectionV3RequiredPullRequestReviewsEl>>,
    required_status_checks: Option<DynamicBlock<BranchProtectionV3RequiredStatusChecksEl>>,
    restrictions: Option<DynamicBlock<BranchProtectionV3RestrictionsEl>>,
}
