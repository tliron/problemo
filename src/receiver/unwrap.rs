/// Like [Result::unwrap] but gives [Err] to a [ProblemReceiver](super::ProblemReceiver) and
/// returns [Ok].
///
/// In practice works somewhat similarly to the `?` operator.
#[macro_export]
macro_rules! give_unwrap {
    ( $result:expr, $receiver:expr, $default:expr $(,)? ) => {
        match $result {
            ::std::result::Result::Ok(ok) => ok,
            ::std::result::Result::Err(error) => {
                $crate::ProblemReceiver::give($receiver, error.into())?;
                return ::std::result::Result::Ok($default);
            }
        }
    };

    ( $result:expr, $receiver:expr $(,)? ) => {
        $crate::give_unwrap!($result, $receiver, ::std::default::Default::default())
    };
}

#[allow(unused_imports)]
pub use give_unwrap;
