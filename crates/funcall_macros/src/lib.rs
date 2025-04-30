use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{FnArg, ItemFn, Pat, PatIdent, Type, parse_macro_input};

#[proc_macro_attribute]
pub fn funcall(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let name = &func.sig.ident;
    let wrapper_name = format_ident!("{}_tool", name);

    let mut arg_names = Vec::new();
    let mut extract_stmts = Vec::new();

    for (i, input) in func.sig.inputs.iter().enumerate() {
        if let FnArg::Typed(pat_type) = input {
            let ident = match &*pat_type.pat {
                Pat::Ident(PatIdent { ident, .. }) => ident,
                _ => panic!("Unsupported pattern"),
            };

            let (stmt, _) = extract_type_code(ident, &pat_type.ty, i);
            arg_names.push(ident);
            extract_stmts.push(stmt);
        }
    }

    let arg_count = arg_names.len();

    let expanded = quote! {
        #func

        pub fn #wrapper_name(args: &::serde_json::Value) -> ::serde_json::Value {
            let args = args.as_array().expect("expected JSON array");
            assert_eq!(args.len(), #arg_count, "wrong number of args");

            #(#extract_stmts)*

            let result = #name(#(#arg_names),*);
            ::serde_json::json!(result)
        }
    };

    TokenStream::from(expanded)
}

fn extract_type_code(
    ident: &syn::Ident,
    ty: &Box<Type>,
    index: usize,
) -> (proc_macro2::TokenStream, bool) {
    let ty_str = quote!(#ty).to_string().replace(' ', "");

    let stmt = if ty_str == "i32" {
        quote! {
            let #ident = args[#index].as_i64().expect("expected i64") as i32;
        }
    } else if ty_str == "f64" {
        quote! {
            let #ident = args[#index].as_f64().expect("expected f64");
        }
    } else if ty_str == "bool" {
        quote! {
            let #ident = args[#index].as_bool().expect("expected bool");
        }
    } else if ty_str == "String" {
        quote! {
            let #ident = args[#index].as_str().expect("expected string").to_string();
        }
    } else if ty_str.starts_with("Option<") {
        quote! {
            let #ident = match args.get(#index) {
                Some(v) if !v.is_null() => Some(::serde::Deserialize::deserialize(v).expect("failed to parse Option")),
                _ => None
            };
        }
    } else if ty_str.starts_with("Vec<") {
        quote! {
            let #ident: #ty = ::serde::Deserialize::deserialize(&args[#index]).expect("failed to parse Vec");
        }
    } else {
        // Assume any other type implements Deserialize
        quote! {
            let #ident: #ty = ::serde::Deserialize::deserialize(&args[#index]).expect("failed to deserialize struct");
        }
    };

    (stmt, true)
}
