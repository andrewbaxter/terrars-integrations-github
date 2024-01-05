use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataRepositoriesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_repo_id: Option<PrimField<bool>>,
    query: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    results_per_page: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<PrimField<String>>,
}

struct DataRepositories_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRepositoriesData>,
}

#[derive(Clone)]
pub struct DataRepositories(Rc<DataRepositories_>);

impl DataRepositories {
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

    #[doc= "Set the field `include_repo_id`.\n"]
    pub fn set_include_repo_id(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_repo_id = Some(v.into());
        self
    }

    #[doc= "Set the field `results_per_page`.\n"]
    pub fn set_results_per_page(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().results_per_page = Some(v.into());
        self
    }

    #[doc= "Set the field `sort`.\n"]
    pub fn set_sort(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().sort = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `full_names` after provisioning.\n"]
    pub fn full_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.full_names", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `include_repo_id` after provisioning.\n"]
    pub fn include_repo_id(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_repo_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `names` after provisioning.\n"]
    pub fn names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.names", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `query` after provisioning.\n"]
    pub fn query(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.query", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repo_ids` after provisioning.\n"]
    pub fn repo_ids(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.repo_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `results_per_page` after provisioning.\n"]
    pub fn results_per_page(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.results_per_page", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort` after provisioning.\n"]
    pub fn sort(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }
}

impl Referable for DataRepositories {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataRepositories { }

impl ToListMappable for DataRepositories {
    type O = ListRef<DataRepositoriesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataRepositories_ {
    fn extract_datasource_type(&self) -> String {
        "github_repositories".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRepositories {
    pub tf_id: String,
    #[doc= ""]
    pub query: PrimField<String>,
}

impl BuildDataRepositories {
    pub fn build(self, stack: &mut Stack) -> DataRepositories {
        let out = DataRepositories(Rc::new(DataRepositories_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRepositoriesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                include_repo_id: core::default::Default::default(),
                query: self.query,
                results_per_page: core::default::Default::default(),
                sort: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRepositoriesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRepositoriesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRepositoriesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `full_names` after provisioning.\n"]
    pub fn full_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.full_names", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `include_repo_id` after provisioning.\n"]
    pub fn include_repo_id(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_repo_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `names` after provisioning.\n"]
    pub fn names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.names", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `query` after provisioning.\n"]
    pub fn query(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.query", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repo_ids` after provisioning.\n"]
    pub fn repo_ids(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.repo_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `results_per_page` after provisioning.\n"]
    pub fn results_per_page(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.results_per_page", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort` after provisioning.\n"]
    pub fn sort(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }
}
