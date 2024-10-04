use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;

#[proc_macro_derive(MemoDoc)]
#[doc = r#"# MemoDoc
Implements the `MemoDoc` trait to struct.
Struct must have field uuid to derive this trait.
```
impl MemoDoc for #doc {
    fn get_id(&self) -> &str {
        &self.uuid
    }

    fn set_id(&mut self, id: &str) {
        self.uuid = id.to_string();
    }
}
```
"#]
pub fn memodoc_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let doc: Ident = ast.ident;
    proc_macro::TokenStream::from(quote! {
        impl MemoDoc for #doc {
            fn get_id(&self) -> &str {
                &self.uuid
            }
            fn set_id(&mut self, id: &str) {
                self.uuid = id.to_string();
            }
        }
    })
}
