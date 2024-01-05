use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataTeamData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    membership_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    results_per_page: Option<PrimField<f64>>,
    slug: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary_only: Option<PrimField<bool>>,
}

struct DataTeam_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataTeamData>,
}

#[derive(Clone)]
pub struct DataTeam(Rc<DataTeam_>);

impl DataTeam {
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

    #[doc= "Set the field `membership_type`.\n"]
    pub fn set_membership_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().membership_type = Some(v.into());
        self
    }

    #[doc= "Set the field `results_per_page`.\n"]
    pub fn set_results_per_page(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().results_per_page = Some(v.into());
        self
    }

    #[doc= "Set the field `summary_only`.\n"]
    pub fn set_summary_only(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().summary_only = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `members` after provisioning.\n"]
    pub fn members(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.members", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `membership_type` after provisioning.\n"]
    pub fn membership_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.membership_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_id` after provisioning.\n"]
    pub fn node_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `permission` after provisioning.\n"]
    pub fn permission(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.permission", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `privacy` after provisioning.\n"]
    pub fn privacy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.privacy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repositories` after provisioning.\n"]
    pub fn repositories(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.repositories", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repositories_detailed` after provisioning.\n"]
    pub fn repositories_detailed(&self) -> ListRef<DataTeamRepositoriesDetailedElRef> {
        ListRef::new(self.shared().clone(), format!("{}.repositories_detailed", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `results_per_page` after provisioning.\n"]
    pub fn results_per_page(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.results_per_page", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `slug` after provisioning.\n"]
    pub fn slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slug", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `summary_only` after provisioning.\n"]
    pub fn summary_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.summary_only", self.extract_ref()))
    }
}

impl Referable for DataTeam {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataTeam { }

impl ToListMappable for DataTeam {
    type O = ListRef<DataTeamRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataTeam_ {
    fn extract_datasource_type(&self) -> String {
        "github_team".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataTeam {
    pub tf_id: String,
    #[doc= ""]
    pub slug: PrimField<String>,
}

impl BuildDataTeam {
    pub fn build(self, stack: &mut Stack) -> DataTeam {
        let out = DataTeam(Rc::new(DataTeam_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataTeamData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                membership_type: core::default::Default::default(),
                results_per_page: core::default::Default::default(),
                slug: self.slug,
                summary_only: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataTeamRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataTeamRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataTeamRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `members` after provisioning.\n"]
    pub fn members(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.members", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `membership_type` after provisioning.\n"]
    pub fn membership_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.membership_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_id` after provisioning.\n"]
    pub fn node_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `permission` after provisioning.\n"]
    pub fn permission(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.permission", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `privacy` after provisioning.\n"]
    pub fn privacy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.privacy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repositories` after provisioning.\n"]
    pub fn repositories(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.repositories", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repositories_detailed` after provisioning.\n"]
    pub fn repositories_detailed(&self) -> ListRef<DataTeamRepositoriesDetailedElRef> {
        ListRef::new(self.shared().clone(), format!("{}.repositories_detailed", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `results_per_page` after provisioning.\n"]
    pub fn results_per_page(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.results_per_page", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `slug` after provisioning.\n"]
    pub fn slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slug", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `summary_only` after provisioning.\n"]
    pub fn summary_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.summary_only", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataTeamRepositoriesDetailedEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    repo_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_name: Option<PrimField<String>>,
}

impl DataTeamRepositoriesDetailedEl {
    #[doc= "Set the field `repo_id`.\n"]
    pub fn set_repo_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.repo_id = Some(v.into());
        self
    }

    #[doc= "Set the field `role_name`.\n"]
    pub fn set_role_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataTeamRepositoriesDetailedEl {
    type O = BlockAssignable<DataTeamRepositoriesDetailedEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataTeamRepositoriesDetailedEl {}

impl BuildDataTeamRepositoriesDetailedEl {
    pub fn build(self) -> DataTeamRepositoriesDetailedEl {
        DataTeamRepositoriesDetailedEl {
            repo_id: core::default::Default::default(),
            role_name: core::default::Default::default(),
        }
    }
}

pub struct DataTeamRepositoriesDetailedElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataTeamRepositoriesDetailedElRef {
    fn new(shared: StackShared, base: String) -> DataTeamRepositoriesDetailedElRef {
        DataTeamRepositoriesDetailedElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataTeamRepositoriesDetailedElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `repo_id` after provisioning.\n"]
    pub fn repo_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_id", self.base))
    }

    #[doc= "Get a reference to the value of field `role_name` after provisioning.\n"]
    pub fn role_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_name", self.base))
    }
}
