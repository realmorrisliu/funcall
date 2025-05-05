use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{FnArg, ItemFn, Pat, PatIdent, Type, parse_macro_input};

#[proc_macro_attribute]
pub fn funcall(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let name = &func.sig.ident;
    let wrapper_name = format_ident!("{}_tool", name);

    let mut arg_idents = Vec::new();
    let mut positional_stmts = Vec::new();
    let mut named_stmts = Vec::new();

    for (i, input) in func.sig.inputs.iter().enumerate() {
        if let FnArg::Typed(pat_type) = input {
            let ident = match &*pat_type.pat {
                Pat::Ident(PatIdent { ident, .. }) => ident.clone(),
                _ => panic!("Unsupported argument pattern"),
            };
            let ty = &pat_type.ty;
            let index = syn::Index::from(i);
            let key = ident.to_string();

            let (pos_stmt, named_stmt) = extract_dual(&ident, ty, &index, &key);
            arg_idents.push(ident);
            positional_stmts.push(pos_stmt);
            named_stmts.push(named_stmt);
        }
    }

    let expanded = quote! {
        #func

        pub fn #wrapper_name(args: &::serde_json::Value) -> ::serde_json::Value {
            #(let #arg_idents;)*
            if let Some(arr) = args.as_array() {
                #(#positional_stmts)*
            } else if let Some(obj) = args.as_object() {
                #(#named_stmts)*
            } else {
                panic!("expected JSON array or object");
            }

            let result = #name(#(#arg_idents),*);
            ::serde_json::json!(result)
        }
    };

    TokenStream::from(expanded)
}

fn extract_dual(
    ident: &syn::Ident,
    ty: &Box<Type>,
    index: &syn::Index,
    key: &str,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let ty_str = quote!(#ty).to_string().replace(' ', "");

    let positional = if ty_str == "i32" {
        quote! { #ident = arr[#index].as_i64().expect("expected i64") as i32; }
    } else if ty_str == "f64" {
        quote! { #ident = arr[#index].as_f64().expect("expected f64"); }
    } else if ty_str == "bool" {
        quote! { #ident = arr[#index].as_bool().expect("expected bool"); }
    } else if ty_str == "String" {
        quote! { #ident = arr[#index].as_str().expect("expected string").to_string(); }
    } else {
        // fallback to full deserialization for Option<T>, Vec<T>, struct
        quote! {
            #ident = ::serde::Deserialize::deserialize(&arr[#index]).expect("failed to deserialize positional");
        }
    };

    let named = if ty_str == "i32" {
        quote! { #ident = obj[#key].as_i64().expect("expected i64") as i32; }
    } else if ty_str == "f64" {
        quote! { #ident = obj[#key].as_f64().expect("expected f64"); }
    } else if ty_str == "bool" {
        quote! { #ident = obj[#key].as_bool().expect("expected bool"); }
    } else if ty_str == "String" {
        quote! { #ident = obj[#key].as_str().expect("expected string").to_string(); }
    } else if ty_str.starts_with("Option<") {
        quote! {
            #ident = match obj.get(#key) {
                Some(v) if !v.is_null() => Some(::serde::Deserialize::deserialize(v).expect("failed to parse Option")),
                _ => None
            };
        }
    } else {
        quote! {
            #ident = ::serde::Deserialize::deserialize(&obj[#key]).expect("failed to deserialize named param");
        }
    };

    (positional, named)
}
