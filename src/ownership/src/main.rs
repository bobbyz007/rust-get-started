use crate::bind_scope::{brace_scope, closure_scope, func_scope, let_scope, loop_scope, match_scope};
use crate::copy_semantic::{copy_semantic, copy_semantic_auto, copy_semantic_borrow, is_copy_semantic};
use crate::lifetime::{lifetime_bound, lifetime_func, lifetime_omit_rules, lifetime_struct, lifetime_trait_object, lifetime_trait_object_explicit, return_str};
use crate::reference_borrow::borrow;
use crate::subtype_variance::{lifetime_covariant, lifetime_invariant};

mod copy_semantic;
mod bind_scope;
mod reference_borrow;
mod lifetime;
mod subtype_variance;

fn main() {
    println!("-----------------------------copy semantic----------------------------------");
    copy_semantic();
    copy_semantic_auto();
    copy_semantic_borrow();
    is_copy_semantic();

    println!("-------------------------------bind scope------------------------------------");
    brace_scope();
    match_scope();
    loop_scope();
    let_scope();
    func_scope();
    closure_scope();

    println!("-------------------------------borrow----------------------------------------");
    borrow();

    println!("-------------------------------lifetime---------------------------------------");
    return_str();
    lifetime_func();
    lifetime_struct();
    lifetime_omit_rules();
    lifetime_bound();
    lifetime_trait_object();
    lifetime_trait_object_explicit(&[2, 3]);
    lifetime_covariant();
    lifetime_invariant();


}



