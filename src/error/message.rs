/// Define a [String] error type.
///
/// It's a trivial [Option] newtype with a `new(ToString)` constructor.
///
/// When the inner value is [None] it behaves like a [tag_error!](super::tag::tag_error) type.
///
/// The first argument is the type name. The second optional argument is a prefix for the
/// [Display](std::fmt::Display) message.
#[macro_export]
macro_rules! message_error {
    ( $type:ident $(,)? ) => {
        $crate::message_error!($type, "");
    };

    ( $type:ident, $display_prefix:literal $(,)? ) => {
        #[doc = concat!(stringify!($type), ".")]
        #[derive(Clone, Debug, Default)]
        pub struct $type(pub ::std::option::Option<::std::string::String>);

        impl $type {
            /// Constructor.
            pub fn new<ToStringT>(message: ToStringT) -> Self
            where
                ToStringT: ::std::string::ToString,
            {
                message.to_string().into()
            }
        }

        impl ::std::fmt::Display for $type {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match self.0.as_ref().filter(|message| !message.is_empty()) {
                    ::std::option::Option::Some(message) => {
                        if $display_prefix.is_empty() {
                            ::std::fmt::Display::fmt(message, formatter)
                        } else {
                            ::std::write!(formatter, "{}: {}", $display_prefix, message)
                        }
                    }

                    ::std::option::Option::None => {
                        if $display_prefix.is_empty() {
                            ::std::fmt::Display::fmt(stringify!($type), formatter)
                        } else {
                            ::std::fmt::Display::fmt($display_prefix, formatter)
                        }
                    }
                }
            }
        }

        impl ::std::error::Error for $type {}

        impl ::std::cmp::PartialEq for $type {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl ::std::cmp::Eq for $type {}

        impl ::std::convert::From<::std::string::String> for $type {
            fn from(message: ::std::string::String) -> Self {
                Self(::std::option::Option::Some(message))
            }
        }

        impl ::std::convert::From<&str> for $type {
            fn from(message: &str) -> Self {
                ::std::string::String::from(message).into()
            }
        }
    };
}

/// Define a `&'static str` error type.
///
/// When the inner value is [None] it behaves like a [tag_error!](super::tag::tag_error) type.
///
/// The first argument is the type name. The second optional argument is a prefix for the
/// [Display](std::fmt::Display) message.
#[macro_export]
macro_rules! static_message_error {
    ( $type:ident $(,)? ) => {
        $crate::static_message_error!($type, "");
    };

    ( $type:ident, $display_prefix:literal $(,)? ) => {
        #[doc = concat!(stringify!($type), ".")]
        #[derive(Clone, Debug, Default)]
        pub struct $type(pub ::std::option::Option<&'static str>);

        impl $type {
            /// Constructor.
            pub fn new<IntoStringT>(message: IntoStringT) -> Self
            where
                IntoStringT: ::std::convert::Into<&'static str>,
            {
                message.into::<&'static str>().into()
            }
        }

        impl ::std::fmt::Display for $type {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match self.0.as_ref().filter(|message| !message.is_empty()) {
                    ::std::option::Option::Some(message) => {
                        if $display_prefix.is_empty() {
                            ::std::fmt::Display::fmt(message, formatter)
                        } else {
                            ::std::write!(formatter, "{}: {}", $display_prefix, message)
                        }
                    }

                    ::std::option::Option::None => {
                        if $display_prefix.is_empty() {
                            ::std::fmt::Display::fmt(stringify!($type), formatter)
                        } else {
                            ::std::fmt::Display::fmt($display_prefix, formatter)
                        }
                    }
                }
            }
        }

        impl ::std::error::Error for $type {}

        impl ::std::cmp::PartialEq for $type {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl ::std::cmp::Eq for $type {}

        impl ::std::convert::From<&'static str> for $type {
            fn from(message: &'static str) -> Self {
                Self(::std::option::Option::Some(message))
            }
        }
    };
}

#[allow(unused_imports)]
pub use {message_error, static_message_error};
