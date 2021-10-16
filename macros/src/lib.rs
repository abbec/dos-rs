use proc_macro::TokenStream;
use quote::quote;

fn token_stream_with_error(mut tokens: TokenStream, error: syn::Error) -> TokenStream {
    tokens.extend(TokenStream::from(error.into_compile_error()));
    tokens
}

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    match syn::parse::<syn::ItemFn>(item.clone()) {
        Ok(f) => {
            let function_name = f.sig.ident;
            let mut wrap = TokenStream::from(quote!(
                #[no_mangle]
                fn start() { dos::exit(#function_name()) }
            ));
            wrap.extend(item);
            wrap
        }
        Err(e) => token_stream_with_error(item, e),
    }
}
