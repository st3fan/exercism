#[macro_export]
macro_rules! hashmap {
    ( $( $k:literal => $v:expr, )* ) => {
        {
            let mut temp_hashmap = ::std::collections::HashMap::new();
            $(
                temp_hashmap.insert($k, $v);
            )*
            temp_hashmap
        }
    };

    ( $( $k:literal => $v:expr ),* ) => {
        macros::hashmap!($($k => $v,)*);
    };
}
