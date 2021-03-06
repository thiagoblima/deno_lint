// Copyright 2020 the Deno authors. All rights reserved. MIT license.
use crate::linter::Context;

pub mod ban_ts_comment;
pub mod ban_ts_ignore;
pub mod ban_types;
pub mod ban_untagged_ignore;
pub mod ban_untagged_todo;
pub mod constructor_super;
pub mod default_param_last;
pub mod eqeqeq;
pub mod explicit_function_return_type;
pub mod for_direction;
pub mod getter_return;
pub mod no_array_constructor;
pub mod no_async_promise_executor;
pub mod no_await_in_loop;
pub mod no_case_declarations;
pub mod no_class_assign;
pub mod no_compare_neg_zero;
pub mod no_cond_assign;
pub mod no_const_assign;
pub mod no_debugger;
pub mod no_delete_var;
pub mod no_dupe_args;
pub mod no_dupe_class_members;
pub mod no_dupe_else_if;
pub mod no_dupe_keys;
pub mod no_duplicate_case;
pub mod no_empty;
pub mod no_empty_character_class;
pub mod no_empty_interface;
pub mod no_empty_pattern;
pub mod no_eval;
pub mod no_ex_assign;
pub mod no_explicit_any;
pub mod no_extra_boolean_cast;
pub mod no_extra_non_null_assertion;
pub mod no_extra_semi;
pub mod no_func_assign;
pub mod no_inferrable_types;
pub mod no_misused_new;
pub mod no_namespace;
pub mod no_new_symbol;
pub mod no_non_null_assertion;
pub mod no_obj_calls;
pub mod no_octal;
pub mod no_prototype_builtins;
pub mod no_regex_spaces;
pub mod no_setter_return;
pub mod no_shadow_restricted_names;
pub mod no_sparse_array;
pub mod no_this_alias;
pub mod no_this_before_super;
pub mod no_throw_literal;
pub mod no_unsafe_finally;
pub mod no_unsafe_negation;
pub mod no_unused_labels;
pub mod no_var;
pub mod no_with;
pub mod prefer_as_const;
pub mod prefer_namespace_keyword;
pub mod require_yield;
pub mod single_var_declarator;
pub mod triple_slash_reference;
pub mod use_isnan;
pub mod valid_typeof;

pub trait LintRule {
  fn new() -> Box<Self>
  where
    Self: Sized;
  fn lint_module(&self, context: Context, module: swc_ecma_ast::Module);
  fn code(&self) -> &'static str;
}

pub fn get_recommended_rules() -> Vec<Box<dyn LintRule>> {
  vec![
    ban_ts_comment::BanTsComment::new(),
    ban_untagged_ignore::BanUntaggedIgnore::new(),
    ban_types::BanTypes::new(),
    constructor_super::ConstructorSuper::new(),
    for_direction::ForDirection::new(),
    getter_return::GetterReturn::new(),
    no_array_constructor::NoArrayConstructor::new(),
    no_async_promise_executor::NoAsyncPromiseExecutor::new(),
    no_case_declarations::NoCaseDeclarations::new(),
    no_class_assign::NoClassAssign::new(),
    no_compare_neg_zero::NoCompareNegZero::new(),
    no_cond_assign::NoCondAssign::new(),
    no_debugger::NoDebugger::new(),
    no_delete_var::NoDeleteVar::new(),
    no_dupe_args::NoDupeArgs::new(),
    no_dupe_class_members::NoDupeClassMembers::new(),
    no_dupe_else_if::NoDupeElseIf::new(),
    no_dupe_keys::NoDupeKeys::new(),
    no_duplicate_case::NoDuplicateCase::new(),
    no_empty_character_class::NoEmptyCharacterClass::new(),
    no_empty_interface::NoEmptyInterface::new(),
    no_empty_pattern::NoEmptyPattern::new(),
    no_empty::NoEmpty::new(),
    no_ex_assign::NoExAssign::new(),
    no_explicit_any::NoExplicitAny::new(),
    no_extra_boolean_cast::NoExtraBooleanCast::new(),
    no_extra_non_null_assertion::NoExtraNonNullAssertion::new(),
    no_extra_semi::NoExtraSemi::new(),
    no_func_assign::NoFuncAssign::new(),
    no_misused_new::NoMisusedNew::new(),
    no_namespace::NoNamespace::new(),
    no_new_symbol::NoNewSymbol::new(),
    no_obj_calls::NoObjCalls::new(),
    no_octal::NoOctal::new(),
    no_prototype_builtins::NoPrototypeBuiltins::new(),
    no_regex_spaces::NoRegexSpaces::new(),
    no_setter_return::NoSetterReturn::new(),
    no_this_alias::NoThisAlias::new(),
    no_this_before_super::NoThisBeforeSuper::new(),
    no_unsafe_finally::NoUnsafeFinally::new(),
    no_unsafe_negation::NoUnsafeNegation::new(),
    no_with::NoWith::new(),
    prefer_as_const::PreferAsConst::new(),
    prefer_namespace_keyword::PreferNamespaceKeyword::new(),
    require_yield::RequireYield::new(),
    triple_slash_reference::TripleSlashReference::new(),
    use_isnan::UseIsNaN::new(),
    valid_typeof::ValidTypeof::new(),
    no_inferrable_types::NoInferrableTypes::new(),
    no_unused_labels::NoUnusedLabels::new(),
    no_shadow_restricted_names::NoShadowRestrictedNames::new(),
  ]
}

pub fn get_all_rules() -> Vec<Box<dyn LintRule>> {
  vec![
    ban_ts_comment::BanTsComment::new(),
    ban_ts_ignore::BanTsIgnore::new(),
    ban_types::BanTypes::new(),
    ban_untagged_ignore::BanUntaggedIgnore::new(),
    ban_untagged_todo::BanUntaggedTodo::new(),
    constructor_super::ConstructorSuper::new(),
    default_param_last::DefaultParamLast::new(),
    eqeqeq::Eqeqeq::new(),
    explicit_function_return_type::ExplicitFunctionReturnType::new(),
    for_direction::ForDirection::new(),
    getter_return::GetterReturn::new(),
    no_array_constructor::NoArrayConstructor::new(),
    no_async_promise_executor::NoAsyncPromiseExecutor::new(),
    no_await_in_loop::NoAwaitInLoop::new(),
    no_case_declarations::NoCaseDeclarations::new(),
    no_class_assign::NoClassAssign::new(),
    no_compare_neg_zero::NoCompareNegZero::new(),
    no_cond_assign::NoCondAssign::new(),
    no_debugger::NoDebugger::new(),
    no_delete_var::NoDeleteVar::new(),
    no_dupe_args::NoDupeArgs::new(),
    no_dupe_class_members::NoDupeClassMembers::new(),
    no_dupe_else_if::NoDupeElseIf::new(),
    no_dupe_keys::NoDupeKeys::new(),
    no_duplicate_case::NoDuplicateCase::new(),
    no_empty_character_class::NoEmptyCharacterClass::new(),
    no_empty_interface::NoEmptyInterface::new(),
    no_empty_pattern::NoEmptyPattern::new(),
    no_empty::NoEmpty::new(),
    no_eval::NoEval::new(),
    no_ex_assign::NoExAssign::new(),
    no_explicit_any::NoExplicitAny::new(),
    no_extra_boolean_cast::NoExtraBooleanCast::new(),
    no_extra_non_null_assertion::NoExtraNonNullAssertion::new(),
    no_extra_semi::NoExtraSemi::new(),
    no_func_assign::NoFuncAssign::new(),
    no_misused_new::NoMisusedNew::new(),
    no_namespace::NoNamespace::new(),
    no_new_symbol::NoNewSymbol::new(),
    no_non_null_assertion::NoNonNullAssertion::new(),
    no_obj_calls::NoObjCalls::new(),
    no_octal::NoOctal::new(),
    no_prototype_builtins::NoPrototypeBuiltins::new(),
    no_regex_spaces::NoRegexSpaces::new(),
    no_setter_return::NoSetterReturn::new(),
    no_sparse_array::NoSparseArray::new(),
    no_this_alias::NoThisAlias::new(),
    no_this_before_super::NoThisBeforeSuper::new(),
    no_throw_literal::NoThrowLiteral::new(),
    no_unsafe_finally::NoUnsafeFinally::new(),
    no_unsafe_negation::NoUnsafeNegation::new(),
    no_var::NoVar::new(),
    no_with::NoWith::new(),
    prefer_as_const::PreferAsConst::new(),
    prefer_namespace_keyword::PreferNamespaceKeyword::new(),
    require_yield::RequireYield::new(),
    single_var_declarator::SingleVarDeclarator::new(),
    triple_slash_reference::TripleSlashReference::new(),
    use_isnan::UseIsNaN::new(),
    valid_typeof::ValidTypeof::new(),
    no_inferrable_types::NoInferrableTypes::new(),
    no_const_assign::NoConstAssign::new(),
    no_unused_labels::NoUnusedLabels::new(),
    no_shadow_restricted_names::NoShadowRestrictedNames::new(),
  ]
}
