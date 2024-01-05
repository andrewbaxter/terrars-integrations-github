use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct ActionsRepositoryOidcSubjectClaimCustomizationTemplateData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_claim_keys: Option<ListField<PrimField<String>>>,
    repository: PrimField<String>,
    use_default: PrimField<bool>,
}

struct ActionsRepositoryOidcSubjectClaimCustomizationTemplate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ActionsRepositoryOidcSubjectClaimCustomizationTemplateData>,
}

#[derive(Clone)]
pub struct ActionsRepositoryOidcSubjectClaimCustomizationTemplate(
    Rc<ActionsRepositoryOidcSubjectClaimCustomizationTemplate_>,
);

impl ActionsRepositoryOidcSubjectClaimCustomizationTemplate {
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

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `include_claim_keys`.\nA list of OpenID Connect claims."]
    pub fn set_include_claim_keys(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().include_claim_keys = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `include_claim_keys` after provisioning.\nA list of OpenID Connect claims."]
    pub fn include_claim_keys(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.include_claim_keys", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\nThe name of the repository."]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `use_default` after provisioning.\nWhether to use the default template or not. If 'true', 'include_claim_keys' must not be set."]
    pub fn use_default(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_default", self.extract_ref()))
    }
}

impl Referable for ActionsRepositoryOidcSubjectClaimCustomizationTemplate {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ActionsRepositoryOidcSubjectClaimCustomizationTemplate { }

impl ToListMappable for ActionsRepositoryOidcSubjectClaimCustomizationTemplate {
    type O = ListRef<ActionsRepositoryOidcSubjectClaimCustomizationTemplateRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ActionsRepositoryOidcSubjectClaimCustomizationTemplate_ {
    fn extract_resource_type(&self) -> String {
        "github_actions_repository_oidc_subject_claim_customization_template".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildActionsRepositoryOidcSubjectClaimCustomizationTemplate {
    pub tf_id: String,
    #[doc= "The name of the repository."]
    pub repository: PrimField<String>,
    #[doc= "Whether to use the default template or not. If 'true', 'include_claim_keys' must not be set."]
    pub use_default: PrimField<bool>,
}

impl BuildActionsRepositoryOidcSubjectClaimCustomizationTemplate {
    pub fn build(self, stack: &mut Stack) -> ActionsRepositoryOidcSubjectClaimCustomizationTemplate {
        let out =
            ActionsRepositoryOidcSubjectClaimCustomizationTemplate(
                Rc::new(ActionsRepositoryOidcSubjectClaimCustomizationTemplate_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: RefCell::new(ActionsRepositoryOidcSubjectClaimCustomizationTemplateData {
                        depends_on: core::default::Default::default(),
                        provider: None,
                        lifecycle: core::default::Default::default(),
                        for_each: None,
                        id: core::default::Default::default(),
                        include_claim_keys: core::default::Default::default(),
                        repository: self.repository,
                        use_default: self.use_default,
                    }),
                }),
            );
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ActionsRepositoryOidcSubjectClaimCustomizationTemplateRef {
    shared: StackShared,
    base: String,
}

impl Ref for ActionsRepositoryOidcSubjectClaimCustomizationTemplateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ActionsRepositoryOidcSubjectClaimCustomizationTemplateRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `include_claim_keys` after provisioning.\nA list of OpenID Connect claims."]
    pub fn include_claim_keys(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.include_claim_keys", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\nThe name of the repository."]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `use_default` after provisioning.\nWhether to use the default template or not. If 'true', 'include_claim_keys' must not be set."]
    pub fn use_default(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_default", self.extract_ref()))
    }
}
