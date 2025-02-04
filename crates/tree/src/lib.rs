// Copyright (c) 2018 The predicates-rs Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Render `Case` as a tree.

use std::fmt;

use predicates_core::reflection;

/// Render `Self` as a displayable tree.
pub trait CaseTreeExt {
    /// Render `Self` as a displayable tree.
    fn tree(&self) -> CaseTree;
}

impl<'a> CaseTreeExt for reflection::Case<'a> {
    fn tree(&self) -> CaseTree {
        CaseTree(convert(self))
    }
}

type CaseTreeInner = treeline::Tree<Box<dyn fmt::Display + Send + Sync>>;

fn convert(case: &reflection::Case<'_>) -> CaseTreeInner {
    let mut leaves: Vec<CaseTreeInner> = vec![];

    leaves.extend(case.predicate().iter().flat_map(|pred| {
        pred.parameters().map(|item| {
            let root: Box<dyn fmt::Display + Send + Sync> = Box::new(item.to_string());
            treeline::Tree::new(root, vec![])
        })
    }));

    leaves.extend(case.products().map(|item| {
        let root: Box<dyn fmt::Display + Send + Sync> = Box::new(item.to_string());
        treeline::Tree::new(root, vec![])
    }));

    leaves.extend(case.children().map(|item| convert(item)));

    let root = Box::new(case.predicate().map(|p| p.to_string()).unwrap_or_default());
    CaseTreeInner::new(root, leaves)
}

/// A `Case` rendered as a tree for display.
#[allow(missing_debug_implementations)]
pub struct CaseTree(CaseTreeInner);

impl fmt::Display for CaseTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
