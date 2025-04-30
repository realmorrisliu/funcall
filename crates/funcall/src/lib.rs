pub use funcall_macros::funcall;

#[macro_export]
macro_rules! tools {
    ( $( $fn_name:ident ),* $(,)? ) => {{
        let mut map: ::std::collections::HashMap<&'static str, fn(&::serde_json::Value) -> ::serde_json::Value> = ::std::collections::HashMap::new();
        $(
            map.insert(stringify!($fn_name), $crate::paste::paste! { [<$fn_name _tool>] } as fn(&::serde_json::Value) -> ::serde_json::Value);
        )*
        map
    }};
}

pub use paste;
