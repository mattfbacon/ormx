use std::{borrow::Cow, convert::TryFrom, marker::PhantomData};

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident, Path, Result, Type};

use crate::backend::{Backend, Implementation};

mod parse;

pub struct Patch<B: Backend> {
    pub ident: Ident,
    pub table_name: String,
    pub reserved_table_name: bool,
    pub table: Path,
    pub id: String,
    pub fields: Vec<PatchField>,
    pub _phantom: PhantomData<*const B>,
}

pub struct PatchField {
    pub ident: Ident,
    pub column: String,
    pub ty: Type,
    pub custom_type: bool,
    pub by_ref: bool,
}

impl<B: Backend> Patch<B> {
    pub fn table_name(&self) -> Cow<str> {
        if self.reserved_table_name {
            format!("{}{}{}", B::QUOTE, self.table_name, B::QUOTE).into()
        } else {
            Cow::Borrowed(&self.table_name)
        }
    }
}

impl PatchField {
    pub fn fmt_as_argument(&self) -> TokenStream {
        let ident = &self.ident;
        let ty = &self.ty;

        let mut out = quote!(#ident);
        if self.custom_type {
            out = quote!(#out as #ty);
        }
        if self.by_ref {
            out = quote!(&(#out));
        }
        out
    }
}

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    let parsed = Patch::try_from(&input)?;
    Ok(Implementation::impl_patch(&parsed))
}
