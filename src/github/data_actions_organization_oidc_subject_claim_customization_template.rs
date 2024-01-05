use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct DataActionsOrganizationOidcSubjectClaimCustomizationTemplateData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

struct DataActionsOrganizationOidcSubjectClaimCustomizationTemplate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataActionsOrganizationOidcSubjectClaimCustomizationTemplateData>,
}

#[derive(Clone)]
pub struct DataActionsOrganizationOidcSubjectClaimCustomizationTemplate(
    Rc<DataActionsOrganizationOidcSubjectClaimCustomizationTemplate_>,
);

impl DataActionsOrganizationOidcSubjectClaimCustomizationTemplate {
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
}

impl Referable for DataActionsOrganizationOidcSubjectClaimCustomizationTemplate {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataActionsOrganizationOidcSubjectClaimCustomizationTemplate { }

impl ToListMappable for DataActionsOrganizationOidcSubjectClaimCustomizationTemplate {
    type O = ListRef<DataActionsOrganizationOidcSubjectClaimCustomizationTemplateRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataActionsOrganizationOidcSubjectClaimCustomizationTemplate_ {
    fn extract_datasource_type(&self) -> String {
        "github_actions_organization_oidc_subject_claim_customization_template".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataActionsOrganizationOidcSubjectClaimCustomizationTemplate {
    pub tf_id: String,
}

impl BuildDataActionsOrganizationOidcSubjectClaimCustomizationTemplate {
    pub fn build(self, stack: &mut Stack) -> DataActionsOrganizationOidcSubjectClaimCustomizationTemplate {
        let out =
            DataActionsOrganizationOidcSubjectClaimCustomizationTemplate(
                Rc::new(DataActionsOrganizationOidcSubjectClaimCustomizationTemplate_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: RefCell::new(DataActionsOrganizationOidcSubjectClaimCustomizationTemplateData {
                        depends_on: core::default::Default::default(),
                        provider: None,
                        for_each: None,
                        id: core::default::Default::default(),
                    }),
                }),
            );
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataActionsOrganizationOidcSubjectClaimCustomizationTemplateRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataActionsOrganizationOidcSubjectClaimCustomizationTemplateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataActionsOrganizationOidcSubjectClaimCustomizationTemplateRef {
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
}
