extern crate proc_macro;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn it(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    general_it(args, input, syn::parse_quote! {#[test]}, None)
}

#[cfg(feature = "tokio")]
#[proc_macro_attribute]
pub fn tokio_it(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    general_it(
        args,
        input,
        syn::parse_quote! {#[test]},
        Some(syn::parse_quote! {#[tokio::test]}),
    )
}

#[cfg(feature = "async-std")]
#[proc_macro_attribute]
pub fn async_std_it(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    general_it(
        args,
        input,
        syn::parse_quote! {#[test]},
        Some(syn::parse_quote! {#[async_std::test]}),
    )
}

#[cfg(feature = "lib-wasm-bindgen")]
#[proc_macro_attribute]
pub fn wasm_bindgen_test_it(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    general_it(
        args,
        input,
        syn::parse_quote! {#[wasm_bindgen_test::wasm_bindgen_test]},
        Some(syn::parse_quote! {#[wasm_bindgen_test::wasm_bindgen_test]}),
    )
}

fn string_to_alphanum(s: String) -> String {
    s.chars()
        .map(|x| match x {
            'A'..='Z' | 'a'..='z' | '0'..='9' => x,
            _ => '_',
        })
        .collect()
}

// NOTE: This function is used in macros
#[allow(dead_code)]
fn general_it(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
    sync_attribute_option: syn::Attribute,
    async_attribute_option: Option<syn::Attribute>,
) -> proc_macro::TokenStream {
    let lit_str = parse_macro_input!(args as syn::LitStr);
    let input_item = parse_macro_input!(input as syn::Item);
    let syn_fn = match input_item {
        syn::Item::Fn(x) => x,
        _ => panic!("not function"),
    };
    let fn_ret_type = syn_fn.sig.output;
    let fn_block = syn_fn.block;
    let mut fn_attrs = syn_fn.attrs;
    let fn_asyncness = syn_fn.sig.asyncness;

    // If async function
    if fn_asyncness.is_some() {
        // If async attribute is found
        if let Some(async_attribute) = async_attribute_option {
            fn_attrs.push(async_attribute);
        }
    } else {
        fn_attrs.push(sync_attribute_option);
    }

    let ident = {
        let new_str: String = string_to_alphanum(lit_str.value());
        syn::Ident::new(&new_str, syn_fn.sig.ident.span())
    };

    let q = quote! {
        #[allow(non_snake_case)]
        #(#fn_attrs)*
        #fn_asyncness fn #ident() #fn_ret_type #fn_block
    };
    q.into()
}

#[proc_macro_attribute]
pub fn describe(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let lit_str = parse_macro_input!(args as syn::LitStr);
    let i = input.clone();
    let item_mod = parse_macro_input!(i as syn::ItemMod);

    let mod_attrs = item_mod.attrs;
    let (_, mod_content_items) = item_mod.content.expect("no mod content");
    let mod_ident = {
        let new_str: String = string_to_alphanum(lit_str.value());
        syn::Ident::new(&new_str, item_mod.ident.span())
    };

    let q = quote! {
        #(#mod_attrs)*
        mod #mod_ident {
            #(#mod_content_items)*
        }
    };
    q.into()
}
