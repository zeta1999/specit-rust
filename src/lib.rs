extern crate proc_macro;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn it(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let lit_str = {
        let a = args.clone();
        parse_macro_input!(a as syn::LitStr)
    };

    let input_item = {
        let i = input.clone();
        parse_macro_input!(i as syn::Item)
    };

    let syn_fn = match input_item {
        syn::Item::Fn(x) => x,
        _ => panic!("not function"),
    };

    let fn_ret_type = syn_fn.sig.output;
    let fn_block = syn_fn.block;
    let fn_attrs = syn_fn.attrs;

    let ident = {
        let s = lit_str.value();
        let new_str: String = s
            .chars()
            .map(|x| match x {
                'A'..='Z' | 'a'..='z' | '0'..='9' => x,
                _ => '_',
            })
            .collect();
        syn::Ident::new(&new_str, syn_fn.sig.ident.span())
    };

    let q = quote! {
        #[test]
        #(#fn_attrs)*
        fn #ident() #fn_ret_type #fn_block
    };
    q.into()
}