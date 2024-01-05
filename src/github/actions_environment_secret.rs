use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct ActionsEnvironmentSecretData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encrypted_value: Option<PrimField<String>>,
    environment: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plaintext_value: Option<PrimField<String>>,
    repository: PrimField<String>,
    secret_name: PrimField<String>,
}

struct ActionsEnvironmentSecret_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ActionsEnvironmentSecretData>,
}

#[derive(Clone)]
pub struct ActionsEnvironmentSecret(Rc<ActionsEnvironmentSecret_>);

impl ActionsEnvironmentSecret {
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

    #[doc= "Set the field `encrypted_value`.\nEncrypted value of the secret using the GitHub public key in Base64 format."]
    pub fn set_encrypted_value(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().encrypted_value = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `plaintext_value`.\nPlaintext value of the secret to be encrypted."]
    pub fn set_plaintext_value(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().plaintext_value = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\nDate of 'actions_environment_secret' creation."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `encrypted_value` after provisioning.\nEncrypted value of the secret using the GitHub public key in Base64 format."]
    pub fn encrypted_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encrypted_value", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `environment` after provisioning.\nName of the environment."]
    pub fn environment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `plaintext_value` after provisioning.\nPlaintext value of the secret to be encrypted."]
    pub fn plaintext_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.plaintext_value", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\nName of the repository."]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `secret_name` after provisioning.\nName of the secret."]
    pub fn secret_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\nDate of 'actions_environment_secret' update."]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

impl Referable for ActionsEnvironmentSecret {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ActionsEnvironmentSecret { }

impl ToListMappable for ActionsEnvironmentSecret {
    type O = ListRef<ActionsEnvironmentSecretRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ActionsEnvironmentSecret_ {
    fn extract_resource_type(&self) -> String {
        "github_actions_environment_secret".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildActionsEnvironmentSecret {
    pub tf_id: String,
    #[doc= "Name of the environment."]
    pub environment: PrimField<String>,
    #[doc= "Name of the repository."]
    pub repository: PrimField<String>,
    #[doc= "Name of the secret."]
    pub secret_name: PrimField<String>,
}

impl BuildActionsEnvironmentSecret {
    pub fn build(self, stack: &mut Stack) -> ActionsEnvironmentSecret {
        let out = ActionsEnvironmentSecret(Rc::new(ActionsEnvironmentSecret_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ActionsEnvironmentSecretData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                encrypted_value: core::default::Default::default(),
                environment: self.environment,
                id: core::default::Default::default(),
                plaintext_value: core::default::Default::default(),
                repository: self.repository,
                secret_name: self.secret_name,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ActionsEnvironmentSecretRef {
    shared: StackShared,
    base: String,
}

impl Ref for ActionsEnvironmentSecretRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ActionsEnvironmentSecretRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\nDate of 'actions_environment_secret' creation."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `encrypted_value` after provisioning.\nEncrypted value of the secret using the GitHub public key in Base64 format."]
    pub fn encrypted_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encrypted_value", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `environment` after provisioning.\nName of the environment."]
    pub fn environment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `plaintext_value` after provisioning.\nPlaintext value of the secret to be encrypted."]
    pub fn plaintext_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.plaintext_value", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\nName of the repository."]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `secret_name` after provisioning.\nName of the secret."]
    pub fn secret_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\nDate of 'actions_environment_secret' update."]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}
