use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGithub;

#[derive(Serialize)]
struct RepositoryRulesetData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    repository: Option<PrimField<String>>,
    target: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bypass_actors: Option<Vec<RepositoryRulesetBypassActorsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conditions: Option<Vec<RepositoryRulesetConditionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules: Option<Vec<RepositoryRulesetRulesEl>>,
    dynamic: RepositoryRulesetDynamic,
}

struct RepositoryRuleset_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RepositoryRulesetData>,
}

#[derive(Clone)]
pub struct RepositoryRuleset(Rc<RepositoryRuleset_>);

impl RepositoryRuleset {
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

    #[doc= "Set the field `repository`.\nName of the repository to apply rulset to."]
    pub fn set_repository(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().repository = Some(v.into());
        self
    }

    #[doc= "Set the field `bypass_actors`.\n"]
    pub fn set_bypass_actors(self, v: impl Into<BlockAssignable<RepositoryRulesetBypassActorsEl>>) -> Self {
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
    pub fn set_conditions(self, v: impl Into<BlockAssignable<RepositoryRulesetConditionsEl>>) -> Self {
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
    pub fn set_rules(self, v: impl Into<BlockAssignable<RepositoryRulesetRulesEl>>) -> Self {
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

    #[doc= "Get a reference to the value of field `repository` after provisioning.\nName of the repository to apply rulset to."]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
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
    pub fn bypass_actors(&self) -> ListRef<RepositoryRulesetBypassActorsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bypass_actors", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> ListRef<RepositoryRulesetConditionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.conditions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<RepositoryRulesetRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }
}

impl Referable for RepositoryRuleset {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for RepositoryRuleset { }

impl ToListMappable for RepositoryRuleset {
    type O = ListRef<RepositoryRulesetRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for RepositoryRuleset_ {
    fn extract_resource_type(&self) -> String {
        "github_repository_ruleset".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRepositoryRuleset {
    pub tf_id: String,
    #[doc= "Possible values for Enforcement are `disabled`, `active`, `evaluate`. Note: `evaluate` is currently only supported for owners of type `organization`."]
    pub enforcement: PrimField<String>,
    #[doc= "The name of the ruleset."]
    pub name: PrimField<String>,
    #[doc= "Possible values are `branch` and `tag`."]
    pub target: PrimField<String>,
}

impl BuildRepositoryRuleset {
    pub fn build(self, stack: &mut Stack) -> RepositoryRuleset {
        let out = RepositoryRuleset(Rc::new(RepositoryRuleset_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RepositoryRulesetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                enforcement: self.enforcement,
                id: core::default::Default::default(),
                name: self.name,
                repository: core::default::Default::default(),
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

pub struct RepositoryRulesetRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositoryRulesetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RepositoryRulesetRef {
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

    #[doc= "Get a reference to the value of field `repository` after provisioning.\nName of the repository to apply rulset to."]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
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
    pub fn bypass_actors(&self) -> ListRef<RepositoryRulesetBypassActorsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bypass_actors", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> ListRef<RepositoryRulesetConditionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.conditions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<RepositoryRulesetRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct RepositoryRulesetBypassActorsEl {
    actor_id: PrimField<f64>,
    actor_type: PrimField<String>,
    bypass_mode: PrimField<String>,
}

impl RepositoryRulesetBypassActorsEl { }

impl ToListMappable for RepositoryRulesetBypassActorsEl {
    type O = BlockAssignable<RepositoryRulesetBypassActorsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRepositoryRulesetBypassActorsEl {
    #[doc= "The ID of the actor that can bypass a ruleset. When `actor_type` is `OrganizationAdmin`, this should be set to `1`."]
    pub actor_id: PrimField<f64>,
    #[doc= "The type of actor that can bypass a ruleset. Can be one of: `RepositoryRole`, `Team`, `Integration`, `OrganizationAdmin`."]
    pub actor_type: PrimField<String>,
    #[doc= "When the specified actor can bypass the ruleset. pull_request means that an actor can only bypass rules on pull requests. Can be one of: `always`, `pull_request`."]
    pub bypass_mode: PrimField<String>,
}

impl BuildRepositoryRulesetBypassActorsEl {
    pub fn build(self) -> RepositoryRulesetBypassActorsEl {
        RepositoryRulesetBypassActorsEl {
            actor_id: self.actor_id,
            actor_type: self.actor_type,
            bypass_mode: self.bypass_mode,
        }
    }
}

pub struct RepositoryRulesetBypassActorsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositoryRulesetBypassActorsElRef {
    fn new(shared: StackShared, base: String) -> RepositoryRulesetBypassActorsElRef {
        RepositoryRulesetBypassActorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RepositoryRulesetBypassActorsElRef {
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
pub struct RepositoryRulesetConditionsElRefNameEl {
    exclude: ListField<PrimField<String>>,
    include: ListField<PrimField<String>>,
}

impl RepositoryRulesetConditionsElRefNameEl { }

impl ToListMappable for RepositoryRulesetConditionsElRefNameEl {
    type O = BlockAssignable<RepositoryRulesetConditionsElRefNameEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRepositoryRulesetConditionsElRefNameEl {
    #[doc= "Array of ref names or patterns to exclude. The condition will not pass if any of these patterns match."]
    pub exclude: ListField<PrimField<String>>,
    #[doc= "Array of ref names or patterns to include. One of these patterns must match for the condition to pass. Also accepts `~DEFAULT_BRANCH` to include the default branch or `~ALL` to include all branches."]
    pub include: ListField<PrimField<String>>,
}

impl BuildRepositoryRulesetConditionsElRefNameEl {
    pub fn build(self) -> RepositoryRulesetConditionsElRefNameEl {
        RepositoryRulesetConditionsElRefNameEl {
            exclude: self.exclude,
            include: self.include,
        }
    }
}

pub struct RepositoryRulesetConditionsElRefNameElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositoryRulesetConditionsElRefNameElRef {
    fn new(shared: StackShared, base: String) -> RepositoryRulesetConditionsElRefNameElRef {
        RepositoryRulesetConditionsElRefNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RepositoryRulesetConditionsElRefNameElRef {
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

#[derive(Serialize, Default)]
struct RepositoryRulesetConditionsElDynamic {
    ref_name: Option<DynamicBlock<RepositoryRulesetConditionsElRefNameEl>>,
}

#[derive(Serialize)]
pub struct RepositoryRulesetConditionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ref_name: Option<Vec<RepositoryRulesetConditionsElRefNameEl>>,
    dynamic: RepositoryRulesetConditionsElDynamic,
}

impl RepositoryRulesetConditionsEl {
    #[doc= "Set the field `ref_name`.\n"]
    pub fn set_ref_name(mut self, v: impl Into<BlockAssignable<RepositoryRulesetConditionsElRefNameEl>>) -> Self {
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
}

impl ToListMappable for RepositoryRulesetConditionsEl {
    type O = BlockAssignable<RepositoryRulesetConditionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRepositoryRulesetConditionsEl {}

impl BuildRepositoryRulesetConditionsEl {
    pub fn build(self) -> RepositoryRulesetConditionsEl {
        RepositoryRulesetConditionsEl {
            ref_name: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct RepositoryRulesetConditionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositoryRulesetConditionsElRef {
    fn new(shared: StackShared, base: String) -> RepositoryRulesetConditionsElRef {
        RepositoryRulesetConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RepositoryRulesetConditionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `ref_name` after provisioning.\n"]
    pub fn ref_name(&self) -> ListRef<RepositoryRulesetConditionsElRefNameElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ref_name", self.base))
    }
}

#[derive(Serialize)]
pub struct RepositoryRulesetRulesElBranchNamePatternEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    negate: Option<PrimField<bool>>,
    operator: PrimField<String>,
    pattern: PrimField<String>,
}

impl RepositoryRulesetRulesElBranchNamePatternEl {
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

impl ToListMappable for RepositoryRulesetRulesElBranchNamePatternEl {
    type O = BlockAssignable<RepositoryRulesetRulesElBranchNamePatternEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRepositoryRulesetRulesElBranchNamePatternEl {
    #[doc= "The operator to use for matching. Can be one of: `starts_with`, `ends_with`, `contains`, `regex`."]
    pub operator: PrimField<String>,
    #[doc= "The pattern to match with."]
    pub pattern: PrimField<String>,
}

impl BuildRepositoryRulesetRulesElBranchNamePatternEl {
    pub fn build(self) -> RepositoryRulesetRulesElBranchNamePatternEl {
        RepositoryRulesetRulesElBranchNamePatternEl {
            name: core::default::Default::default(),
            negate: core::default::Default::default(),
            operator: self.operator,
            pattern: self.pattern,
        }
    }
}

pub struct RepositoryRulesetRulesElBranchNamePatternElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositoryRulesetRulesElBranchNamePatternElRef {
    fn new(shared: StackShared, base: String) -> RepositoryRulesetRulesElBranchNamePatternElRef {
        RepositoryRulesetRulesElBranchNamePatternElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RepositoryRulesetRulesElBranchNamePatternElRef {
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
pub struct RepositoryRulesetRulesElCommitAuthorEmailPatternEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    negate: Option<PrimField<bool>>,
    operator: PrimField<String>,
    pattern: PrimField<String>,
}

impl RepositoryRulesetRulesElCommitAuthorEmailPatternEl {
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

impl ToListMappable for RepositoryRulesetRulesElCommitAuthorEmailPatternEl {
    type O = BlockAssignable<RepositoryRulesetRulesElCommitAuthorEmailPatternEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRepositoryRulesetRulesElCommitAuthorEmailPatternEl {
    #[doc= "The operator to use for matching. Can be one of: `starts_with`, `ends_with`, `contains`, `regex`."]
    pub operator: PrimField<String>,
    #[doc= "The pattern to match with."]
    pub pattern: PrimField<String>,
}

impl BuildRepositoryRulesetRulesElCommitAuthorEmailPatternEl {
    pub fn build(self) -> RepositoryRulesetRulesElCommitAuthorEmailPatternEl {
        RepositoryRulesetRulesElCommitAuthorEmailPatternEl {
            name: core::default::Default::default(),
            negate: core::default::Default::default(),
            operator: self.operator,
            pattern: self.pattern,
        }
    }
}

pub struct RepositoryRulesetRulesElCommitAuthorEmailPatternElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositoryRulesetRulesElCommitAuthorEmailPatternElRef {
    fn new(shared: StackShared, base: String) -> RepositoryRulesetRulesElCommitAuthorEmailPatternElRef {
        RepositoryRulesetRulesElCommitAuthorEmailPatternElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RepositoryRulesetRulesElCommitAuthorEmailPatternElRef {
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
pub struct RepositoryRulesetRulesElCommitMessagePatternEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    negate: Option<PrimField<bool>>,
    operator: PrimField<String>,
    pattern: PrimField<String>,
}

impl RepositoryRulesetRulesElCommitMessagePatternEl {
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

impl ToListMappable for RepositoryRulesetRulesElCommitMessagePatternEl {
    type O = BlockAssignable<RepositoryRulesetRulesElCommitMessagePatternEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRepositoryRulesetRulesElCommitMessagePatternEl {
    #[doc= "The operator to use for matching. Can be one of: `starts_with`, `ends_with`, `contains`, `regex`."]
    pub operator: PrimField<String>,
    #[doc= "The pattern to match with."]
    pub pattern: PrimField<String>,
}

impl BuildRepositoryRulesetRulesElCommitMessagePatternEl {
    pub fn build(self) -> RepositoryRulesetRulesElCommitMessagePatternEl {
        RepositoryRulesetRulesElCommitMessagePatternEl {
            name: core::default::Default::default(),
            negate: core::default::Default::default(),
            operator: self.operator,
            pattern: self.pattern,
        }
    }
}

pub struct RepositoryRulesetRulesElCommitMessagePatternElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositoryRulesetRulesElCommitMessagePatternElRef {
    fn new(shared: StackShared, base: String) -> RepositoryRulesetRulesElCommitMessagePatternElRef {
        RepositoryRulesetRulesElCommitMessagePatternElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RepositoryRulesetRulesElCommitMessagePatternElRef {
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
pub struct RepositoryRulesetRulesElCommitterEmailPatternEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    negate: Option<PrimField<bool>>,
    operator: PrimField<String>,
    pattern: PrimField<String>,
}

impl RepositoryRulesetRulesElCommitterEmailPatternEl {
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

impl ToListMappable for RepositoryRulesetRulesElCommitterEmailPatternEl {
    type O = BlockAssignable<RepositoryRulesetRulesElCommitterEmailPatternEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRepositoryRulesetRulesElCommitterEmailPatternEl {
    #[doc= "The operator to use for matching. Can be one of: `starts_with`, `ends_with`, `contains`, `regex`."]
    pub operator: PrimField<String>,
    #[doc= "The pattern to match with."]
    pub pattern: PrimField<String>,
}

impl BuildRepositoryRulesetRulesElCommitterEmailPatternEl {
    pub fn build(self) -> RepositoryRulesetRulesElCommitterEmailPatternEl {
        RepositoryRulesetRulesElCommitterEmailPatternEl {
            name: core::default::Default::default(),
            negate: core::default::Default::default(),
            operator: self.operator,
            pattern: self.pattern,
        }
    }
}

pub struct RepositoryRulesetRulesElCommitterEmailPatternElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositoryRulesetRulesElCommitterEmailPatternElRef {
    fn new(shared: StackShared, base: String) -> RepositoryRulesetRulesElCommitterEmailPatternElRef {
        RepositoryRulesetRulesElCommitterEmailPatternElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RepositoryRulesetRulesElCommitterEmailPatternElRef {
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
pub struct RepositoryRulesetRulesElPullRequestEl {
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

impl RepositoryRulesetRulesElPullRequestEl {
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

impl ToListMappable for RepositoryRulesetRulesElPullRequestEl {
    type O = BlockAssignable<RepositoryRulesetRulesElPullRequestEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRepositoryRulesetRulesElPullRequestEl {}

impl BuildRepositoryRulesetRulesElPullRequestEl {
    pub fn build(self) -> RepositoryRulesetRulesElPullRequestEl {
        RepositoryRulesetRulesElPullRequestEl {
            dismiss_stale_reviews_on_push: core::default::Default::default(),
            require_code_owner_review: core::default::Default::default(),
            require_last_push_approval: core::default::Default::default(),
            required_approving_review_count: core::default::Default::default(),
            required_review_thread_resolution: core::default::Default::default(),
        }
    }
}

pub struct RepositoryRulesetRulesElPullRequestElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositoryRulesetRulesElPullRequestElRef {
    fn new(shared: StackShared, base: String) -> RepositoryRulesetRulesElPullRequestElRef {
        RepositoryRulesetRulesElPullRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RepositoryRulesetRulesElPullRequestElRef {
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
pub struct RepositoryRulesetRulesElRequiredDeploymentsEl {
    required_deployment_environments: ListField<PrimField<String>>,
}

impl RepositoryRulesetRulesElRequiredDeploymentsEl { }

impl ToListMappable for RepositoryRulesetRulesElRequiredDeploymentsEl {
    type O = BlockAssignable<RepositoryRulesetRulesElRequiredDeploymentsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRepositoryRulesetRulesElRequiredDeploymentsEl {
    #[doc= "The environments that must be successfully deployed to before branches can be merged."]
    pub required_deployment_environments: ListField<PrimField<String>>,
}

impl BuildRepositoryRulesetRulesElRequiredDeploymentsEl {
    pub fn build(self) -> RepositoryRulesetRulesElRequiredDeploymentsEl {
        RepositoryRulesetRulesElRequiredDeploymentsEl {
            required_deployment_environments: self.required_deployment_environments,
        }
    }
}

pub struct RepositoryRulesetRulesElRequiredDeploymentsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositoryRulesetRulesElRequiredDeploymentsElRef {
    fn new(shared: StackShared, base: String) -> RepositoryRulesetRulesElRequiredDeploymentsElRef {
        RepositoryRulesetRulesElRequiredDeploymentsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RepositoryRulesetRulesElRequiredDeploymentsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `required_deployment_environments` after provisioning.\nThe environments that must be successfully deployed to before branches can be merged."]
    pub fn required_deployment_environments(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.required_deployment_environments", self.base))
    }
}

#[derive(Serialize)]
pub struct RepositoryRulesetRulesElRequiredStatusChecksElRequiredCheckEl {
    context: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    integration_id: Option<PrimField<f64>>,
}

impl RepositoryRulesetRulesElRequiredStatusChecksElRequiredCheckEl {
    #[doc= "Set the field `integration_id`.\nThe optional integration ID that this status check must originate from."]
    pub fn set_integration_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.integration_id = Some(v.into());
        self
    }
}

impl ToListMappable for RepositoryRulesetRulesElRequiredStatusChecksElRequiredCheckEl {
    type O = BlockAssignable<RepositoryRulesetRulesElRequiredStatusChecksElRequiredCheckEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRepositoryRulesetRulesElRequiredStatusChecksElRequiredCheckEl {
    #[doc= "The status check context name that must be present on the commit."]
    pub context: PrimField<String>,
}

impl BuildRepositoryRulesetRulesElRequiredStatusChecksElRequiredCheckEl {
    pub fn build(self) -> RepositoryRulesetRulesElRequiredStatusChecksElRequiredCheckEl {
        RepositoryRulesetRulesElRequiredStatusChecksElRequiredCheckEl {
            context: self.context,
            integration_id: core::default::Default::default(),
        }
    }
}

pub struct RepositoryRulesetRulesElRequiredStatusChecksElRequiredCheckElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositoryRulesetRulesElRequiredStatusChecksElRequiredCheckElRef {
    fn new(shared: StackShared, base: String) -> RepositoryRulesetRulesElRequiredStatusChecksElRequiredCheckElRef {
        RepositoryRulesetRulesElRequiredStatusChecksElRequiredCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RepositoryRulesetRulesElRequiredStatusChecksElRequiredCheckElRef {
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
struct RepositoryRulesetRulesElRequiredStatusChecksElDynamic {
    required_check: Option<DynamicBlock<RepositoryRulesetRulesElRequiredStatusChecksElRequiredCheckEl>>,
}

#[derive(Serialize)]
pub struct RepositoryRulesetRulesElRequiredStatusChecksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    strict_required_status_checks_policy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required_check: Option<Vec<RepositoryRulesetRulesElRequiredStatusChecksElRequiredCheckEl>>,
    dynamic: RepositoryRulesetRulesElRequiredStatusChecksElDynamic,
}

impl RepositoryRulesetRulesElRequiredStatusChecksEl {
    #[doc= "Set the field `strict_required_status_checks_policy`.\nWhether pull requests targeting a matching branch must be tested with the latest code. This setting will not take effect unless at least one status check is enabled. Defaults to `false`."]
    pub fn set_strict_required_status_checks_policy(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.strict_required_status_checks_policy = Some(v.into());
        self
    }

    #[doc= "Set the field `required_check`.\n"]
    pub fn set_required_check(
        mut self,
        v: impl Into<BlockAssignable<RepositoryRulesetRulesElRequiredStatusChecksElRequiredCheckEl>>,
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

impl ToListMappable for RepositoryRulesetRulesElRequiredStatusChecksEl {
    type O = BlockAssignable<RepositoryRulesetRulesElRequiredStatusChecksEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRepositoryRulesetRulesElRequiredStatusChecksEl {}

impl BuildRepositoryRulesetRulesElRequiredStatusChecksEl {
    pub fn build(self) -> RepositoryRulesetRulesElRequiredStatusChecksEl {
        RepositoryRulesetRulesElRequiredStatusChecksEl {
            strict_required_status_checks_policy: core::default::Default::default(),
            required_check: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct RepositoryRulesetRulesElRequiredStatusChecksElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositoryRulesetRulesElRequiredStatusChecksElRef {
    fn new(shared: StackShared, base: String) -> RepositoryRulesetRulesElRequiredStatusChecksElRef {
        RepositoryRulesetRulesElRequiredStatusChecksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RepositoryRulesetRulesElRequiredStatusChecksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `strict_required_status_checks_policy` after provisioning.\nWhether pull requests targeting a matching branch must be tested with the latest code. This setting will not take effect unless at least one status check is enabled. Defaults to `false`."]
    pub fn strict_required_status_checks_policy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.strict_required_status_checks_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct RepositoryRulesetRulesElTagNamePatternEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    negate: Option<PrimField<bool>>,
    operator: PrimField<String>,
    pattern: PrimField<String>,
}

impl RepositoryRulesetRulesElTagNamePatternEl {
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

impl ToListMappable for RepositoryRulesetRulesElTagNamePatternEl {
    type O = BlockAssignable<RepositoryRulesetRulesElTagNamePatternEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRepositoryRulesetRulesElTagNamePatternEl {
    #[doc= "The operator to use for matching. Can be one of: `starts_with`, `ends_with`, `contains`, `regex`."]
    pub operator: PrimField<String>,
    #[doc= "The pattern to match with."]
    pub pattern: PrimField<String>,
}

impl BuildRepositoryRulesetRulesElTagNamePatternEl {
    pub fn build(self) -> RepositoryRulesetRulesElTagNamePatternEl {
        RepositoryRulesetRulesElTagNamePatternEl {
            name: core::default::Default::default(),
            negate: core::default::Default::default(),
            operator: self.operator,
            pattern: self.pattern,
        }
    }
}

pub struct RepositoryRulesetRulesElTagNamePatternElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositoryRulesetRulesElTagNamePatternElRef {
    fn new(shared: StackShared, base: String) -> RepositoryRulesetRulesElTagNamePatternElRef {
        RepositoryRulesetRulesElTagNamePatternElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RepositoryRulesetRulesElTagNamePatternElRef {
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
struct RepositoryRulesetRulesElDynamic {
    branch_name_pattern: Option<DynamicBlock<RepositoryRulesetRulesElBranchNamePatternEl>>,
    commit_author_email_pattern: Option<DynamicBlock<RepositoryRulesetRulesElCommitAuthorEmailPatternEl>>,
    commit_message_pattern: Option<DynamicBlock<RepositoryRulesetRulesElCommitMessagePatternEl>>,
    committer_email_pattern: Option<DynamicBlock<RepositoryRulesetRulesElCommitterEmailPatternEl>>,
    pull_request: Option<DynamicBlock<RepositoryRulesetRulesElPullRequestEl>>,
    required_deployments: Option<DynamicBlock<RepositoryRulesetRulesElRequiredDeploymentsEl>>,
    required_status_checks: Option<DynamicBlock<RepositoryRulesetRulesElRequiredStatusChecksEl>>,
    tag_name_pattern: Option<DynamicBlock<RepositoryRulesetRulesElTagNamePatternEl>>,
}

#[derive(Serialize)]
pub struct RepositoryRulesetRulesEl {
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
    update_allows_fetch_and_merge: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branch_name_pattern: Option<Vec<RepositoryRulesetRulesElBranchNamePatternEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_author_email_pattern: Option<Vec<RepositoryRulesetRulesElCommitAuthorEmailPatternEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_message_pattern: Option<Vec<RepositoryRulesetRulesElCommitMessagePatternEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    committer_email_pattern: Option<Vec<RepositoryRulesetRulesElCommitterEmailPatternEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pull_request: Option<Vec<RepositoryRulesetRulesElPullRequestEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required_deployments: Option<Vec<RepositoryRulesetRulesElRequiredDeploymentsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required_status_checks: Option<Vec<RepositoryRulesetRulesElRequiredStatusChecksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_name_pattern: Option<Vec<RepositoryRulesetRulesElTagNamePatternEl>>,
    dynamic: RepositoryRulesetRulesElDynamic,
}

impl RepositoryRulesetRulesEl {
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

    #[doc= "Set the field `update_allows_fetch_and_merge`.\nBranch can pull changes from its upstream repository. This is only applicable to forked repositories. Requires `update` to be set to `true`."]
    pub fn set_update_allows_fetch_and_merge(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.update_allows_fetch_and_merge = Some(v.into());
        self
    }

    #[doc= "Set the field `branch_name_pattern`.\n"]
    pub fn set_branch_name_pattern(
        mut self,
        v: impl Into<BlockAssignable<RepositoryRulesetRulesElBranchNamePatternEl>>,
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
        v: impl Into<BlockAssignable<RepositoryRulesetRulesElCommitAuthorEmailPatternEl>>,
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
        v: impl Into<BlockAssignable<RepositoryRulesetRulesElCommitMessagePatternEl>>,
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
        v: impl Into<BlockAssignable<RepositoryRulesetRulesElCommitterEmailPatternEl>>,
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
    pub fn set_pull_request(mut self, v: impl Into<BlockAssignable<RepositoryRulesetRulesElPullRequestEl>>) -> Self {
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

    #[doc= "Set the field `required_deployments`.\n"]
    pub fn set_required_deployments(
        mut self,
        v: impl Into<BlockAssignable<RepositoryRulesetRulesElRequiredDeploymentsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.required_deployments = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.required_deployments = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `required_status_checks`.\n"]
    pub fn set_required_status_checks(
        mut self,
        v: impl Into<BlockAssignable<RepositoryRulesetRulesElRequiredStatusChecksEl>>,
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
        v: impl Into<BlockAssignable<RepositoryRulesetRulesElTagNamePatternEl>>,
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

impl ToListMappable for RepositoryRulesetRulesEl {
    type O = BlockAssignable<RepositoryRulesetRulesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRepositoryRulesetRulesEl {}

impl BuildRepositoryRulesetRulesEl {
    pub fn build(self) -> RepositoryRulesetRulesEl {
        RepositoryRulesetRulesEl {
            creation: core::default::Default::default(),
            deletion: core::default::Default::default(),
            non_fast_forward: core::default::Default::default(),
            required_linear_history: core::default::Default::default(),
            required_signatures: core::default::Default::default(),
            update: core::default::Default::default(),
            update_allows_fetch_and_merge: core::default::Default::default(),
            branch_name_pattern: core::default::Default::default(),
            commit_author_email_pattern: core::default::Default::default(),
            commit_message_pattern: core::default::Default::default(),
            committer_email_pattern: core::default::Default::default(),
            pull_request: core::default::Default::default(),
            required_deployments: core::default::Default::default(),
            required_status_checks: core::default::Default::default(),
            tag_name_pattern: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct RepositoryRulesetRulesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RepositoryRulesetRulesElRef {
    fn new(shared: StackShared, base: String) -> RepositoryRulesetRulesElRef {
        RepositoryRulesetRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RepositoryRulesetRulesElRef {
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

    #[doc= "Get a reference to the value of field `update_allows_fetch_and_merge` after provisioning.\nBranch can pull changes from its upstream repository. This is only applicable to forked repositories. Requires `update` to be set to `true`."]
    pub fn update_allows_fetch_and_merge(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_allows_fetch_and_merge", self.base))
    }

    #[doc= "Get a reference to the value of field `branch_name_pattern` after provisioning.\n"]
    pub fn branch_name_pattern(&self) -> ListRef<RepositoryRulesetRulesElBranchNamePatternElRef> {
        ListRef::new(self.shared().clone(), format!("{}.branch_name_pattern", self.base))
    }

    #[doc= "Get a reference to the value of field `commit_author_email_pattern` after provisioning.\n"]
    pub fn commit_author_email_pattern(&self) -> ListRef<RepositoryRulesetRulesElCommitAuthorEmailPatternElRef> {
        ListRef::new(self.shared().clone(), format!("{}.commit_author_email_pattern", self.base))
    }

    #[doc= "Get a reference to the value of field `commit_message_pattern` after provisioning.\n"]
    pub fn commit_message_pattern(&self) -> ListRef<RepositoryRulesetRulesElCommitMessagePatternElRef> {
        ListRef::new(self.shared().clone(), format!("{}.commit_message_pattern", self.base))
    }

    #[doc= "Get a reference to the value of field `committer_email_pattern` after provisioning.\n"]
    pub fn committer_email_pattern(&self) -> ListRef<RepositoryRulesetRulesElCommitterEmailPatternElRef> {
        ListRef::new(self.shared().clone(), format!("{}.committer_email_pattern", self.base))
    }

    #[doc= "Get a reference to the value of field `pull_request` after provisioning.\n"]
    pub fn pull_request(&self) -> ListRef<RepositoryRulesetRulesElPullRequestElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pull_request", self.base))
    }

    #[doc= "Get a reference to the value of field `required_deployments` after provisioning.\n"]
    pub fn required_deployments(&self) -> ListRef<RepositoryRulesetRulesElRequiredDeploymentsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.required_deployments", self.base))
    }

    #[doc= "Get a reference to the value of field `required_status_checks` after provisioning.\n"]
    pub fn required_status_checks(&self) -> ListRef<RepositoryRulesetRulesElRequiredStatusChecksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.required_status_checks", self.base))
    }

    #[doc= "Get a reference to the value of field `tag_name_pattern` after provisioning.\n"]
    pub fn tag_name_pattern(&self) -> ListRef<RepositoryRulesetRulesElTagNamePatternElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tag_name_pattern", self.base))
    }
}

#[derive(Serialize, Default)]
struct RepositoryRulesetDynamic {
    bypass_actors: Option<DynamicBlock<RepositoryRulesetBypassActorsEl>>,
    conditions: Option<DynamicBlock<RepositoryRulesetConditionsEl>>,
    rules: Option<DynamicBlock<RepositoryRulesetRulesEl>>,
}
