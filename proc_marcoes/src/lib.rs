use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // 1. Parse the user's function
    let mut input_fn = parse_macro_input!(item as ItemFn);

    // 2. Rename the user's function so we can call it from our wrappers
    // We change "main" to "__engine_user_main"
    let user_main_ident = syn::Ident::new("__engine_user_main", input_fn.sig.ident.span());
    input_fn.sig.ident = user_main_ident.clone();

    // 3. Generate the wrapper code
    let expanded = quote! {
        // Emit the user's original function (renamed)
        #input_fn

        // --- Native Entry Point ---
        #[cfg(not(target_arch = "wasm32"))]
        fn main(){
            // Simply call the user's function
            __engine_user_main()
        }

        // --- WASM Entry Point ---
        #[cfg(target_arch = "wasm32")]
        use wasm_bindgen::prelude::*;

        #[cfg(target_arch = "wasm32")]
        #[wasm_bindgen(start)]
        pub fn main(){
            // Call the user's function
            // We map the Rust error to a JsValue string so the browser console shows it
            __engine_user_main()
        }
    };

    TokenStream::from(expanded)
}