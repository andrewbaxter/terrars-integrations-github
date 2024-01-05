use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct ProviderGithubData {
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insecure: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organization: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parallel_requests: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_delay_ms: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    write_delay_ms: Option<PrimField<f64>>,
}

struct ProviderGithub_ {
    data: RefCell<ProviderGithubData>,
}

pub struct ProviderGithub(Rc<ProviderGithub_>);

impl ProviderGithub {
    pub fn provider_ref(&self) -> String {
        let data = self.0.data.borrow();
        if let Some(alias) = &data.alias {
            format!("{}.{}", "github", alias)
        } else {
            "github".into()
        }
    }

    pub fn set_alias(self, alias: impl ToString) -> Self {
        self.0.data.borrow_mut().alias = Some(alias.to_string());
        self
    }

    #[doc= "Set the field `base_url`.\nThe GitHub Base API URL"]
    pub fn set_base_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().base_url = Some(v.into());
        self
    }

    #[doc= "Set the field `insecure`.\nEnable `insecure` mode for testing purposes"]
    pub fn set_insecure(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().insecure = Some(v.into());
        self
    }

    #[doc= "Set the field `organization`.\nThe GitHub organization name to manage. Use this field instead of `owner` when managing organization accounts."]
    pub fn set_organization(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().organization = Some(v.into());
        self
    }

    #[doc= "Set the field `owner`.\nThe GitHub owner name to manage. Use this field instead of `organization` when managing individual accounts."]
    pub fn set_owner(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().owner = Some(v.into());
        self
    }

    #[doc= "Set the field `parallel_requests`.\nAllow the provider to make parallel API calls to GitHub. You may want to set it to true when you have a private Github Enterprise without strict rate limits. Although, it is not possible to enable this setting on github.com because we enforce the respect of github.com's best practices to avoid hitting abuse rate limitsDefaults to false if not set"]
    pub fn set_parallel_requests(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().parallel_requests = Some(v.into());
        self
    }

    #[doc= "Set the field `read_delay_ms`.\nAmount of time in milliseconds to sleep in between non-write requests to GitHub API. Defaults to 0ms if not set."]
    pub fn set_read_delay_ms(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().read_delay_ms = Some(v.into());
        self
    }

    #[doc= "Set the field `token`.\nThe OAuth token used to connect to GitHub. Anonymous mode is enabled if both `token` and `app_auth` are not set."]
    pub fn set_token(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().token = Some(v.into());
        self
    }

    #[doc= "Set the field `write_delay_ms`.\nAmount of time in milliseconds to sleep in between writes to GitHub API. Defaults to 1000ms or 1s if not set."]
    pub fn set_write_delay_ms(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().write_delay_ms = Some(v.into());
        self
    }
}

impl Provider for ProviderGithub_ {
    fn extract_type_tf_id(&self) -> String {
        "github".into()
    }

    fn extract_provider_type(&self) -> serde_json::Value {
        serde_json::json!({
            "source": "integrations/github",
            "version": "5.43.0",
        })
    }

    fn extract_provider(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProviderGithub {}

impl BuildProviderGithub {
    pub fn build(self, stack: &mut Stack) -> ProviderGithub {
        let out = ProviderGithub(Rc::new(ProviderGithub_ { data: RefCell::new(ProviderGithubData {
            alias: None,
            base_url: core::default::Default::default(),
            insecure: core::default::Default::default(),
            organization: core::default::Default::default(),
            owner: core::default::Default::default(),
            parallel_requests: core::default::Default::default(),
            read_delay_ms: core::default::Default::default(),
            token: core::default::Default::default(),
            write_delay_ms: core::default::Default::default(),
        }) }));
        stack.add_provider(out.0.clone());
        out
    }
}
