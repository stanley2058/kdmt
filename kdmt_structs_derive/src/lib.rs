extern crate proc_macro;
use quote::quote;
use syn;

use proc_macro::TokenStream;

#[proc_macro_derive(KdmtService)]
pub fn to_service_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_to_service_macro(&ast)
}

fn impl_to_service_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        use crate::data::base::{Service, ToService};
        impl ToService for #name {
            fn to_service(&self) -> Service {
                let unique_service_name = self.unique_service_name.clone();
                let service = self.service.clone();
                let namespace = self.namespace.clone();
                let version = self.namespace.clone();

                Service {
                    unique_service_name,
                    service,
                    namespace,
                    version,
                }
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(KdmtEndpoint)]
pub fn to_endpoint_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_to_endpoint_macro(&ast)
}

fn impl_to_endpoint_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        use crate::data::base::{Endpoint, ToEndpoint};
        impl ToEndpoint for #name {
            fn to_endpoint(&self) -> Endpoint {
                let service = self.to_service();
                let unique_endpoint_name = self.unique_endpoint_name.clone();
                let method = self.method.clone();

                Endpoint {
                    service,
                    unique_endpoint_name,
                    method,
                }
            }
        }
    };
    gen.into()
}
