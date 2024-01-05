use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataOrganizationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
}

struct DataOrganization_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOrganizationData>,
}

#[derive(Clone)]
pub struct DataOrganization(Rc<DataOrganization_>);

impl DataOrganization {
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

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `advanced_security_enabled_for_new_repositories` after provisioning.\n"]
    pub fn advanced_security_enabled_for_new_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.advanced_security_enabled_for_new_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `default_repository_permission` after provisioning.\n"]
    pub fn default_repository_permission(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_repository_permission", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dependabot_alerts_enabled_for_new_repositories` after provisioning.\n"]
    pub fn dependabot_alerts_enabled_for_new_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dependabot_alerts_enabled_for_new_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `dependabot_security_updates_enabled_for_new_repositories` after provisioning.\n"]
    pub fn dependabot_security_updates_enabled_for_new_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dependabot_security_updates_enabled_for_new_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `dependency_graph_enabled_for_new_repositories` after provisioning.\n"]
    pub fn dependency_graph_enabled_for_new_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dependency_graph_enabled_for_new_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `login` after provisioning.\n"]
    pub fn login(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.login", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `members` after provisioning.\n"]
    pub fn members(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.members", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `members_allowed_repository_creation_type` after provisioning.\n"]
    pub fn members_allowed_repository_creation_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.members_allowed_repository_creation_type", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `members_can_create_internal_repositories` after provisioning.\n"]
    pub fn members_can_create_internal_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.members_can_create_internal_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `members_can_create_pages` after provisioning.\n"]
    pub fn members_can_create_pages(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.members_can_create_pages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `members_can_create_private_pages` after provisioning.\n"]
    pub fn members_can_create_private_pages(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.members_can_create_private_pages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `members_can_create_private_repositories` after provisioning.\n"]
    pub fn members_can_create_private_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.members_can_create_private_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `members_can_create_public_pages` after provisioning.\n"]
    pub fn members_can_create_public_pages(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.members_can_create_public_pages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `members_can_create_public_repositories` after provisioning.\n"]
    pub fn members_can_create_public_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.members_can_create_public_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `members_can_create_repositories` after provisioning.\n"]
    pub fn members_can_create_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.members_can_create_repositories", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `members_can_fork_private_repositories` after provisioning.\n"]
    pub fn members_can_fork_private_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.members_can_fork_private_repositories", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_id` after provisioning.\n"]
    pub fn node_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `orgname` after provisioning.\n"]
    pub fn orgname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.orgname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `plan` after provisioning.\n"]
    pub fn plan(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.plan", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repositories` after provisioning.\n"]
    pub fn repositories(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.repositories", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `secret_scanning_enabled_for_new_repositories` after provisioning.\n"]
    pub fn secret_scanning_enabled_for_new_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_scanning_enabled_for_new_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `secret_scanning_push_protection_enabled_for_new_repositories` after provisioning.\n"]
    pub fn secret_scanning_push_protection_enabled_for_new_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_scanning_push_protection_enabled_for_new_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `two_factor_requirement_enabled` after provisioning.\n"]
    pub fn two_factor_requirement_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.two_factor_requirement_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `users` after provisioning.\n"]
    pub fn users(&self) -> ListRef<RecRef<PrimExpr<String>>> {
        ListRef::new(self.shared().clone(), format!("{}.users", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `web_commit_signoff_required` after provisioning.\n"]
    pub fn web_commit_signoff_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_commit_signoff_required", self.extract_ref()))
    }
}

impl Referable for DataOrganization {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataOrganization { }

impl ToListMappable for DataOrganization {
    type O = ListRef<DataOrganizationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataOrganization_ {
    fn extract_datasource_type(&self) -> String {
        "github_organization".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOrganization {
    pub tf_id: String,
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildDataOrganization {
    pub fn build(self, stack: &mut Stack) -> DataOrganization {
        let out = DataOrganization(Rc::new(DataOrganization_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataOrganizationData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOrganizationRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOrganizationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOrganizationRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `advanced_security_enabled_for_new_repositories` after provisioning.\n"]
    pub fn advanced_security_enabled_for_new_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.advanced_security_enabled_for_new_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `default_repository_permission` after provisioning.\n"]
    pub fn default_repository_permission(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_repository_permission", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dependabot_alerts_enabled_for_new_repositories` after provisioning.\n"]
    pub fn dependabot_alerts_enabled_for_new_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dependabot_alerts_enabled_for_new_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `dependabot_security_updates_enabled_for_new_repositories` after provisioning.\n"]
    pub fn dependabot_security_updates_enabled_for_new_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dependabot_security_updates_enabled_for_new_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `dependency_graph_enabled_for_new_repositories` after provisioning.\n"]
    pub fn dependency_graph_enabled_for_new_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dependency_graph_enabled_for_new_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `login` after provisioning.\n"]
    pub fn login(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.login", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `members` after provisioning.\n"]
    pub fn members(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.members", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `members_allowed_repository_creation_type` after provisioning.\n"]
    pub fn members_allowed_repository_creation_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.members_allowed_repository_creation_type", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `members_can_create_internal_repositories` after provisioning.\n"]
    pub fn members_can_create_internal_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.members_can_create_internal_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `members_can_create_pages` after provisioning.\n"]
    pub fn members_can_create_pages(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.members_can_create_pages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `members_can_create_private_pages` after provisioning.\n"]
    pub fn members_can_create_private_pages(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.members_can_create_private_pages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `members_can_create_private_repositories` after provisioning.\n"]
    pub fn members_can_create_private_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.members_can_create_private_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `members_can_create_public_pages` after provisioning.\n"]
    pub fn members_can_create_public_pages(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.members_can_create_public_pages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `members_can_create_public_repositories` after provisioning.\n"]
    pub fn members_can_create_public_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.members_can_create_public_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `members_can_create_repositories` after provisioning.\n"]
    pub fn members_can_create_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.members_can_create_repositories", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `members_can_fork_private_repositories` after provisioning.\n"]
    pub fn members_can_fork_private_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.members_can_fork_private_repositories", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_id` after provisioning.\n"]
    pub fn node_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `orgname` after provisioning.\n"]
    pub fn orgname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.orgname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `plan` after provisioning.\n"]
    pub fn plan(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.plan", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repositories` after provisioning.\n"]
    pub fn repositories(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.repositories", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `secret_scanning_enabled_for_new_repositories` after provisioning.\n"]
    pub fn secret_scanning_enabled_for_new_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_scanning_enabled_for_new_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `secret_scanning_push_protection_enabled_for_new_repositories` after provisioning.\n"]
    pub fn secret_scanning_push_protection_enabled_for_new_repositories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_scanning_push_protection_enabled_for_new_repositories", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `two_factor_requirement_enabled` after provisioning.\n"]
    pub fn two_factor_requirement_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.two_factor_requirement_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `users` after provisioning.\n"]
    pub fn users(&self) -> ListRef<RecRef<PrimExpr<String>>> {
        ListRef::new(self.shared().clone(), format!("{}.users", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `web_commit_signoff_required` after provisioning.\n"]
    pub fn web_commit_signoff_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.web_commit_signoff_required", self.extract_ref()))
    }
}
