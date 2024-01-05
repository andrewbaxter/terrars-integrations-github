use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataOrganizationWebhooksData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

struct DataOrganizationWebhooks_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOrganizationWebhooksData>,
}

#[derive(Clone)]
pub struct DataOrganizationWebhooks(Rc<DataOrganizationWebhooks_>);

impl DataOrganizationWebhooks {
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

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `webhooks` after provisioning.\n"]
    pub fn webhooks(&self) -> ListRef<DataOrganizationWebhooksWebhooksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.webhooks", self.extract_ref()))
    }
}

impl Referable for DataOrganizationWebhooks {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataOrganizationWebhooks { }

impl ToListMappable for DataOrganizationWebhooks {
    type O = ListRef<DataOrganizationWebhooksRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataOrganizationWebhooks_ {
    fn extract_datasource_type(&self) -> String {
        "github_organization_webhooks".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOrganizationWebhooks {
    pub tf_id: String,
}

impl BuildDataOrganizationWebhooks {
    pub fn build(self, stack: &mut Stack) -> DataOrganizationWebhooks {
        let out = DataOrganizationWebhooks(Rc::new(DataOrganizationWebhooks_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataOrganizationWebhooksData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOrganizationWebhooksRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOrganizationWebhooksRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOrganizationWebhooksRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `webhooks` after provisioning.\n"]
    pub fn webhooks(&self) -> ListRef<DataOrganizationWebhooksWebhooksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.webhooks", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOrganizationWebhooksWebhooksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<PrimField<String>>,
}

impl DataOrganizationWebhooksWebhooksEl {
    #[doc= "Set the field `active`.\n"]
    pub fn set_active(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.active = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url = Some(v.into());
        self
    }
}

impl ToListMappable for DataOrganizationWebhooksWebhooksEl {
    type O = BlockAssignable<DataOrganizationWebhooksWebhooksEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOrganizationWebhooksWebhooksEl {}

impl BuildDataOrganizationWebhooksWebhooksEl {
    pub fn build(self) -> DataOrganizationWebhooksWebhooksEl {
        DataOrganizationWebhooksWebhooksEl {
            active: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
            url: core::default::Default::default(),
        }
    }
}

pub struct DataOrganizationWebhooksWebhooksElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOrganizationWebhooksWebhooksElRef {
    fn new(shared: StackShared, base: String) -> DataOrganizationWebhooksWebhooksElRef {
        DataOrganizationWebhooksWebhooksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOrganizationWebhooksWebhooksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `active` after provisioning.\n"]
    pub fn active(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.active", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
