use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataRepositoryEnvironmentsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    repository: PrimField<String>,
}

struct DataRepositoryEnvironments_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRepositoryEnvironmentsData>,
}

#[derive(Clone)]
pub struct DataRepositoryEnvironments(Rc<DataRepositoryEnvironments_>);

impl DataRepositoryEnvironments {
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

    #[doc= "Get a reference to the value of field `environments` after provisioning.\n"]
    pub fn environments(&self) -> ListRef<DataRepositoryEnvironmentsEnvironmentsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.environments", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }
}

impl Referable for DataRepositoryEnvironments {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataRepositoryEnvironments { }

impl ToListMappable for DataRepositoryEnvironments {
    type O = ListRef<DataRepositoryEnvironmentsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataRepositoryEnvironments_ {
    fn extract_datasource_type(&self) -> String {
        "github_repository_environments".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRepositoryEnvironments {
    pub tf_id: String,
    #[doc= ""]
    pub repository: PrimField<String>,
}

impl BuildDataRepositoryEnvironments {
    pub fn build(self, stack: &mut Stack) -> DataRepositoryEnvironments {
        let out = DataRepositoryEnvironments(Rc::new(DataRepositoryEnvironments_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRepositoryEnvironmentsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                repository: self.repository,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRepositoryEnvironmentsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRepositoryEnvironmentsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRepositoryEnvironmentsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `environments` after provisioning.\n"]
    pub fn environments(&self) -> ListRef<DataRepositoryEnvironmentsEnvironmentsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.environments", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataRepositoryEnvironmentsEnvironmentsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_id: Option<PrimField<String>>,
}

impl DataRepositoryEnvironmentsEnvironmentsEl {
    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `node_id`.\n"]
    pub fn set_node_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataRepositoryEnvironmentsEnvironmentsEl {
    type O = BlockAssignable<DataRepositoryEnvironmentsEnvironmentsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRepositoryEnvironmentsEnvironmentsEl {}

impl BuildDataRepositoryEnvironmentsEnvironmentsEl {
    pub fn build(self) -> DataRepositoryEnvironmentsEnvironmentsEl {
        DataRepositoryEnvironmentsEnvironmentsEl {
            name: core::default::Default::default(),
            node_id: core::default::Default::default(),
        }
    }
}

pub struct DataRepositoryEnvironmentsEnvironmentsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRepositoryEnvironmentsEnvironmentsElRef {
    fn new(shared: StackShared, base: String) -> DataRepositoryEnvironmentsEnvironmentsElRef {
        DataRepositoryEnvironmentsEnvironmentsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRepositoryEnvironmentsEnvironmentsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `node_id` after provisioning.\n"]
    pub fn node_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_id", self.base))
    }
}
