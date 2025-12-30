/// Define a message error.
#[macro_export]
macro_rules! message_error {
    ( $type:ident $(,)? ) => {
        $crate::message_error!($type, "");
    };

    ( $type:ident, $display_prefix:expr $(,)? ) => {
        #[doc = concat!(stringify!($type), ".")]
        #[derive(Clone, Debug, Default)]
        pub struct $type(pub ::std::option::Option<::std::string::String>);

        impl $type {
            /// Constructor.
            pub fn new<ToStringT>(message: ToStringT) -> Self
            where
                ToStringT: ToString,
            {
                message.to_string().into()
            }
        }

        impl ::std::fmt::Display for $type {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match &self.0 {
                    ::std::option::Option::Some(message) => {
                        if $display_prefix.is_empty() {
                            ::std::fmt::Display::fmt(message, formatter)
                        } else {
                            ::std::write!(formatter, "{}: {}", $display_prefix, message)
                        }
                    }
                    ::std::option::Option::None => {
                        if $display_prefix.is_empty() {
                            Ok(())
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

        impl ::std::convert::Into<String> for $type {
            fn into(self) -> String {
                self.0.unwrap_or_else(|| $display_prefix.to_string())
            }
        }
    };
}

#[allow(unused_imports)]
pub use message_error;
