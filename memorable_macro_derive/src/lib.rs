use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;

#[proc_macro_derive(MemoDoc)]
pub fn memodoc_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let doc: Ident = ast.ident;
    quote! {
        impl MemoDoc for #doc {
            fn get_id(&self) -> &str {
                &self.uuid
            }
            fn set_id(&mut self, id: &str) {
                self.uuid = id.to_string();
            }
        }
    }.into()
}
