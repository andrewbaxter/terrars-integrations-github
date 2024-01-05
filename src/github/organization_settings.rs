use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct OrganizationSettingsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_security_enabled_for_new_repositories: Option<PrimField<bool>>,
    billing_email: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    blog: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    company: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_repository_permission: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dependabot_alerts_enabled_for_new_repositories: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dependabot_security_updates_enabled_for_new_repositories: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dependency_graph_enabled_for_new_repositories: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_organization_projects: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_repository_projects: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    members_can_create_internal_repositories: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    members_can_create_pages: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    members_can_create_private_pages: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    members_can_create_private_repositories: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    members_can_create_public_pages: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    members_can_create_public_repositories: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    members_can_create_repositories: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    members_can_fork_private_repositories: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_scanning_enabled_for_new_repositories: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_scanning_push_protection_enabled_for_new_repositories: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    twitter_username: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_commit_signoff_required: Option<PrimField<bool>>,
}

struct OrganizationSettings_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OrganizationSettingsData>,
}

#[derive(Clone)]
pub struct OrganizationSettings(Rc<OrganizationSettings_>);

impl OrganizationSettings {
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

    #[doc= "Set the field `advanced_security_enabled_for_new_repositories`.\n Whether or not advanced security is enabled for new repositories."]
    pub fn set_advanced_security_enabled_for_new_repositories(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().advanced_security_enabled_for_new_repositories = Some(v.into());
        self
    }

    #[doc= "Set the field `blog`.\nThe blog URL for the organization."]
    pub fn set_blog(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().blog = Some(v.into());
        self
    }

    #[doc= "Set the field `company`.\nThe company name for the organization."]
    pub fn set_company(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().company = Some(v.into());
        self
    }

    #[doc= "Set the field `default_repository_permission`.\nThe default permission for organization members to create new repositories. Can be one of 'read', 'write', 'admin' or 'none'."]
    pub fn set_default_repository_permission(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_repository_permission = Some(v.into());
        self
    }

    #[doc= "Set the field `dependabot_alerts_enabled_for_new_repositories`.\nWhether or not dependabot alerts are enabled for new repositories."]
    pub fn set_dependabot_alerts_enabled_for_new_repositories(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().dependabot_alerts_enabled_for_new_repositories = Some(v.into());
        self
    }

    #[doc= "Set the field `dependabot_security_updates_enabled_for_new_repositories`.\n Whether or not dependabot security updates are enabled for new repositories."]
    pub fn set_dependabot_security_updates_enabled_for_new_repositories(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().dependabot_security_updates_enabled_for_new_repositories = Some(v.into());
        self
    }

    #[doc= "Set the field `dependency_graph_enabled_for_new_repositories`.\nWhether or not dependency graph is enabled for new repositories."]
    pub fn set_dependency_graph_enabled_for_new_repositories(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().dependency_graph_enabled_for_new_repositories = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nThe description for the organization."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `email`.\nThe email address for the organization."]
    pub fn set_email(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().email = Some(v.into());
        self
    }

    #[doc= "Set the field `has_organization_projects`.\nWhether or not organization projects are enabled for the organization."]
    pub fn set_has_organization_projects(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().has_organization_projects = Some(v.into());
        self
    }

    #[doc= "Set the field `has_repository_projects`.\nWhether or not repository projects are enabled for the organization."]
    pub fn set_has_repository_projects(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().has_repository_projects = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `location`.\nThe location for the organization."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }

    #[doc= "Set the field `members_can_create_internal_repositories`.\nWhether or not organization members can create new internal repositories. For Enterprise Organizations only."]
    pub fn set_members_can_create_internal_repositories(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().members_can_create_internal_repositories = Some(v.into());
        self
    }

    #[doc= "Set the field `members_can_create_pages`.\nWhether or not organization members can create new pages."]
    pub fn set_members_can_create_pages(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().members_can_create_pages = Some(v.into());
        self
    }

    #[doc= "Set the field `members_can_create_private_pages`.\nWhether or not organization members can create new private pages."]
    pub fn set_members_can_create_private_pages(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().members_can_create_private_pages = Some(v.into());
        self
    }

    #[doc= "Set the field `members_can_create_private_repositories`.\nWhether or not organization members can create new private repositories."]
    pub fn set_members_can_create_private_repositories(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().members_can_create_private_repositories = Some(v.into());
        self
    }

    #[doc= "Set the field `members_can_create_public_pages`.\nWhether or not organization members can create new public pages."]
    pub fn set_members_can_create_public_pages(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().members_can_create_public_pages = Some(v.into());
        self
    }

    #[doc= "Set the field `members_can_create_public_repositories`.\nWhether or not organization members can create new public repositories."]
    pub fn set_members_can_create_public_repositories(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().members_can_create_public_repositories = Some(v.into());
        self
    }

    #[doc= "Set the field `members_can_create_repositories`.\nWhether or not organization members can create new repositories."]
    pub fn set_members_can_create_repositories(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().members_can_create_repositories = Some(v.into());
        self
    }

    #[doc= "Set the field `members_can_fork_private_repositories`.\nWhether or not organization members can fork private repositories."]
    pub fn set_members_can_fork_private_repositories(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().members_can_fork_private_repositories = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\nThe name for the organization."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc= "Set the field `secret_scanning_enabled_for_new_repositories`.\nWhether or not secret scanning is enabled for new repositories."]
    pub fn set_secret_scanning_enabled_for_new_repositories(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().secret_scanning_enabled_for_new_repositories = Some(v.into());
        self
    }

    #[doc= "Set the field `secret_scanning_push_protection_enabled_for_new_repositories`.\nWhether or not secret scanning push protection is enabled for new repositories."]
    pub fn set_secret_scanning_push_protection_enabled_for_new_repositories(
        self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().secret_scanning_push_protection_enabled_for_new_repositories = Some(v.into());
        self
    }

    #[doc= "Set the field `twitter_username`.\nThe Twitter username for the organization."]
    pub fn set_twitter_username(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().twitter_username = Some(v.into());
        self
    }

    #[doc= "Set the field `web_commit_signoff_required`.\nWhether or not commit signatures are required for commits to the organization."]
    pub fn set_web_commit_signoff_required(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().web_commit_signoff_required = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `advanced_security_enabled_for_new_repositories` after provisioning.\n Whether or not advanced security is enabled for new repositories."]
    pub fn advanced_security_enabled_for_new_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.advanced_security_enabled_for_new_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `billing_email` after provisioning.\nThe billing email address for the organization."]
    pub fn billing_email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.billing_email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `blog` after provisioning.\nThe blog URL for the organization."]
    pub fn blog(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.blog", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `company` after provisioning.\nThe company name for the organization."]
    pub fn company(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.company", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_repository_permission` after provisioning.\nThe default permission for organization members to create new repositories. Can be one of 'read', 'write', 'admin' or 'none'."]
    pub fn default_repository_permission(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_repository_permission", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dependabot_alerts_enabled_for_new_repositories` after provisioning.\nWhether or not dependabot alerts are enabled for new repositories."]
    pub fn dependabot_alerts_enabled_for_new_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dependabot_alerts_enabled_for_new_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `dependabot_security_updates_enabled_for_new_repositories` after provisioning.\n Whether or not dependabot security updates are enabled for new repositories."]
    pub fn dependabot_security_updates_enabled_for_new_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dependabot_security_updates_enabled_for_new_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `dependency_graph_enabled_for_new_repositories` after provisioning.\nWhether or not dependency graph is enabled for new repositories."]
    pub fn dependency_graph_enabled_for_new_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dependency_graph_enabled_for_new_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nThe description for the organization."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\nThe email address for the organization."]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `has_organization_projects` after provisioning.\nWhether or not organization projects are enabled for the organization."]
    pub fn has_organization_projects(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.has_organization_projects", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `has_repository_projects` after provisioning.\nWhether or not repository projects are enabled for the organization."]
    pub fn has_repository_projects(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.has_repository_projects", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe location for the organization."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `members_can_create_internal_repositories` after provisioning.\nWhether or not organization members can create new internal repositories. For Enterprise Organizations only."]
    pub fn members_can_create_internal_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.members_can_create_internal_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `members_can_create_pages` after provisioning.\nWhether or not organization members can create new pages."]
    pub fn members_can_create_pages(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.members_can_create_pages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `members_can_create_private_pages` after provisioning.\nWhether or not organization members can create new private pages."]
    pub fn members_can_create_private_pages(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.members_can_create_private_pages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `members_can_create_private_repositories` after provisioning.\nWhether or not organization members can create new private repositories."]
    pub fn members_can_create_private_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.members_can_create_private_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `members_can_create_public_pages` after provisioning.\nWhether or not organization members can create new public pages."]
    pub fn members_can_create_public_pages(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.members_can_create_public_pages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `members_can_create_public_repositories` after provisioning.\nWhether or not organization members can create new public repositories."]
    pub fn members_can_create_public_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.members_can_create_public_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `members_can_create_repositories` after provisioning.\nWhether or not organization members can create new repositories."]
    pub fn members_can_create_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.members_can_create_repositories", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `members_can_fork_private_repositories` after provisioning.\nWhether or not organization members can fork private repositories."]
    pub fn members_can_fork_private_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.members_can_fork_private_repositories", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name for the organization."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `secret_scanning_enabled_for_new_repositories` after provisioning.\nWhether or not secret scanning is enabled for new repositories."]
    pub fn secret_scanning_enabled_for_new_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_scanning_enabled_for_new_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `secret_scanning_push_protection_enabled_for_new_repositories` after provisioning.\nWhether or not secret scanning push protection is enabled for new repositories."]
    pub fn secret_scanning_push_protection_enabled_for_new_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_scanning_push_protection_enabled_for_new_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `twitter_username` after provisioning.\nThe Twitter username for the organization."]
    pub fn twitter_username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.twitter_username", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `web_commit_signoff_required` after provisioning.\nWhether or not commit signatures are required for commits to the organization."]
    pub fn web_commit_signoff_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_commit_signoff_required", self.extract_ref()))
    }
}

impl Referable for OrganizationSettings {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for OrganizationSettings { }

impl ToListMappable for OrganizationSettings {
    type O = ListRef<OrganizationSettingsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for OrganizationSettings_ {
    fn extract_resource_type(&self) -> String {
        "github_organization_settings".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOrganizationSettings {
    pub tf_id: String,
    #[doc= "The billing email address for the organization."]
    pub billing_email: PrimField<String>,
}

impl BuildOrganizationSettings {
    pub fn build(self, stack: &mut Stack) -> OrganizationSettings {
        let out = OrganizationSettings(Rc::new(OrganizationSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(OrganizationSettingsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                advanced_security_enabled_for_new_repositories: core::default::Default::default(),
                billing_email: self.billing_email,
                blog: core::default::Default::default(),
                company: core::default::Default::default(),
                default_repository_permission: core::default::Default::default(),
                dependabot_alerts_enabled_for_new_repositories: core::default::Default::default(),
                dependabot_security_updates_enabled_for_new_repositories: core::default::Default::default(),
                dependency_graph_enabled_for_new_repositories: core::default::Default::default(),
                description: core::default::Default::default(),
                email: core::default::Default::default(),
                has_organization_projects: core::default::Default::default(),
                has_repository_projects: core::default::Default::default(),
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                members_can_create_internal_repositories: core::default::Default::default(),
                members_can_create_pages: core::default::Default::default(),
                members_can_create_private_pages: core::default::Default::default(),
                members_can_create_private_repositories: core::default::Default::default(),
                members_can_create_public_pages: core::default::Default::default(),
                members_can_create_public_repositories: core::default::Default::default(),
                members_can_create_repositories: core::default::Default::default(),
                members_can_fork_private_repositories: core::default::Default::default(),
                name: core::default::Default::default(),
                secret_scanning_enabled_for_new_repositories: core::default::Default::default(),
                secret_scanning_push_protection_enabled_for_new_repositories: core::default::Default::default(),
                twitter_username: core::default::Default::default(),
                web_commit_signoff_required: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OrganizationSettingsRef {
    shared: StackShared,
    base: String,
}

impl Ref for OrganizationSettingsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OrganizationSettingsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `advanced_security_enabled_for_new_repositories` after provisioning.\n Whether or not advanced security is enabled for new repositories."]
    pub fn advanced_security_enabled_for_new_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.advanced_security_enabled_for_new_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `billing_email` after provisioning.\nThe billing email address for the organization."]
    pub fn billing_email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.billing_email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `blog` after provisioning.\nThe blog URL for the organization."]
    pub fn blog(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.blog", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `company` after provisioning.\nThe company name for the organization."]
    pub fn company(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.company", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_repository_permission` after provisioning.\nThe default permission for organization members to create new repositories. Can be one of 'read', 'write', 'admin' or 'none'."]
    pub fn default_repository_permission(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_repository_permission", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dependabot_alerts_enabled_for_new_repositories` after provisioning.\nWhether or not dependabot alerts are enabled for new repositories."]
    pub fn dependabot_alerts_enabled_for_new_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dependabot_alerts_enabled_for_new_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `dependabot_security_updates_enabled_for_new_repositories` after provisioning.\n Whether or not dependabot security updates are enabled for new repositories."]
    pub fn dependabot_security_updates_enabled_for_new_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dependabot_security_updates_enabled_for_new_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `dependency_graph_enabled_for_new_repositories` after provisioning.\nWhether or not dependency graph is enabled for new repositories."]
    pub fn dependency_graph_enabled_for_new_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dependency_graph_enabled_for_new_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nThe description for the organization."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\nThe email address for the organization."]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `has_organization_projects` after provisioning.\nWhether or not organization projects are enabled for the organization."]
    pub fn has_organization_projects(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.has_organization_projects", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `has_repository_projects` after provisioning.\nWhether or not repository projects are enabled for the organization."]
    pub fn has_repository_projects(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.has_repository_projects", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe location for the organization."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `members_can_create_internal_repositories` after provisioning.\nWhether or not organization members can create new internal repositories. For Enterprise Organizations only."]
    pub fn members_can_create_internal_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.members_can_create_internal_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `members_can_create_pages` after provisioning.\nWhether or not organization members can create new pages."]
    pub fn members_can_create_pages(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.members_can_create_pages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `members_can_create_private_pages` after provisioning.\nWhether or not organization members can create new private pages."]
    pub fn members_can_create_private_pages(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.members_can_create_private_pages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `members_can_create_private_repositories` after provisioning.\nWhether or not organization members can create new private repositories."]
    pub fn members_can_create_private_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.members_can_create_private_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `members_can_create_public_pages` after provisioning.\nWhether or not organization members can create new public pages."]
    pub fn members_can_create_public_pages(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.members_can_create_public_pages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `members_can_create_public_repositories` after provisioning.\nWhether or not organization members can create new public repositories."]
    pub fn members_can_create_public_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.members_can_create_public_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `members_can_create_repositories` after provisioning.\nWhether or not organization members can create new repositories."]
    pub fn members_can_create_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.members_can_create_repositories", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `members_can_fork_private_repositories` after provisioning.\nWhether or not organization members can fork private repositories."]
    pub fn members_can_fork_private_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.members_can_fork_private_repositories", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name for the organization."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `secret_scanning_enabled_for_new_repositories` after provisioning.\nWhether or not secret scanning is enabled for new repositories."]
    pub fn secret_scanning_enabled_for_new_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_scanning_enabled_for_new_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `secret_scanning_push_protection_enabled_for_new_repositories` after provisioning.\nWhether or not secret scanning push protection is enabled for new repositories."]
    pub fn secret_scanning_push_protection_enabled_for_new_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_scanning_push_protection_enabled_for_new_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `twitter_username` after provisioning.\nThe Twitter username for the organization."]
    pub fn twitter_username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.twitter_username", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `web_commit_signoff_required` after provisioning.\nWhether or not commit signatures are required for commits to the organization."]
    pub fn web_commit_signoff_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_commit_signoff_required", self.extract_ref()))
    }
}
