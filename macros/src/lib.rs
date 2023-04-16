use quote::{quote, quote_spanned};
use proc_macro::TokenStream;
use syn::{
    DeriveInput, Error, Fields,
    spanned::Spanned,
};


#[proc_macro_derive(Empty)]
pub fn derive_empty(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    derive_impl(&input).unwrap_or_else(|e| e.to_compile_error().into())
}


/// Generates the `impl Empty` code for the given input type definition.
fn derive_impl(input: &DeriveInput) -> Result<TokenStream, Error> {
    // Make sure this is a struct and pull out the fields.
    let fields = if let syn::Data::Struct(s) = &input.data {
        &s.fields
    } else {
        return Err(Error::new(input.span(), "only structs can derive `Empty`"))
    };

    // Prepare stuff for impl header.
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Generate the method body
    let body = match fields {
        Fields::Named(fields) => {
            let field_initializer = fields.named.iter().map(|f| {
                let name = &f.ident;
                let ty = &f.ty;
                quote_spanned! {f.span()=>
                    #name: <#ty as leer::Empty>::empty()
                }
            });

            quote! {
                Self {
                    #(#field_initializer ,)*
                }
            }
        }
        Fields::Unnamed(fields) => {
            let field_initializer = fields.unnamed.iter().map(|f| {
                let ty = &f.ty;
                quote_spanned! {f.span()=>
                    <#ty as leer::Empty>::empty()
                }
            });

            quote! {
                #name(#(#field_initializer ,)*)
            }

        }
        Fields::Unit => quote! { #name },
    };

    // Combine everything.
    let out = quote! {
        impl #impl_generics leer::Empty for #name #ty_generics #where_clause {
            fn empty() -> Self {
                #body
            }
        }
    };

    Ok(out.into())
}
