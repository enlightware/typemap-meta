extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{self, Data, Fields};

/// Add static type-to-value getters to a tuple struct containing disjoint heterogeneous types
#[proc_macro_derive(Typemap)]
pub fn typemap_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_typemap_macro(&ast)
}

fn impl_typemap_macro(ast: &syn::DeriveInput) -> TokenStream {
    let struct_data = if let Data::Struct(s) = &ast.data {
        s
    } else {
        panic!("Typemap only applies to tuple struct, but used on a non-struct!")
    };
    let tuple_fields = if let Fields::Unnamed(f) = &struct_data.fields {
        f
    } else {
        panic!("Typemap only applies to tuple struct, but used on a non-tuple struct!")
    };
    let types = tuple_fields.unnamed.iter().map(|e| e.ty.to_token_stream());
    let indices = (0..types.len()).map(syn::Index::from);
    let name = &ast.ident;
    let generics = &ast.generics;
    let gen = quote! {
        #(impl #generics Get<#types> for #name #generics {
            fn get(&self) -> &#types {
                &self.#indices
            }
        })*
    };
    gen.into()
}
