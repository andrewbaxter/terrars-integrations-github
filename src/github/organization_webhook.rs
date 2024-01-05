use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct OrganizationWebhookData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<PrimField<bool>>,
    events: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<Vec<OrganizationWebhookConfigurationEl>>,
    dynamic: OrganizationWebhookDynamic,
}

struct OrganizationWebhook_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OrganizationWebhookData>,
}

#[derive(Clone)]
pub struct OrganizationWebhook(Rc<OrganizationWebhook_>);

impl OrganizationWebhook {
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

    #[doc= "Set the field `active`.\nIndicate if the webhook should receive events."]
    pub fn set_active(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().active = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc= "Set the field `configuration`.\n"]
    pub fn set_configuration(self, v: impl Into<BlockAssignable<OrganizationWebhookConfigurationEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `active` after provisioning.\nIndicate if the webhook should receive events."]
    pub fn active(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.active", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `events` after provisioning.\nA list of events which should trigger the webhook."]
    pub fn events(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\nURL of the webhook."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<OrganizationWebhookConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }
}

impl Referable for OrganizationWebhook {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for OrganizationWebhook { }

impl ToListMappable for OrganizationWebhook {
    type O = ListRef<OrganizationWebhookRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for OrganizationWebhook_ {
    fn extract_resource_type(&self) -> String {
        "github_organization_webhook".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOrganizationWebhook {
    pub tf_id: String,
    #[doc= "A list of events which should trigger the webhook."]
    pub events: SetField<PrimField<String>>,
}

impl BuildOrganizationWebhook {
    pub fn build(self, stack: &mut Stack) -> OrganizationWebhook {
        let out = OrganizationWebhook(Rc::new(OrganizationWebhook_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(OrganizationWebhookData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                active: core::default::Default::default(),
                events: self.events,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OrganizationWebhookRef {
    shared: StackShared,
    base: String,
}

impl Ref for OrganizationWebhookRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OrganizationWebhookRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `active` after provisioning.\nIndicate if the webhook should receive events."]
    pub fn active(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.active", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `events` after provisioning.\nA list of events which should trigger the webhook."]
    pub fn events(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\nURL of the webhook."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<OrganizationWebhookConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct OrganizationWebhookConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insecure_ssl: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret: Option<PrimField<String>>,
    url: PrimField<String>,
}

impl OrganizationWebhookConfigurationEl {
    #[doc= "Set the field `content_type`.\nThe content type for the payload. Valid values are either 'form' or 'json'."]
    pub fn set_content_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content_type = Some(v.into());
        self
    }

    #[doc= "Set the field `insecure_ssl`.\nInsecure SSL boolean toggle. Defaults to 'false'."]
    pub fn set_insecure_ssl(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.insecure_ssl = Some(v.into());
        self
    }

    #[doc= "Set the field `secret`.\nThe shared secret for the webhook"]
    pub fn set_secret(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret = Some(v.into());
        self
    }
}

impl ToListMappable for OrganizationWebhookConfigurationEl {
    type O = BlockAssignable<OrganizationWebhookConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOrganizationWebhookConfigurationEl {
    #[doc= "The URL of the webhook."]
    pub url: PrimField<String>,
}

impl BuildOrganizationWebhookConfigurationEl {
    pub fn build(self) -> OrganizationWebhookConfigurationEl {
        OrganizationWebhookConfigurationEl {
            content_type: core::default::Default::default(),
            insecure_ssl: core::default::Default::default(),
            secret: core::default::Default::default(),
            url: self.url,
        }
    }
}

pub struct OrganizationWebhookConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OrganizationWebhookConfigurationElRef {
    fn new(shared: StackShared, base: String) -> OrganizationWebhookConfigurationElRef {
        OrganizationWebhookConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OrganizationWebhookConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `content_type` after provisioning.\nThe content type for the payload. Valid values are either 'form' or 'json'."]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc= "Get a reference to the value of field `insecure_ssl` after provisioning.\nInsecure SSL boolean toggle. Defaults to 'false'."]
    pub fn insecure_ssl(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.insecure_ssl", self.base))
    }

    #[doc= "Get a reference to the value of field `secret` after provisioning.\nThe shared secret for the webhook"]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\nThe URL of the webhook."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize, Default)]
struct OrganizationWebhookDynamic {
    configuration: Option<DynamicBlock<OrganizationWebhookConfigurationEl>>,
}
