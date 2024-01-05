use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct OrganizationRulesetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    enforcement: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    target: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bypass_actors: Option<Vec<OrganizationRulesetBypassActorsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conditions: Option<Vec<OrganizationRulesetConditionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules: Option<Vec<OrganizationRulesetRulesEl>>,
    dynamic: OrganizationRulesetDynamic,
}

struct OrganizationRuleset_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OrganizationRulesetData>,
}

#[derive(Clone)]
pub struct OrganizationRuleset(Rc<OrganizationRuleset_>);

impl OrganizationRuleset {
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

    #[doc= "Set the field `bypass_actors`.\n"]
    pub fn set_bypass_actors(self, v: impl Into<BlockAssignable<OrganizationRulesetBypassActorsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bypass_actors = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bypass_actors = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `conditions`.\n"]
    pub fn set_conditions(self, v: impl Into<BlockAssignable<OrganizationRulesetConditionsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().conditions = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.conditions = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `rules`.\n"]
    pub fn set_rules(self, v: impl Into<BlockAssignable<OrganizationRulesetRulesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rules = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rules = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `enforcement` after provisioning.\nPossible values for Enforcement are `disabled`, `active`, `evaluate`. Note: `evaluate` is currently only supported for owners of type `organization`."]
    pub fn enforcement(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.enforcement", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the ruleset."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_id` after provisioning.\nGraphQL global node id for use with v4 API."]
    pub fn node_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ruleset_id` after provisioning.\nGitHub ID for the ruleset."]
    pub fn ruleset_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ruleset_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `target` after provisioning.\nPossible values are `branch` and `tag`."]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bypass_actors` after provisioning.\n"]
    pub fn bypass_actors(&self) -> ListRef<OrganizationRulesetBypassActorsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bypass_actors", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> ListRef<OrganizationRulesetConditionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.conditions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<OrganizationRulesetRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }
}

impl Referable for OrganizationRuleset {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for OrganizationRuleset { }

impl ToListMappable for OrganizationRuleset {
    type O = ListRef<OrganizationRulesetRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for OrganizationRuleset_ {
    fn extract_resource_type(&self) -> String {
        "github_organization_ruleset".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOrganizationRuleset {
    pub tf_id: String,
    #[doc= "Possible values for Enforcement are `disabled`, `active`, `evaluate`. Note: `evaluate` is currently only supported for owners of type `organization`."]
    pub enforcement: PrimField<String>,
    #[doc= "The name of the ruleset."]
    pub name: PrimField<String>,
    #[doc= "Possible values are `branch` and `tag`."]
    pub target: PrimField<String>,
}

impl BuildOrganizationRuleset {
    pub fn build(self, stack: &mut Stack) -> OrganizationRuleset {
        let out = OrganizationRuleset(Rc::new(OrganizationRuleset_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(OrganizationRulesetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                enforcement: self.enforcement,
                id: core::default::Default::default(),
                name: self.name,
                target: self.target,
                bypass_actors: core::default::Default::default(),
                conditions: core::default::Default::default(),
                rules: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OrganizationRulesetRef {
    shared: StackShared,
    base: String,
}

impl Ref for OrganizationRulesetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OrganizationRulesetRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enforcement` after provisioning.\nPossible values for Enforcement are `disabled`, `active`, `evaluate`. Note: `evaluate` is currently only supported for owners of type `organization`."]
    pub fn enforcement(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.enforcement", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the ruleset."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_id` after provisioning.\nGraphQL global node id for use with v4 API."]
    pub fn node_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ruleset_id` after provisioning.\nGitHub ID for the ruleset."]
    pub fn ruleset_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ruleset_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `target` after provisioning.\nPossible values are `branch` and `tag`."]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bypass_actors` after provisioning.\n"]
    pub fn bypass_actors(&self) -> ListRef<OrganizationRulesetBypassActorsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bypass_actors", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> ListRef<OrganizationRulesetConditionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.conditions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<OrganizationRulesetRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct OrganizationRulesetBypassActorsEl {
    actor_id: PrimField<f64>,
    actor_type: PrimField<String>,
    bypass_mode: PrimField<String>,
}

impl OrganizationRulesetBypassActorsEl { }

impl ToListMappable for OrganizationRulesetBypassActorsEl {
    type O = BlockAssignable<OrganizationRulesetBypassActorsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOrganizationRulesetBypassActorsEl {
    #[doc= "The ID of the actor that can bypass a ruleset. When `actor_type` is `OrganizationAdmin`, this should be set to `1`."]
    pub actor_id: PrimField<f64>,
    #[doc= "The type of actor that can bypass a ruleset. Can be one of: `RepositoryRole`, `Team`, `Integration`, `OrganizationAdmin`."]
    pub actor_type: PrimField<String>,
    #[doc= "When the specified actor can bypass the ruleset. pull_request means that an actor can only bypass rules on pull requests. Can be one of: `always`, `pull_request`."]
    pub bypass_mode: PrimField<String>,
}

impl BuildOrganizationRulesetBypassActorsEl {
    pub fn build(self) -> OrganizationRulesetBypassActorsEl {
        OrganizationRulesetBypassActorsEl {
            actor_id: self.actor_id,
            actor_type: self.actor_type,
            bypass_mode: self.bypass_mode,
        }
    }
}

pub struct OrganizationRulesetBypassActorsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OrganizationRulesetBypassActorsElRef {
    fn new(shared: StackShared, base: String) -> OrganizationRulesetBypassActorsElRef {
        OrganizationRulesetBypassActorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OrganizationRulesetBypassActorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `actor_id` after provisioning.\nThe ID of the actor that can bypass a ruleset. When `actor_type` is `OrganizationAdmin`, this should be set to `1`."]
    pub fn actor_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.actor_id", self.base))
    }

    #[doc= "Get a reference to the value of field `actor_type` after provisioning.\nThe type of actor that can bypass a ruleset. Can be one of: `RepositoryRole`, `Team`, `Integration`, `OrganizationAdmin`."]
    pub fn actor_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.actor_type", self.base))
    }

    #[doc= "Get a reference to the value of field `bypass_mode` after provisioning.\nWhen the specified actor can bypass the ruleset. pull_request means that an actor can only bypass rules on pull requests. Can be one of: `always`, `pull_request`."]
    pub fn bypass_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bypass_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct OrganizationRulesetConditionsElRefNameEl {
    exclude: ListField<PrimField<String>>,
    include: ListField<PrimField<String>>,
}

impl OrganizationRulesetConditionsElRefNameEl { }

impl ToListMappable for OrganizationRulesetConditionsElRefNameEl {
    type O = BlockAssignable<OrganizationRulesetConditionsElRefNameEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOrganizationRulesetConditionsElRefNameEl {
    #[doc= "Array of ref names or patterns to exclude. The condition will not pass if any of these patterns match."]
    pub exclude: ListField<PrimField<String>>,
    #[doc= "Array of ref names or patterns to include. One of these patterns must match for the condition to pass. Also accepts `~DEFAULT_BRANCH` to include the default branch or `~ALL` to include all branches."]
    pub include: ListField<PrimField<String>>,
}

impl BuildOrganizationRulesetConditionsElRefNameEl {
    pub fn build(self) -> OrganizationRulesetConditionsElRefNameEl {
        OrganizationRulesetConditionsElRefNameEl {
            exclude: self.exclude,
            include: self.include,
        }
    }
}

pub struct OrganizationRulesetConditionsElRefNameElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OrganizationRulesetConditionsElRefNameElRef {
    fn new(shared: StackShared, base: String) -> OrganizationRulesetConditionsElRefNameElRef {
        OrganizationRulesetConditionsElRefNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OrganizationRulesetConditionsElRefNameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `exclude` after provisioning.\nArray of ref names or patterns to exclude. The condition will not pass if any of these patterns match."]
    pub fn exclude(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.exclude", self.base))
    }

    #[doc= "Get a reference to the value of field `include` after provisioning.\nArray of ref names or patterns to include. One of these patterns must match for the condition to pass. Also accepts `~DEFAULT_BRANCH` to include the default branch or `~ALL` to include all branches."]
    pub fn include(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.include", self.base))
    }
}

#[derive(Serialize)]
pub struct OrganizationRulesetConditionsElRepositoryNameEl {
    exclude: ListField<PrimField<String>>,
    include: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protected: Option<PrimField<bool>>,
}

impl OrganizationRulesetConditionsElRepositoryNameEl {
    #[doc= "Set the field `protected`.\nWhether renaming of target repositories is prevented."]
    pub fn set_protected(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.protected = Some(v.into());
        self
    }
}

impl ToListMappable for OrganizationRulesetConditionsElRepositoryNameEl {
    type O = BlockAssignable<OrganizationRulesetConditionsElRepositoryNameEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOrganizationRulesetConditionsElRepositoryNameEl {
    #[doc= "Array of repository names or patterns to exclude. The condition will not pass if any of these patterns match."]
    pub exclude: ListField<PrimField<String>>,
    #[doc= "Array of repository names or patterns to include. One of these patterns must match for the condition to pass. Also accepts `~ALL` to include all repositories."]
    pub include: ListField<PrimField<String>>,
}

impl BuildOrganizationRulesetConditionsElRepositoryNameEl {
    pub fn build(self) -> OrganizationRulesetConditionsElRepositoryNameEl {
        OrganizationRulesetConditionsElRepositoryNameEl {
            exclude: self.exclude,
            include: self.include,
            protected: core::default::Default::default(),
        }
    }
}

pub struct OrganizationRulesetConditionsElRepositoryNameElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OrganizationRulesetConditionsElRepositoryNameElRef {
    fn new(shared: StackShared, base: String) -> OrganizationRulesetConditionsElRepositoryNameElRef {
        OrganizationRulesetConditionsElRepositoryNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OrganizationRulesetConditionsElRepositoryNameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `exclude` after provisioning.\nArray of repository names or patterns to exclude. The condition will not pass if any of these patterns match."]
    pub fn exclude(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.exclude", self.base))
    }

    #[doc= "Get a reference to the value of field `include` after provisioning.\nArray of repository names or patterns to include. One of these patterns must match for the condition to pass. Also accepts `~ALL` to include all repositories."]
    pub fn include(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.include", self.base))
    }

    #[doc= "Get a reference to the value of field `protected` after provisioning.\nWhether renaming of target repositories is prevented."]
    pub fn protected(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.protected", self.base))
    }
}

#[derive(Serialize, Default)]
struct OrganizationRulesetConditionsElDynamic {
    ref_name: Option<DynamicBlock<OrganizationRulesetConditionsElRefNameEl>>,
    repository_name: Option<DynamicBlock<OrganizationRulesetConditionsElRepositoryNameEl>>,
}

#[derive(Serialize)]
pub struct OrganizationRulesetConditionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    repository_id: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ref_name: Option<Vec<OrganizationRulesetConditionsElRefNameEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository_name: Option<Vec<OrganizationRulesetConditionsElRepositoryNameEl>>,
    dynamic: OrganizationRulesetConditionsElDynamic,
}

impl OrganizationRulesetConditionsEl {
    #[doc= "Set the field `repository_id`.\nThe repository IDs that the ruleset applies to. One of these IDs must match for the condition to pass."]
    pub fn set_repository_id(mut self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.repository_id = Some(v.into());
        self
    }

    #[doc= "Set the field `ref_name`.\n"]
    pub fn set_ref_name(mut self, v: impl Into<BlockAssignable<OrganizationRulesetConditionsElRefNameEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ref_name = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ref_name = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `repository_name`.\n"]
    pub fn set_repository_name(
        mut self,
        v: impl Into<BlockAssignable<OrganizationRulesetConditionsElRepositoryNameEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.repository_name = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.repository_name = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for OrganizationRulesetConditionsEl {
    type O = BlockAssignable<OrganizationRulesetConditionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOrganizationRulesetConditionsEl {}

impl BuildOrganizationRulesetConditionsEl {
    pub fn build(self) -> OrganizationRulesetConditionsEl {
        OrganizationRulesetConditionsEl {
            repository_id: core::default::Default::default(),
            ref_name: core::default::Default::default(),
            repository_name: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct OrganizationRulesetConditionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OrganizationRulesetConditionsElRef {
    fn new(shared: StackShared, base: String) -> OrganizationRulesetConditionsElRef {
        OrganizationRulesetConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OrganizationRulesetConditionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `repository_id` after provisioning.\nThe repository IDs that the ruleset applies to. One of these IDs must match for the condition to pass."]
    pub fn repository_id(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.repository_id", self.base))
    }

    #[doc= "Get a reference to the value of field `ref_name` after provisioning.\n"]
    pub fn ref_name(&self) -> ListRef<OrganizationRulesetConditionsElRefNameElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ref_name", self.base))
    }

    #[doc= "Get a reference to the value of field `repository_name` after provisioning.\n"]
    pub fn repository_name(&self) -> ListRef<OrganizationRulesetConditionsElRepositoryNameElRef> {
        ListRef::new(self.shared().clone(), format!("{}.repository_name", self.base))
    }
}

#[derive(Serialize)]
pub struct OrganizationRulesetRulesElBranchNamePatternEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    negate: Option<PrimField<bool>>,
    operator: PrimField<String>,
    pattern: PrimField<String>,
}

impl OrganizationRulesetRulesElBranchNamePatternEl {
    #[doc= "Set the field `name`.\nHow this rule will appear to users."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `negate`.\nIf true, the rule will fail if the pattern matches."]
    pub fn set_negate(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.negate = Some(v.into());
        self
    }
}

impl ToListMappable for OrganizationRulesetRulesElBranchNamePatternEl {
    type O = BlockAssignable<OrganizationRulesetRulesElBranchNamePatternEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOrganizationRulesetRulesElBranchNamePatternEl {
    #[doc= "The operator to use for matching. Can be one of: `starts_with`, `ends_with`, `contains`, `regex`."]
    pub operator: PrimField<String>,
    #[doc= "The pattern to match with."]
    pub pattern: PrimField<String>,
}

impl BuildOrganizationRulesetRulesElBranchNamePatternEl {
    pub fn build(self) -> OrganizationRulesetRulesElBranchNamePatternEl {
        OrganizationRulesetRulesElBranchNamePatternEl {
            name: core::default::Default::default(),
            negate: core::default::Default::default(),
            operator: self.operator,
            pattern: self.pattern,
        }
    }
}

pub struct OrganizationRulesetRulesElBranchNamePatternElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OrganizationRulesetRulesElBranchNamePatternElRef {
    fn new(shared: StackShared, base: String) -> OrganizationRulesetRulesElBranchNamePatternElRef {
        OrganizationRulesetRulesElBranchNamePatternElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OrganizationRulesetRulesElBranchNamePatternElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nHow this rule will appear to users."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `negate` after provisioning.\nIf true, the rule will fail if the pattern matches."]
    pub fn negate(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.negate", self.base))
    }

    #[doc= "Get a reference to the value of field `operator` after provisioning.\nThe operator to use for matching. Can be one of: `starts_with`, `ends_with`, `contains`, `regex`."]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc= "Get a reference to the value of field `pattern` after provisioning.\nThe pattern to match with."]
    pub fn pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }
}

#[derive(Serialize)]
pub struct OrganizationRulesetRulesElCommitAuthorEmailPatternEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    negate: Option<PrimField<bool>>,
    operator: PrimField<String>,
    pattern: PrimField<String>,
}

impl OrganizationRulesetRulesElCommitAuthorEmailPatternEl {
    #[doc= "Set the field `name`.\nHow this rule will appear to users."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `negate`.\nIf true, the rule will fail if the pattern matches."]
    pub fn set_negate(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.negate = Some(v.into());
        self
    }
}

impl ToListMappable for OrganizationRulesetRulesElCommitAuthorEmailPatternEl {
    type O = BlockAssignable<OrganizationRulesetRulesElCommitAuthorEmailPatternEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOrganizationRulesetRulesElCommitAuthorEmailPatternEl {
    #[doc= "The operator to use for matching. Can be one of: `starts_with`, `ends_with`, `contains`, `regex`."]
    pub operator: PrimField<String>,
    #[doc= "The pattern to match with."]
    pub pattern: PrimField<String>,
}

impl BuildOrganizationRulesetRulesElCommitAuthorEmailPatternEl {
    pub fn build(self) -> OrganizationRulesetRulesElCommitAuthorEmailPatternEl {
        OrganizationRulesetRulesElCommitAuthorEmailPatternEl {
            name: core::default::Default::default(),
            negate: core::default::Default::default(),
            operator: self.operator,
            pattern: self.pattern,
        }
    }
}

pub struct OrganizationRulesetRulesElCommitAuthorEmailPatternElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OrganizationRulesetRulesElCommitAuthorEmailPatternElRef {
    fn new(shared: StackShared, base: String) -> OrganizationRulesetRulesElCommitAuthorEmailPatternElRef {
        OrganizationRulesetRulesElCommitAuthorEmailPatternElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OrganizationRulesetRulesElCommitAuthorEmailPatternElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nHow this rule will appear to users."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `negate` after provisioning.\nIf true, the rule will fail if the pattern matches."]
    pub fn negate(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.negate", self.base))
    }

    #[doc= "Get a reference to the value of field `operator` after provisioning.\nThe operator to use for matching. Can be one of: `starts_with`, `ends_with`, `contains`, `regex`."]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc= "Get a reference to the value of field `pattern` after provisioning.\nThe pattern to match with."]
    pub fn pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }
}

#[derive(Serialize)]
pub struct OrganizationRulesetRulesElCommitMessagePatternEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    negate: Option<PrimField<bool>>,
    operator: PrimField<String>,
    pattern: PrimField<String>,
}

impl OrganizationRulesetRulesElCommitMessagePatternEl {
    #[doc= "Set the field `name`.\nHow this rule will appear to users."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `negate`.\nIf true, the rule will fail if the pattern matches."]
    pub fn set_negate(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.negate = Some(v.into());
        self
    }
}

impl ToListMappable for OrganizationRulesetRulesElCommitMessagePatternEl {
    type O = BlockAssignable<OrganizationRulesetRulesElCommitMessagePatternEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOrganizationRulesetRulesElCommitMessagePatternEl {
    #[doc= "The operator to use for matching. Can be one of: `starts_with`, `ends_with`, `contains`, `regex`."]
    pub operator: PrimField<String>,
    #[doc= "The pattern to match with."]
    pub pattern: PrimField<String>,
}

impl BuildOrganizationRulesetRulesElCommitMessagePatternEl {
    pub fn build(self) -> OrganizationRulesetRulesElCommitMessagePatternEl {
        OrganizationRulesetRulesElCommitMessagePatternEl {
            name: core::default::Default::default(),
            negate: core::default::Default::default(),
            operator: self.operator,
            pattern: self.pattern,
        }
    }
}

pub struct OrganizationRulesetRulesElCommitMessagePatternElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OrganizationRulesetRulesElCommitMessagePatternElRef {
    fn new(shared: StackShared, base: String) -> OrganizationRulesetRulesElCommitMessagePatternElRef {
        OrganizationRulesetRulesElCommitMessagePatternElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OrganizationRulesetRulesElCommitMessagePatternElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nHow this rule will appear to users."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `negate` after provisioning.\nIf true, the rule will fail if the pattern matches."]
    pub fn negate(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.negate", self.base))
    }

    #[doc= "Get a reference to the value of field `operator` after provisioning.\nThe operator to use for matching. Can be one of: `starts_with`, `ends_with`, `contains`, `regex`."]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc= "Get a reference to the value of field `pattern` after provisioning.\nThe pattern to match with."]
    pub fn pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }
}

#[derive(Serialize)]
pub struct OrganizationRulesetRulesElCommitterEmailPatternEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    negate: Option<PrimField<bool>>,
    operator: PrimField<String>,
    pattern: PrimField<String>,
}

impl OrganizationRulesetRulesElCommitterEmailPatternEl {
    #[doc= "Set the field `name`.\nHow this rule will appear to users."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `negate`.\nIf true, the rule will fail if the pattern matches."]
    pub fn set_negate(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.negate = Some(v.into());
        self
    }
}

impl ToListMappable for OrganizationRulesetRulesElCommitterEmailPatternEl {
    type O = BlockAssignable<OrganizationRulesetRulesElCommitterEmailPatternEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOrganizationRulesetRulesElCommitterEmailPatternEl {
    #[doc= "The operator to use for matching. Can be one of: `starts_with`, `ends_with`, `contains`, `regex`."]
    pub operator: PrimField<String>,
    #[doc= "The pattern to match with."]
    pub pattern: PrimField<String>,
}

impl BuildOrganizationRulesetRulesElCommitterEmailPatternEl {
    pub fn build(self) -> OrganizationRulesetRulesElCommitterEmailPatternEl {
        OrganizationRulesetRulesElCommitterEmailPatternEl {
            name: core::default::Default::default(),
            negate: core::default::Default::default(),
            operator: self.operator,
            pattern: self.pattern,
        }
    }
}

pub struct OrganizationRulesetRulesElCommitterEmailPatternElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OrganizationRulesetRulesElCommitterEmailPatternElRef {
    fn new(shared: StackShared, base: String) -> OrganizationRulesetRulesElCommitterEmailPatternElRef {
        OrganizationRulesetRulesElCommitterEmailPatternElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OrganizationRulesetRulesElCommitterEmailPatternElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nHow this rule will appear to users."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `negate` after provisioning.\nIf true, the rule will fail if the pattern matches."]
    pub fn negate(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.negate", self.base))
    }

    #[doc= "Get a reference to the value of field `operator` after provisioning.\nThe operator to use for matching. Can be one of: `starts_with`, `ends_with`, `contains`, `regex`."]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc= "Get a reference to the value of field `pattern` after provisioning.\nThe pattern to match with."]
    pub fn pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }
}

#[derive(Serialize)]
pub struct OrganizationRulesetRulesElPullRequestEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dismiss_stale_reviews_on_push: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_code_owner_review: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_last_push_approval: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required_approving_review_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required_review_thread_resolution: Option<PrimField<bool>>,
}

impl OrganizationRulesetRulesElPullRequestEl {
    #[doc= "Set the field `dismiss_stale_reviews_on_push`.\nNew, reviewable commits pushed will dismiss previous pull request review approvals. Defaults to `false`."]
    pub fn set_dismiss_stale_reviews_on_push(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.dismiss_stale_reviews_on_push = Some(v.into());
        self
    }

    #[doc= "Set the field `require_code_owner_review`.\nRequire an approving review in pull requests that modify files that have a designated code owner. Defaults to `false`."]
    pub fn set_require_code_owner_review(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_code_owner_review = Some(v.into());
        self
    }

    #[doc= "Set the field `require_last_push_approval`.\nWhether the most recent reviewable push must be approved by someone other than the person who pushed it. Defaults to `false`."]
    pub fn set_require_last_push_approval(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_last_push_approval = Some(v.into());
        self
    }

    #[doc= "Set the field `required_approving_review_count`.\nThe number of approving reviews that are required before a pull request can be merged. Defaults to `0`."]
    pub fn set_required_approving_review_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.required_approving_review_count = Some(v.into());
        self
    }

    #[doc= "Set the field `required_review_thread_resolution`.\nAll conversations on code must be resolved before a pull request can be merged. Defaults to `false`."]
    pub fn set_required_review_thread_resolution(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.required_review_thread_resolution = Some(v.into());
        self
    }
}

impl ToListMappable for OrganizationRulesetRulesElPullRequestEl {
    type O = BlockAssignable<OrganizationRulesetRulesElPullRequestEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOrganizationRulesetRulesElPullRequestEl {}

impl BuildOrganizationRulesetRulesElPullRequestEl {
    pub fn build(self) -> OrganizationRulesetRulesElPullRequestEl {
        OrganizationRulesetRulesElPullRequestEl {
            dismiss_stale_reviews_on_push: core::default::Default::default(),
            require_code_owner_review: core::default::Default::default(),
            require_last_push_approval: core::default::Default::default(),
            required_approving_review_count: core::default::Default::default(),
            required_review_thread_resolution: core::default::Default::default(),
        }
    }
}

pub struct OrganizationRulesetRulesElPullRequestElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OrganizationRulesetRulesElPullRequestElRef {
    fn new(shared: StackShared, base: String) -> OrganizationRulesetRulesElPullRequestElRef {
        OrganizationRulesetRulesElPullRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OrganizationRulesetRulesElPullRequestElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `dismiss_stale_reviews_on_push` after provisioning.\nNew, reviewable commits pushed will dismiss previous pull request review approvals. Defaults to `false`."]
    pub fn dismiss_stale_reviews_on_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.dismiss_stale_reviews_on_push", self.base))
    }

    #[doc= "Get a reference to the value of field `require_code_owner_review` after provisioning.\nRequire an approving review in pull requests that modify files that have a designated code owner. Defaults to `false`."]
    pub fn require_code_owner_review(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_code_owner_review", self.base))
    }

    #[doc= "Get a reference to the value of field `require_last_push_approval` after provisioning.\nWhether the most recent reviewable push must be approved by someone other than the person who pushed it. Defaults to `false`."]
    pub fn require_last_push_approval(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_last_push_approval", self.base))
    }

    #[doc= "Get a reference to the value of field `required_approving_review_count` after provisioning.\nThe number of approving reviews that are required before a pull request can be merged. Defaults to `0`."]
    pub fn required_approving_review_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.required_approving_review_count", self.base))
    }

    #[doc= "Get a reference to the value of field `required_review_thread_resolution` after provisioning.\nAll conversations on code must be resolved before a pull request can be merged. Defaults to `false`."]
    pub fn required_review_thread_resolution(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.required_review_thread_resolution", self.base))
    }
}

#[derive(Serialize)]
pub struct OrganizationRulesetRulesElRequiredStatusChecksElRequiredCheckEl {
    context: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    integration_id: Option<PrimField<f64>>,
}

impl OrganizationRulesetRulesElRequiredStatusChecksElRequiredCheckEl {
    #[doc= "Set the field `integration_id`.\nThe optional integration ID that this status check must originate from."]
    pub fn set_integration_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.integration_id = Some(v.into());
        self
    }
}

impl ToListMappable for OrganizationRulesetRulesElRequiredStatusChecksElRequiredCheckEl {
    type O = BlockAssignable<OrganizationRulesetRulesElRequiredStatusChecksElRequiredCheckEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOrganizationRulesetRulesElRequiredStatusChecksElRequiredCheckEl {
    #[doc= "The status check context name that must be present on the commit."]
    pub context: PrimField<String>,
}

impl BuildOrganizationRulesetRulesElRequiredStatusChecksElRequiredCheckEl {
    pub fn build(self) -> OrganizationRulesetRulesElRequiredStatusChecksElRequiredCheckEl {
        OrganizationRulesetRulesElRequiredStatusChecksElRequiredCheckEl {
            context: self.context,
            integration_id: core::default::Default::default(),
        }
    }
}

pub struct OrganizationRulesetRulesElRequiredStatusChecksElRequiredCheckElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OrganizationRulesetRulesElRequiredStatusChecksElRequiredCheckElRef {
    fn new(shared: StackShared, base: String) -> OrganizationRulesetRulesElRequiredStatusChecksElRequiredCheckElRef {
        OrganizationRulesetRulesElRequiredStatusChecksElRequiredCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OrganizationRulesetRulesElRequiredStatusChecksElRequiredCheckElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `context` after provisioning.\nThe status check context name that must be present on the commit."]
    pub fn context(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.context", self.base))
    }

    #[doc= "Get a reference to the value of field `integration_id` after provisioning.\nThe optional integration ID that this status check must originate from."]
    pub fn integration_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.integration_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct OrganizationRulesetRulesElRequiredStatusChecksElDynamic {
    required_check: Option<DynamicBlock<OrganizationRulesetRulesElRequiredStatusChecksElRequiredCheckEl>>,
}

#[derive(Serialize)]
pub struct OrganizationRulesetRulesElRequiredStatusChecksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    strict_required_status_checks_policy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required_check: Option<Vec<OrganizationRulesetRulesElRequiredStatusChecksElRequiredCheckEl>>,
    dynamic: OrganizationRulesetRulesElRequiredStatusChecksElDynamic,
}

impl OrganizationRulesetRulesElRequiredStatusChecksEl {
    #[doc= "Set the field `strict_required_status_checks_policy`.\nWhether pull requests targeting a matching branch must be tested with the latest code. This setting will not take effect unless at least one status check is enabled. Defaults to `false`."]
    pub fn set_strict_required_status_checks_policy(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.strict_required_status_checks_policy = Some(v.into());
        self
    }

    #[doc= "Set the field `required_check`.\n"]
    pub fn set_required_check(
        mut self,
        v: impl Into<BlockAssignable<OrganizationRulesetRulesElRequiredStatusChecksElRequiredCheckEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.required_check = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.required_check = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for OrganizationRulesetRulesElRequiredStatusChecksEl {
    type O = BlockAssignable<OrganizationRulesetRulesElRequiredStatusChecksEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOrganizationRulesetRulesElRequiredStatusChecksEl {}

impl BuildOrganizationRulesetRulesElRequiredStatusChecksEl {
    pub fn build(self) -> OrganizationRulesetRulesElRequiredStatusChecksEl {
        OrganizationRulesetRulesElRequiredStatusChecksEl {
            strict_required_status_checks_policy: core::default::Default::default(),
            required_check: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct OrganizationRulesetRulesElRequiredStatusChecksElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OrganizationRulesetRulesElRequiredStatusChecksElRef {
    fn new(shared: StackShared, base: String) -> OrganizationRulesetRulesElRequiredStatusChecksElRef {
        OrganizationRulesetRulesElRequiredStatusChecksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OrganizationRulesetRulesElRequiredStatusChecksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `strict_required_status_checks_policy` after provisioning.\nWhether pull requests targeting a matching branch must be tested with the latest code. This setting will not take effect unless at least one status check is enabled. Defaults to `false`."]
    pub fn strict_required_status_checks_policy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.strict_required_status_checks_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct OrganizationRulesetRulesElTagNamePatternEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    negate: Option<PrimField<bool>>,
    operator: PrimField<String>,
    pattern: PrimField<String>,
}

impl OrganizationRulesetRulesElTagNamePatternEl {
    #[doc= "Set the field `name`.\nHow this rule will appear to users."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `negate`.\nIf true, the rule will fail if the pattern matches."]
    pub fn set_negate(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.negate = Some(v.into());
        self
    }
}

impl ToListMappable for OrganizationRulesetRulesElTagNamePatternEl {
    type O = BlockAssignable<OrganizationRulesetRulesElTagNamePatternEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOrganizationRulesetRulesElTagNamePatternEl {
    #[doc= "The operator to use for matching. Can be one of: `starts_with`, `ends_with`, `contains`, `regex`."]
    pub operator: PrimField<String>,
    #[doc= "The pattern to match with."]
    pub pattern: PrimField<String>,
}

impl BuildOrganizationRulesetRulesElTagNamePatternEl {
    pub fn build(self) -> OrganizationRulesetRulesElTagNamePatternEl {
        OrganizationRulesetRulesElTagNamePatternEl {
            name: core::default::Default::default(),
            negate: core::default::Default::default(),
            operator: self.operator,
            pattern: self.pattern,
        }
    }
}

pub struct OrganizationRulesetRulesElTagNamePatternElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OrganizationRulesetRulesElTagNamePatternElRef {
    fn new(shared: StackShared, base: String) -> OrganizationRulesetRulesElTagNamePatternElRef {
        OrganizationRulesetRulesElTagNamePatternElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OrganizationRulesetRulesElTagNamePatternElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nHow this rule will appear to users."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `negate` after provisioning.\nIf true, the rule will fail if the pattern matches."]
    pub fn negate(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.negate", self.base))
    }

    #[doc= "Get a reference to the value of field `operator` after provisioning.\nThe operator to use for matching. Can be one of: `starts_with`, `ends_with`, `contains`, `regex`."]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc= "Get a reference to the value of field `pattern` after provisioning.\nThe pattern to match with."]
    pub fn pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }
}

#[derive(Serialize, Default)]
struct OrganizationRulesetRulesElDynamic {
    branch_name_pattern: Option<DynamicBlock<OrganizationRulesetRulesElBranchNamePatternEl>>,
    commit_author_email_pattern: Option<DynamicBlock<OrganizationRulesetRulesElCommitAuthorEmailPatternEl>>,
    commit_message_pattern: Option<DynamicBlock<OrganizationRulesetRulesElCommitMessagePatternEl>>,
    committer_email_pattern: Option<DynamicBlock<OrganizationRulesetRulesElCommitterEmailPatternEl>>,
    pull_request: Option<DynamicBlock<OrganizationRulesetRulesElPullRequestEl>>,
    required_status_checks: Option<DynamicBlock<OrganizationRulesetRulesElRequiredStatusChecksEl>>,
    tag_name_pattern: Option<DynamicBlock<OrganizationRulesetRulesElTagNamePatternEl>>,
}

#[derive(Serialize)]
pub struct OrganizationRulesetRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    creation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    non_fast_forward: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required_linear_history: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required_signatures: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branch_name_pattern: Option<Vec<OrganizationRulesetRulesElBranchNamePatternEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_author_email_pattern: Option<Vec<OrganizationRulesetRulesElCommitAuthorEmailPatternEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_message_pattern: Option<Vec<OrganizationRulesetRulesElCommitMessagePatternEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    committer_email_pattern: Option<Vec<OrganizationRulesetRulesElCommitterEmailPatternEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pull_request: Option<Vec<OrganizationRulesetRulesElPullRequestEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required_status_checks: Option<Vec<OrganizationRulesetRulesElRequiredStatusChecksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_name_pattern: Option<Vec<OrganizationRulesetRulesElTagNamePatternEl>>,
    dynamic: OrganizationRulesetRulesElDynamic,
}

impl OrganizationRulesetRulesEl {
    #[doc= "Set the field `creation`.\nOnly allow users with bypass permission to create matching refs."]
    pub fn set_creation(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.creation = Some(v.into());
        self
    }

    #[doc= "Set the field `deletion`.\nOnly allow users with bypass permissions to delete matching refs."]
    pub fn set_deletion(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deletion = Some(v.into());
        self
    }

    #[doc= "Set the field `non_fast_forward`.\nPrevent users with push access from force pushing to branches."]
    pub fn set_non_fast_forward(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.non_fast_forward = Some(v.into());
        self
    }

    #[doc= "Set the field `required_linear_history`.\nPrevent merge commits from being pushed to matching branches."]
    pub fn set_required_linear_history(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.required_linear_history = Some(v.into());
        self
    }

    #[doc= "Set the field `required_signatures`.\nCommits pushed to matching branches must have verified signatures."]
    pub fn set_required_signatures(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.required_signatures = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\nOnly allow users with bypass permission to update matching refs."]
    pub fn set_update(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.update = Some(v.into());
        self
    }

    #[doc= "Set the field `branch_name_pattern`.\n"]
    pub fn set_branch_name_pattern(
        mut self,
        v: impl Into<BlockAssignable<OrganizationRulesetRulesElBranchNamePatternEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.branch_name_pattern = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.branch_name_pattern = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `commit_author_email_pattern`.\n"]
    pub fn set_commit_author_email_pattern(
        mut self,
        v: impl Into<BlockAssignable<OrganizationRulesetRulesElCommitAuthorEmailPatternEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.commit_author_email_pattern = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.commit_author_email_pattern = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `commit_message_pattern`.\n"]
    pub fn set_commit_message_pattern(
        mut self,
        v: impl Into<BlockAssignable<OrganizationRulesetRulesElCommitMessagePatternEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.commit_message_pattern = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.commit_message_pattern = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `committer_email_pattern`.\n"]
    pub fn set_committer_email_pattern(
        mut self,
        v: impl Into<BlockAssignable<OrganizationRulesetRulesElCommitterEmailPatternEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.committer_email_pattern = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.committer_email_pattern = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `pull_request`.\n"]
    pub fn set_pull_request(mut self, v: impl Into<BlockAssignable<OrganizationRulesetRulesElPullRequestEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pull_request = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pull_request = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `required_status_checks`.\n"]
    pub fn set_required_status_checks(
        mut self,
        v: impl Into<BlockAssignable<OrganizationRulesetRulesElRequiredStatusChecksEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.required_status_checks = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.required_status_checks = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `tag_name_pattern`.\n"]
    pub fn set_tag_name_pattern(
        mut self,
        v: impl Into<BlockAssignable<OrganizationRulesetRulesElTagNamePatternEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tag_name_pattern = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_name_pattern = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for OrganizationRulesetRulesEl {
    type O = BlockAssignable<OrganizationRulesetRulesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOrganizationRulesetRulesEl {}

impl BuildOrganizationRulesetRulesEl {
    pub fn build(self) -> OrganizationRulesetRulesEl {
        OrganizationRulesetRulesEl {
            creation: core::default::Default::default(),
            deletion: core::default::Default::default(),
            non_fast_forward: core::default::Default::default(),
            required_linear_history: core::default::Default::default(),
            required_signatures: core::default::Default::default(),
            update: core::default::Default::default(),
            branch_name_pattern: core::default::Default::default(),
            commit_author_email_pattern: core::default::Default::default(),
            commit_message_pattern: core::default::Default::default(),
            committer_email_pattern: core::default::Default::default(),
            pull_request: core::default::Default::default(),
            required_status_checks: core::default::Default::default(),
            tag_name_pattern: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct OrganizationRulesetRulesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OrganizationRulesetRulesElRef {
    fn new(shared: StackShared, base: String) -> OrganizationRulesetRulesElRef {
        OrganizationRulesetRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OrganizationRulesetRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `creation` after provisioning.\nOnly allow users with bypass permission to create matching refs."]
    pub fn creation(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation", self.base))
    }

    #[doc= "Get a reference to the value of field `deletion` after provisioning.\nOnly allow users with bypass permissions to delete matching refs."]
    pub fn deletion(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion", self.base))
    }

    #[doc= "Get a reference to the value of field `non_fast_forward` after provisioning.\nPrevent users with push access from force pushing to branches."]
    pub fn non_fast_forward(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.non_fast_forward", self.base))
    }

    #[doc= "Get a reference to the value of field `required_linear_history` after provisioning.\nPrevent merge commits from being pushed to matching branches."]
    pub fn required_linear_history(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.required_linear_history", self.base))
    }

    #[doc= "Get a reference to the value of field `required_signatures` after provisioning.\nCommits pushed to matching branches must have verified signatures."]
    pub fn required_signatures(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.required_signatures", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\nOnly allow users with bypass permission to update matching refs."]
    pub fn update(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }

    #[doc= "Get a reference to the value of field `branch_name_pattern` after provisioning.\n"]
    pub fn branch_name_pattern(&self) -> ListRef<OrganizationRulesetRulesElBranchNamePatternElRef> {
        ListRef::new(self.shared().clone(), format!("{}.branch_name_pattern", self.base))
    }

    #[doc= "Get a reference to the value of field `commit_author_email_pattern` after provisioning.\n"]
    pub fn commit_author_email_pattern(&self) -> ListRef<OrganizationRulesetRulesElCommitAuthorEmailPatternElRef> {
        ListRef::new(self.shared().clone(), format!("{}.commit_author_email_pattern", self.base))
    }

    #[doc= "Get a reference to the value of field `commit_message_pattern` after provisioning.\n"]
    pub fn commit_message_pattern(&self) -> ListRef<OrganizationRulesetRulesElCommitMessagePatternElRef> {
        ListRef::new(self.shared().clone(), format!("{}.commit_message_pattern", self.base))
    }

    #[doc= "Get a reference to the value of field `committer_email_pattern` after provisioning.\n"]
    pub fn committer_email_pattern(&self) -> ListRef<OrganizationRulesetRulesElCommitterEmailPatternElRef> {
        ListRef::new(self.shared().clone(), format!("{}.committer_email_pattern", self.base))
    }

    #[doc= "Get a reference to the value of field `pull_request` after provisioning.\n"]
    pub fn pull_request(&self) -> ListRef<OrganizationRulesetRulesElPullRequestElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pull_request", self.base))
    }

    #[doc= "Get a reference to the value of field `required_status_checks` after provisioning.\n"]
    pub fn required_status_checks(&self) -> ListRef<OrganizationRulesetRulesElRequiredStatusChecksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.required_status_checks", self.base))
    }

    #[doc= "Get a reference to the value of field `tag_name_pattern` after provisioning.\n"]
    pub fn tag_name_pattern(&self) -> ListRef<OrganizationRulesetRulesElTagNamePatternElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tag_name_pattern", self.base))
    }
}

#[derive(Serialize, Default)]
struct OrganizationRulesetDynamic {
    bypass_actors: Option<DynamicBlock<OrganizationRulesetBypassActorsEl>>,
    conditions: Option<DynamicBlock<OrganizationRulesetConditionsEl>>,
    rules: Option<DynamicBlock<OrganizationRulesetRulesEl>>,
}
