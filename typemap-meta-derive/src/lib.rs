extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{self, Attribute, Data, Fields};

/// Add static type-to-value getters to a tuple struct containing disjoint heterogeneous types
#[proc_macro_derive(Typemap, attributes(typemap_mut))]
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
    let all_mut = has_mut_attr(&ast.attrs);

    let types: Vec<_> = tuple_fields
        .unnamed
        .iter()
        .map(|e| e.ty.to_token_stream())
        .collect();
    let indices: Vec<_> = (0..types.len()).map(syn::Index::from).collect();
    let name = &ast.ident;
    let generics = &ast.generics;
    let gen = quote! {
        #(impl #generics Get<#types> for #name #generics {
            fn get(&self) -> &#types {
                &self.#indices
            }
        })*
    };
    let gen_mut = if all_mut {
        Some(quote! {
            #(impl #generics GetMut<#types> for #name #generics {
                fn get_mut(&mut self) -> &mut #types {
                    &mut self.#indices
                }
            })*
        })
    } else {
        None
    };

    quote! {
        #gen
        #gen_mut
    }
    .into()
}

fn has_mut_attr(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| attr.path.is_ident("typemap_mut"))
}
