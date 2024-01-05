use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataActionsRepositoryOidcSubjectClaimCustomizationTemplateData {
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

struct DataActionsRepositoryOidcSubjectClaimCustomizationTemplate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataActionsRepositoryOidcSubjectClaimCustomizationTemplateData>,
}

#[derive(Clone)]
pub struct DataActionsRepositoryOidcSubjectClaimCustomizationTemplate(
    Rc<DataActionsRepositoryOidcSubjectClaimCustomizationTemplate_>,
);

impl DataActionsRepositoryOidcSubjectClaimCustomizationTemplate {
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

    #[doc= "Get a reference to the value of field `include_claim_keys` after provisioning.\n"]
    pub fn include_claim_keys(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.include_claim_keys", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `use_default` after provisioning.\n"]
    pub fn use_default(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_default", self.extract_ref()))
    }
}

impl Referable for DataActionsRepositoryOidcSubjectClaimCustomizationTemplate {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataActionsRepositoryOidcSubjectClaimCustomizationTemplate { }

impl ToListMappable for DataActionsRepositoryOidcSubjectClaimCustomizationTemplate {
    type O = ListRef<DataActionsRepositoryOidcSubjectClaimCustomizationTemplateRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataActionsRepositoryOidcSubjectClaimCustomizationTemplate_ {
    fn extract_datasource_type(&self) -> String {
        "github_actions_repository_oidc_subject_claim_customization_template".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataActionsRepositoryOidcSubjectClaimCustomizationTemplate {
    pub tf_id: String,
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildDataActionsRepositoryOidcSubjectClaimCustomizationTemplate {
    pub fn build(self, stack: &mut Stack) -> DataActionsRepositoryOidcSubjectClaimCustomizationTemplate {
        let out =
            DataActionsRepositoryOidcSubjectClaimCustomizationTemplate(
                Rc::new(DataActionsRepositoryOidcSubjectClaimCustomizationTemplate_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: RefCell::new(DataActionsRepositoryOidcSubjectClaimCustomizationTemplateData {
                        depends_on: core::default::Default::default(),
                        provider: None,
                        for_each: None,
                        id: core::default::Default::default(),
                        name: self.name,
                    }),
                }),
            );
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataActionsRepositoryOidcSubjectClaimCustomizationTemplateRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataActionsRepositoryOidcSubjectClaimCustomizationTemplateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataActionsRepositoryOidcSubjectClaimCustomizationTemplateRef {
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

    #[doc= "Get a reference to the value of field `include_claim_keys` after provisioning.\n"]
    pub fn include_claim_keys(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.include_claim_keys", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `use_default` after provisioning.\n"]
    pub fn use_default(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_default", self.extract_ref()))
    }
}
