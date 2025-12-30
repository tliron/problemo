/// Define a tag error.
#[macro_export]
macro_rules! tag_error {
    ( $type:ident $(,)? ) => {
        $crate::tag_error!($type, stringify!($type));
    };

    ( $type:ident, $display:expr $(,)? ) => {
        #[doc = concat!(stringify!($type), ".")]
        #[derive(Clone, Copy, Debug, Default)]
        pub struct $type;

        impl ::std::fmt::Display for $type {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::write!(formatter, "{}", $display)
            }
        }

        impl ::std::error::Error for $type {}

        impl ::std::cmp::PartialEq for $type {
            fn eq(&self, _other: &Self) -> bool {
                true
            }
        }

        impl ::std::cmp::Eq for $type {}
    };
}

#[allow(unused_imports)]
pub use tag_error;
