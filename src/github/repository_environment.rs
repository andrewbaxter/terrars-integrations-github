use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct RepositoryEnvironmentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_admins_bypass: Option<PrimField<bool>>,
    environment: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    repository: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_timer: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_branch_policy: Option<Vec<RepositoryEnvironmentDeploymentBranchPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reviewers: Option<Vec<RepositoryEnvironmentReviewersEl>>,
    dynamic: RepositoryEnvironmentDynamic,
}

struct RepositoryEnvironment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RepositoryEnvironmentData>,
}

#[derive(Clone)]
pub struct RepositoryEnvironment(Rc<RepositoryEnvironment_>);

impl RepositoryEnvironment {
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

    #[doc= "Set the field `can_admins_bypass`.\nCan Admins bypass deployment protections"]
    pub fn set_can_admins_bypass(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().can_admins_bypass = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `wait_timer`.\nAmount of time to delay a job after the job is initially triggered."]
    pub fn set_wait_timer(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().wait_timer = Some(v.into());
        self
    }

    #[doc= "Set the field `deployment_branch_policy`.\n"]
    pub fn set_deployment_branch_policy(
        self,
        v: impl Into<BlockAssignable<RepositoryEnvironmentDeploymentBranchPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().deployment_branch_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.deployment_branch_policy = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `reviewers`.\n"]
    pub fn set_reviewers(self, v: impl Into<BlockAssignable<RepositoryEnvironmentReviewersEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().reviewers = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.reviewers = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `can_admins_bypass` after provisioning.\nCan Admins bypass deployment protections"]
    pub fn can_admins_bypass(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.can_admins_bypass", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `environment` after provisioning.\nThe name of the environment."]
    pub fn environment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\nThe repository of the environment."]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `wait_timer` after provisioning.\nAmount of time to delay a job after the job is initially triggered."]
    pub fn wait_timer(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.wait_timer", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deployment_branch_policy` after provisioning.\n"]
    pub fn deployment_branch_policy(&self) -> ListRef<RepositoryEnvironmentDeploymentBranchPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deployment_branch_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `reviewers` after provisioning.\n"]
    pub fn reviewers(&self) -> ListRef<RepositoryEnvironmentReviewersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.reviewers", self.extract_ref()))
    }
}

impl Referable for RepositoryEnvironment {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for RepositoryEnvironment { }

impl ToListMappable for RepositoryEnvironment {
    type O = ListRef<RepositoryEnvironmentRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for RepositoryEnvironment_ {
    fn extract_resource_type(&self) -> String {
        "github_repository_environment".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRepositoryEnvironment {
    pub tf_id: String,
    #[doc= "The name of the environment."]
    pub environment: PrimField<String>,
    #[doc= "The repository of the environment."]
    pub repository: PrimField<String>,
}

impl BuildRepositoryEnvironment {
    pub fn build(self, stack: &mut Stack) -> RepositoryEnvironment {
        let out = RepositoryEnvironment(Rc::new(RepositoryEnvironment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RepositoryEnvironmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                can_admins_bypass: core::default::Default::default(),
                environment: self.environment,
                id: core::default::Default::default(),
                repository: self.repository,
                wait_timer: core::default::Default::default(),
                deployment_branch_policy: core::default::Default::default(),
                reviewers: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RepositoryEnvironmentRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositoryEnvironmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RepositoryEnvironmentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `can_admins_bypass` after provisioning.\nCan Admins bypass deployment protections"]
    pub fn can_admins_bypass(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.can_admins_bypass", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `environment` after provisioning.\nThe name of the environment."]
    pub fn environment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\nThe repository of the environment."]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `wait_timer` after provisioning.\nAmount of time to delay a job after the job is initially triggered."]
    pub fn wait_timer(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.wait_timer", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deployment_branch_policy` after provisioning.\n"]
    pub fn deployment_branch_policy(&self) -> ListRef<RepositoryEnvironmentDeploymentBranchPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deployment_branch_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `reviewers` after provisioning.\n"]
    pub fn reviewers(&self) -> ListRef<RepositoryEnvironmentReviewersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.reviewers", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct RepositoryEnvironmentDeploymentBranchPolicyEl {
    custom_branch_policies: PrimField<bool>,
    protected_branches: PrimField<bool>,
}

impl RepositoryEnvironmentDeploymentBranchPolicyEl { }

impl ToListMappable for RepositoryEnvironmentDeploymentBranchPolicyEl {
    type O = BlockAssignable<RepositoryEnvironmentDeploymentBranchPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRepositoryEnvironmentDeploymentBranchPolicyEl {
    #[doc= "Whether only branches that match the specified name patterns can deploy to this environment."]
    pub custom_branch_policies: PrimField<bool>,
    #[doc= "Whether only branches with branch protection rules can deploy to this environment."]
    pub protected_branches: PrimField<bool>,
}

impl BuildRepositoryEnvironmentDeploymentBranchPolicyEl {
    pub fn build(self) -> RepositoryEnvironmentDeploymentBranchPolicyEl {
        RepositoryEnvironmentDeploymentBranchPolicyEl {
            custom_branch_policies: self.custom_branch_policies,
            protected_branches: self.protected_branches,
        }
    }
}

pub struct RepositoryEnvironmentDeploymentBranchPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositoryEnvironmentDeploymentBranchPolicyElRef {
    fn new(shared: StackShared, base: String) -> RepositoryEnvironmentDeploymentBranchPolicyElRef {
        RepositoryEnvironmentDeploymentBranchPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RepositoryEnvironmentDeploymentBranchPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `custom_branch_policies` after provisioning.\nWhether only branches that match the specified name patterns can deploy to this environment."]
    pub fn custom_branch_policies(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_branch_policies", self.base))
    }

    #[doc= "Get a reference to the value of field `protected_branches` after provisioning.\nWhether only branches with branch protection rules can deploy to this environment."]
    pub fn protected_branches(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.protected_branches", self.base))
    }
}

#[derive(Serialize)]
pub struct RepositoryEnvironmentReviewersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    teams: Option<SetField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    users: Option<SetField<PrimField<f64>>>,
}

impl RepositoryEnvironmentReviewersEl {
    #[doc= "Set the field `teams`.\nUp to 6 IDs for teams who may review jobs that reference the environment. Reviewers must have at least read access to the repository. Only one of the required reviewers needs to approve the job for it to proceed."]
    pub fn set_teams(mut self, v: impl Into<SetField<PrimField<f64>>>) -> Self {
        self.teams = Some(v.into());
        self
    }

    #[doc= "Set the field `users`.\nUp to 6 IDs for users who may review jobs that reference the environment. Reviewers must have at least read access to the repository. Only one of the required reviewers needs to approve the job for it to proceed."]
    pub fn set_users(mut self, v: impl Into<SetField<PrimField<f64>>>) -> Self {
        self.users = Some(v.into());
        self
    }
}

impl ToListMappable for RepositoryEnvironmentReviewersEl {
    type O = BlockAssignable<RepositoryEnvironmentReviewersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRepositoryEnvironmentReviewersEl {}

impl BuildRepositoryEnvironmentReviewersEl {
    pub fn build(self) -> RepositoryEnvironmentReviewersEl {
        RepositoryEnvironmentReviewersEl {
            teams: core::default::Default::default(),
            users: core::default::Default::default(),
        }
    }
}

pub struct RepositoryEnvironmentReviewersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositoryEnvironmentReviewersElRef {
    fn new(shared: StackShared, base: String) -> RepositoryEnvironmentReviewersElRef {
        RepositoryEnvironmentReviewersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RepositoryEnvironmentReviewersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `teams` after provisioning.\nUp to 6 IDs for teams who may review jobs that reference the environment. Reviewers must have at least read access to the repository. Only one of the required reviewers needs to approve the job for it to proceed."]
    pub fn teams(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.teams", self.base))
    }

    #[doc= "Get a reference to the value of field `users` after provisioning.\nUp to 6 IDs for users who may review jobs that reference the environment. Reviewers must have at least read access to the repository. Only one of the required reviewers needs to approve the job for it to proceed."]
    pub fn users(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.users", self.base))
    }
}

#[derive(Serialize, Default)]
struct RepositoryEnvironmentDynamic {
    deployment_branch_policy: Option<DynamicBlock<RepositoryEnvironmentDeploymentBranchPolicyEl>>,
    reviewers: Option<DynamicBlock<RepositoryEnvironmentReviewersEl>>,
}
