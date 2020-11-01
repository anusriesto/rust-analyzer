//! This module defines an accumulator for completions which are going to be presented to user.

pub(crate) mod attribute;
pub(crate) mod dot;
pub(crate) mod record;
pub(crate) mod pattern;
pub(crate) mod fn_param;
pub(crate) mod keyword;
pub(crate) mod snippet;
pub(crate) mod qualified_path;
pub(crate) mod unqualified_path;
pub(crate) mod postfix;
pub(crate) mod macro_in_item_position;
pub(crate) mod trait_impl;
pub(crate) mod mod_;

use hir::{ModPath, ScopeDef, Type};

use crate::{item::Builder, render::*, CompletionContext, CompletionItem};

/// Represents an in-progress set of completions being built.
#[derive(Debug, Default)]
pub struct Completions {
    buf: Vec<CompletionItem>,
}

impl Into<Vec<CompletionItem>> for Completions {
    fn into(self) -> Vec<CompletionItem> {
        self.buf
    }
}

impl Builder {
    /// Convenience method, which allows to add a freshly created completion into accumulator
    /// without binding it to the variable.
    pub(crate) fn add_to(self, acc: &mut Completions) {
        acc.add(self.build())
    }
}

impl Completions {
    pub(crate) fn add(&mut self, item: CompletionItem) {
        self.buf.push(item.into())
    }

    pub(crate) fn add_all<I>(&mut self, items: I)
    where
        I: IntoIterator,
        I::Item: Into<CompletionItem>,
    {
        items.into_iter().for_each(|item| self.add(item.into()))
    }

    pub(crate) fn add_field(&mut self, ctx: &CompletionContext, field: hir::Field, ty: &Type) {
        let item = Render::new(ctx.into()).add_field(field, ty);
        self.add(item);
    }

    pub(crate) fn add_tuple_field(&mut self, ctx: &CompletionContext, field: usize, ty: &Type) {
        let item = Render::new(ctx.into()).add_tuple_field(field, ty);
        self.add(item);
    }

    pub(crate) fn add_resolution(
        &mut self,
        ctx: &CompletionContext,
        local_name: String,
        resolution: &ScopeDef,
    ) {
        if let Some(item) = Render::new(ctx.into()).render_resolution(local_name, resolution) {
            self.add(item);
        }
    }

    pub(crate) fn add_macro(
        &mut self,
        ctx: &CompletionContext,
        name: Option<String>,
        macro_: hir::MacroDef,
    ) {
        let name = match name {
            Some(it) => it,
            None => return,
        };
        if let Some(item) = MacroRender::new(ctx.into(), name, macro_).render() {
            self.add(item);
        }
    }

    pub(crate) fn add_function(
        &mut self,
        ctx: &CompletionContext,
        func: hir::Function,
        local_name: Option<String>,
    ) {
        let item = FunctionRender::new(ctx.into(), local_name, func).render();
        self.add(item)
    }

    pub(crate) fn add_const(&mut self, ctx: &CompletionContext, constant: hir::Const) {
        if let Some(item) = ConstRender::new(ctx.into(), constant).render() {
            self.add(item);
        }
    }

    pub(crate) fn add_type_alias(&mut self, ctx: &CompletionContext, type_alias: hir::TypeAlias) {
        if let Some(item) = TypeAliasRender::new(ctx.into(), type_alias).render() {
            self.add(item)
        }
    }

    pub(crate) fn add_qualified_enum_variant(
        &mut self,
        ctx: &CompletionContext,
        variant: hir::EnumVariant,
        path: ModPath,
    ) {
        let item = EnumVariantRender::new(ctx.into(), None, variant, Some(path)).render();
        self.add(item);
    }

    pub(crate) fn add_enum_variant(
        &mut self,
        ctx: &CompletionContext,
        variant: hir::EnumVariant,
        local_name: Option<String>,
    ) {
        let item = EnumVariantRender::new(ctx.into(), local_name, variant, None).render();
        self.add(item);
    }
}
