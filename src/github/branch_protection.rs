use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct BranchProtectionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allows_deletions: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allows_force_pushes: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    blocks_creations: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_admins: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_push_bypassers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lock_branch: Option<PrimField<bool>>,
    pattern: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    push_restrictions: Option<SetField<PrimField<String>>>,
    repository_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_conversation_resolution: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_signed_commits: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required_linear_history: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required_pull_request_reviews: Option<Vec<BranchProtectionRequiredPullRequestReviewsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required_status_checks: Option<Vec<BranchProtectionRequiredStatusChecksEl>>,
    dynamic: BranchProtectionDynamic,
}

struct BranchProtection_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BranchProtectionData>,
}

#[derive(Clone)]
pub struct BranchProtection(Rc<BranchProtection_>);

impl BranchProtection {
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

    #[doc= "Set the field `allows_deletions`.\nSetting this to 'true' to allow the branch to be deleted."]
    pub fn set_allows_deletions(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allows_deletions = Some(v.into());
        self
    }

    #[doc= "Set the field `allows_force_pushes`.\nSetting this to 'true' to allow force pushes on the branch."]
    pub fn set_allows_force_pushes(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allows_force_pushes = Some(v.into());
        self
    }

    #[doc= "Set the field `blocks_creations`.\nSetting this to 'true' to block creating the branch."]
    pub fn set_blocks_creations(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().blocks_creations = Some(v.into());
        self
    }

    #[doc= "Set the field `enforce_admins`.\nSetting this to 'true' enforces status checks for repository administrators."]
    pub fn set_enforce_admins(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enforce_admins = Some(v.into());
        self
    }

    #[doc= "Set the field `force_push_bypassers`.\nThe list of actor Names/IDs that are allowed to bypass force push restrictions. Actor names must either begin with a '/' for users or the organization name followed by a '/' for teams."]
    pub fn set_force_push_bypassers(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().force_push_bypassers = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `lock_branch`.\nSetting this to 'true' will make the branch read-only and preventing any pushes to it."]
    pub fn set_lock_branch(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().lock_branch = Some(v.into());
        self
    }

    #[doc= "Set the field `push_restrictions`.\nThe list of actor Names/IDs that may push to the branch. Actor names must either begin with a '/' for users or the organization name followed by a '/' for teams."]
    pub fn set_push_restrictions(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().push_restrictions = Some(v.into());
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

    #[doc= "Set the field `required_linear_history`.\nSetting this to 'true' enforces a linear commit Git history, which prevents anyone from pushing merge commits to a branch."]
    pub fn set_required_linear_history(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().required_linear_history = Some(v.into());
        self
    }

    #[doc= "Set the field `required_pull_request_reviews`.\n"]
    pub fn set_required_pull_request_reviews(
        self,
        v: impl Into<BlockAssignable<BranchProtectionRequiredPullRequestReviewsEl>>,
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
        v: impl Into<BlockAssignable<BranchProtectionRequiredStatusChecksEl>>,
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

    #[doc= "Get a reference to the value of field `allows_deletions` after provisioning.\nSetting this to 'true' to allow the branch to be deleted."]
    pub fn allows_deletions(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allows_deletions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allows_force_pushes` after provisioning.\nSetting this to 'true' to allow force pushes on the branch."]
    pub fn allows_force_pushes(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allows_force_pushes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `blocks_creations` after provisioning.\nSetting this to 'true' to block creating the branch."]
    pub fn blocks_creations(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.blocks_creations", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enforce_admins` after provisioning.\nSetting this to 'true' enforces status checks for repository administrators."]
    pub fn enforce_admins(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enforce_admins", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `force_push_bypassers` after provisioning.\nThe list of actor Names/IDs that are allowed to bypass force push restrictions. Actor names must either begin with a '/' for users or the organization name followed by a '/' for teams."]
    pub fn force_push_bypassers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.force_push_bypassers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `lock_branch` after provisioning.\nSetting this to 'true' will make the branch read-only and preventing any pushes to it."]
    pub fn lock_branch(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.lock_branch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pattern` after provisioning.\nIdentifies the protection rule pattern."]
    pub fn pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_restrictions` after provisioning.\nThe list of actor Names/IDs that may push to the branch. Actor names must either begin with a '/' for users or the organization name followed by a '/' for teams."]
    pub fn push_restrictions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.push_restrictions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository_id` after provisioning.\nThe name or node ID of the repository associated with this branch protection rule."]
    pub fn repository_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `require_conversation_resolution` after provisioning.\nSetting this to 'true' requires all conversations on code must be resolved before a pull request can be merged."]
    pub fn require_conversation_resolution(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_conversation_resolution", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `require_signed_commits` after provisioning.\nSetting this to 'true' requires all commits to be signed with GPG."]
    pub fn require_signed_commits(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_signed_commits", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `required_linear_history` after provisioning.\nSetting this to 'true' enforces a linear commit Git history, which prevents anyone from pushing merge commits to a branch."]
    pub fn required_linear_history(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.required_linear_history", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `required_pull_request_reviews` after provisioning.\n"]
    pub fn required_pull_request_reviews(&self) -> ListRef<BranchProtectionRequiredPullRequestReviewsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.required_pull_request_reviews", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `required_status_checks` after provisioning.\n"]
    pub fn required_status_checks(&self) -> ListRef<BranchProtectionRequiredStatusChecksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.required_status_checks", self.extract_ref()))
    }
}

impl Referable for BranchProtection {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for BranchProtection { }

impl ToListMappable for BranchProtection {
    type O = ListRef<BranchProtectionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for BranchProtection_ {
    fn extract_resource_type(&self) -> String {
        "github_branch_protection".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBranchProtection {
    pub tf_id: String,
    #[doc= "Identifies the protection rule pattern."]
    pub pattern: PrimField<String>,
    #[doc= "The name or node ID of the repository associated with this branch protection rule."]
    pub repository_id: PrimField<String>,
}

impl BuildBranchProtection {
    pub fn build(self, stack: &mut Stack) -> BranchProtection {
        let out = BranchProtection(Rc::new(BranchProtection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BranchProtectionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                allows_deletions: core::default::Default::default(),
                allows_force_pushes: core::default::Default::default(),
                blocks_creations: core::default::Default::default(),
                enforce_admins: core::default::Default::default(),
                force_push_bypassers: core::default::Default::default(),
                id: core::default::Default::default(),
                lock_branch: core::default::Default::default(),
                pattern: self.pattern,
                push_restrictions: core::default::Default::default(),
                repository_id: self.repository_id,
                require_conversation_resolution: core::default::Default::default(),
                require_signed_commits: core::default::Default::default(),
                required_linear_history: core::default::Default::default(),
                required_pull_request_reviews: core::default::Default::default(),
                required_status_checks: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BranchProtectionRef {
    shared: StackShared,
    base: String,
}

impl Ref for BranchProtectionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BranchProtectionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allows_deletions` after provisioning.\nSetting this to 'true' to allow the branch to be deleted."]
    pub fn allows_deletions(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allows_deletions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allows_force_pushes` after provisioning.\nSetting this to 'true' to allow force pushes on the branch."]
    pub fn allows_force_pushes(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allows_force_pushes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `blocks_creations` after provisioning.\nSetting this to 'true' to block creating the branch."]
    pub fn blocks_creations(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.blocks_creations", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enforce_admins` after provisioning.\nSetting this to 'true' enforces status checks for repository administrators."]
    pub fn enforce_admins(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enforce_admins", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `force_push_bypassers` after provisioning.\nThe list of actor Names/IDs that are allowed to bypass force push restrictions. Actor names must either begin with a '/' for users or the organization name followed by a '/' for teams."]
    pub fn force_push_bypassers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.force_push_bypassers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `lock_branch` after provisioning.\nSetting this to 'true' will make the branch read-only and preventing any pushes to it."]
    pub fn lock_branch(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.lock_branch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pattern` after provisioning.\nIdentifies the protection rule pattern."]
    pub fn pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_restrictions` after provisioning.\nThe list of actor Names/IDs that may push to the branch. Actor names must either begin with a '/' for users or the organization name followed by a '/' for teams."]
    pub fn push_restrictions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.push_restrictions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository_id` after provisioning.\nThe name or node ID of the repository associated with this branch protection rule."]
    pub fn repository_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `require_conversation_resolution` after provisioning.\nSetting this to 'true' requires all conversations on code must be resolved before a pull request can be merged."]
    pub fn require_conversation_resolution(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_conversation_resolution", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `require_signed_commits` after provisioning.\nSetting this to 'true' requires all commits to be signed with GPG."]
    pub fn require_signed_commits(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_signed_commits", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `required_linear_history` after provisioning.\nSetting this to 'true' enforces a linear commit Git history, which prevents anyone from pushing merge commits to a branch."]
    pub fn required_linear_history(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.required_linear_history", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `required_pull_request_reviews` after provisioning.\n"]
    pub fn required_pull_request_reviews(&self) -> ListRef<BranchProtectionRequiredPullRequestReviewsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.required_pull_request_reviews", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `required_status_checks` after provisioning.\n"]
    pub fn required_status_checks(&self) -> ListRef<BranchProtectionRequiredStatusChecksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.required_status_checks", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BranchProtectionRequiredPullRequestReviewsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dismiss_stale_reviews: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dismissal_restrictions: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pull_request_bypassers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_code_owner_reviews: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_last_push_approval: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required_approving_review_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restrict_dismissals: Option<PrimField<bool>>,
}

impl BranchProtectionRequiredPullRequestReviewsEl {
    #[doc= "Set the field `dismiss_stale_reviews`.\nDismiss approved reviews automatically when a new commit is pushed."]
    pub fn set_dismiss_stale_reviews(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.dismiss_stale_reviews = Some(v.into());
        self
    }

    #[doc= "Set the field `dismissal_restrictions`.\nThe list of actor Names/IDs with dismissal access. If not empty, 'restrict_dismissals' is ignored. Actor names must either begin with a '/' for users or the organization name followed by a '/' for teams."]
    pub fn set_dismissal_restrictions(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.dismissal_restrictions = Some(v.into());
        self
    }

    #[doc= "Set the field `pull_request_bypassers`.\nThe list of actor Names/IDs that are allowed to bypass pull request requirements. Actor names must either begin with a '/' for users or the organization name followed by a '/' for teams."]
    pub fn set_pull_request_bypassers(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.pull_request_bypassers = Some(v.into());
        self
    }

    #[doc= "Set the field `require_code_owner_reviews`.\nRequire an approved review in pull requests including files with a designated code owner."]
    pub fn set_require_code_owner_reviews(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_code_owner_reviews = Some(v.into());
        self
    }

    #[doc= "Set the field `require_last_push_approval`.\nRequire that The most recent push must be approved by someone other than the last pusher."]
    pub fn set_require_last_push_approval(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_last_push_approval = Some(v.into());
        self
    }

    #[doc= "Set the field `required_approving_review_count`.\nRequire 'x' number of approvals to satisfy branch protection requirements. If this is specified it must be a number between 0-6."]
    pub fn set_required_approving_review_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.required_approving_review_count = Some(v.into());
        self
    }

    #[doc= "Set the field `restrict_dismissals`.\nRestrict pull request review dismissals."]
    pub fn set_restrict_dismissals(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.restrict_dismissals = Some(v.into());
        self
    }
}

impl ToListMappable for BranchProtectionRequiredPullRequestReviewsEl {
    type O = BlockAssignable<BranchProtectionRequiredPullRequestReviewsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBranchProtectionRequiredPullRequestReviewsEl {}

impl BuildBranchProtectionRequiredPullRequestReviewsEl {
    pub fn build(self) -> BranchProtectionRequiredPullRequestReviewsEl {
        BranchProtectionRequiredPullRequestReviewsEl {
            dismiss_stale_reviews: core::default::Default::default(),
            dismissal_restrictions: core::default::Default::default(),
            pull_request_bypassers: core::default::Default::default(),
            require_code_owner_reviews: core::default::Default::default(),
            require_last_push_approval: core::default::Default::default(),
            required_approving_review_count: core::default::Default::default(),
            restrict_dismissals: core::default::Default::default(),
        }
    }
}

pub struct BranchProtectionRequiredPullRequestReviewsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BranchProtectionRequiredPullRequestReviewsElRef {
    fn new(shared: StackShared, base: String) -> BranchProtectionRequiredPullRequestReviewsElRef {
        BranchProtectionRequiredPullRequestReviewsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BranchProtectionRequiredPullRequestReviewsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `dismiss_stale_reviews` after provisioning.\nDismiss approved reviews automatically when a new commit is pushed."]
    pub fn dismiss_stale_reviews(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.dismiss_stale_reviews", self.base))
    }

    #[doc= "Get a reference to the value of field `dismissal_restrictions` after provisioning.\nThe list of actor Names/IDs with dismissal access. If not empty, 'restrict_dismissals' is ignored. Actor names must either begin with a '/' for users or the organization name followed by a '/' for teams."]
    pub fn dismissal_restrictions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.dismissal_restrictions", self.base))
    }

    #[doc= "Get a reference to the value of field `pull_request_bypassers` after provisioning.\nThe list of actor Names/IDs that are allowed to bypass pull request requirements. Actor names must either begin with a '/' for users or the organization name followed by a '/' for teams."]
    pub fn pull_request_bypassers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.pull_request_bypassers", self.base))
    }

    #[doc= "Get a reference to the value of field `require_code_owner_reviews` after provisioning.\nRequire an approved review in pull requests including files with a designated code owner."]
    pub fn require_code_owner_reviews(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_code_owner_reviews", self.base))
    }

    #[doc= "Get a reference to the value of field `require_last_push_approval` after provisioning.\nRequire that The most recent push must be approved by someone other than the last pusher."]
    pub fn require_last_push_approval(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_last_push_approval", self.base))
    }

    #[doc= "Get a reference to the value of field `required_approving_review_count` after provisioning.\nRequire 'x' number of approvals to satisfy branch protection requirements. If this is specified it must be a number between 0-6."]
    pub fn required_approving_review_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.required_approving_review_count", self.base))
    }

    #[doc= "Get a reference to the value of field `restrict_dismissals` after provisioning.\nRestrict pull request review dismissals."]
    pub fn restrict_dismissals(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.restrict_dismissals", self.base))
    }
}

#[derive(Serialize)]
pub struct BranchProtectionRequiredStatusChecksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    contexts: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strict: Option<PrimField<bool>>,
}

impl BranchProtectionRequiredStatusChecksEl {
    #[doc= "Set the field `contexts`.\nThe list of status checks to require in order to merge into this branch. No status checks are required by default."]
    pub fn set_contexts(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.contexts = Some(v.into());
        self
    }

    #[doc= "Set the field `strict`.\nRequire branches to be up to date before merging."]
    pub fn set_strict(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.strict = Some(v.into());
        self
    }
}

impl ToListMappable for BranchProtectionRequiredStatusChecksEl {
    type O = BlockAssignable<BranchProtectionRequiredStatusChecksEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBranchProtectionRequiredStatusChecksEl {}

impl BuildBranchProtectionRequiredStatusChecksEl {
    pub fn build(self) -> BranchProtectionRequiredStatusChecksEl {
        BranchProtectionRequiredStatusChecksEl {
            contexts: core::default::Default::default(),
            strict: core::default::Default::default(),
        }
    }
}

pub struct BranchProtectionRequiredStatusChecksElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BranchProtectionRequiredStatusChecksElRef {
    fn new(shared: StackShared, base: String) -> BranchProtectionRequiredStatusChecksElRef {
        BranchProtectionRequiredStatusChecksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BranchProtectionRequiredStatusChecksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `contexts` after provisioning.\nThe list of status checks to require in order to merge into this branch. No status checks are required by default."]
    pub fn contexts(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.contexts", self.base))
    }

    #[doc= "Get a reference to the value of field `strict` after provisioning.\nRequire branches to be up to date before merging."]
    pub fn strict(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.strict", self.base))
    }
}

#[derive(Serialize, Default)]
struct BranchProtectionDynamic {
    required_pull_request_reviews: Option<DynamicBlock<BranchProtectionRequiredPullRequestReviewsEl>>,
    required_status_checks: Option<DynamicBlock<BranchProtectionRequiredStatusChecksEl>>,
}
