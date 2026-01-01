/// Define a [String] attachment type.
///
/// It's a trivial newtype with a `new(ToString)` constructor.
///
/// The argument is the type name.
#[macro_export]
macro_rules! string_attachment {
    ( $type:ident $(,)? ) => {
        #[doc = concat!(stringify!($type), ".")]
        #[derive(Clone, Debug, Default)]
        pub struct $type(pub ::std::string::String);

        impl $type {
            /// Constructor.
            pub fn new<ToStringT>(inner: ToStringT) -> Self
            where
                ToStringT: ::std::string::ToString,
            {
                Self(inner.to_string())
            }
        }

        impl ::std::convert::From<::std::string::String> for $type {
            fn from(inner: ::std::string::String) -> Self {
                Self(inner)
            }
        }

        impl ::std::convert::Into<::std::string::String> for $type {
            fn into(self) -> ::std::string::String {
                self.0
            }
        }
    };
}

/// Define a [&'static str] attachment type.
///
/// It's a trivial newtype with a `new(Into<&'static str>)` constructor.
///
/// The argument is the type name.
#[macro_export]
macro_rules! static_string_attachment {
    ( $type:ident $(,)? ) => {
        $crate::simple_attachment!($type, &'static str);
    };
}

/// Define an attachment type.
///
/// It's a trivial newtype with a `new(Into<InnerT>)` constructor.
///
/// The first argument is the type name. The second argument is the inner type name.
#[macro_export]
macro_rules! attachment {
    ( $type:ident, $inner_type:ty $(,)? ) => {
        #[doc = concat!(stringify!($type), ".")]
        #[derive(Clone, Debug, Default)]
        pub struct $type(pub $inner_type);

        impl $type {
            /// Constructor.
            pub fn new<InnerT>(inner: InnerT) -> Self
            where
                InnerT: ::std::convert::Into<$inner_type>,
            {
                Self(inner.into())
            }
        }

        impl ::std::convert::From<$inner_type> for $type {
            fn from(inner: $inner_type) -> Self {
                Self(inner)
            }
        }

        impl ::std::convert::Into<$inner_type> for $type {
            fn into(self) -> $inner_type {
                self.0
            }
        }
    };
}

#[allow(unused_imports)]
pub use {attachment, static_string_attachment, string_attachment};
