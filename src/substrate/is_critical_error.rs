// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use crate::substrate::{
    CriticalError,
    SupportCriticalError,
};

pub struct ToCriticalErr<T>(pub T);

impl<T, E> ToCriticalErr<Result<T, E>>
where
    E: SupportCriticalError,
{
    #[inline]
    // We need to allow for dead code at this point because
    // the Rust compiler thinks this function is unused even
    // though it acts as the specialized case for detection.
    #[allow(dead_code)]
    pub fn try_to_critical_error(self) -> Result<Result<T, E>, CriticalError> {
        match self.0 {
            Ok(result) => Ok(Ok(result)),
            Err(error) => {
                match error.try_to_critical() {
                    Ok(critical_error) => Err(critical_error),
                    Err(not_critical) => Ok(Err(not_critical)),
                }
            }
        }
    }
}

pub trait ToCriticalErrFallback<T> {
    fn try_to_critical_error(self) -> Result<T, CriticalError>;
}
impl<T> ToCriticalErrFallback<T> for ToCriticalErr<T> {
    #[inline]
    fn try_to_critical_error(self) -> Result<T, CriticalError> {
        Ok(self.0)
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! to_critical_error {
    ( $e:expr $(,)? ) => {{
        #[allow(unused_imports)]
        use $crate::substrate::ToCriticalErrFallback;
        $crate::substrate::ToCriticalErr($e).try_to_critical_error()
    }};
}

#[cfg(test)]
mod tests {
    use crate::substrate::{
        CriticalError,
        SupportCriticalError,
    };

    #[test]
    fn to_critical_error_works_if_trait_implemented() {
        #[derive(PartialEq, Eq, Debug)]
        enum Error {
            NonCritical,
            Critical(CriticalError),
        }

        impl SupportCriticalError for Error {
            fn try_to_critical(self) -> Result<CriticalError, Self> {
                match self {
                    Error::Critical(critical) => Ok(critical),
                    _ => Err(self),
                }
            }
        }

        let error: Result<(), _> = Err(Error::Critical(CriticalError::BadOrigin));
        assert_eq!(to_critical_error!(error), Err(CriticalError::BadOrigin));

        let error: Result<(), _> = Err(Error::NonCritical);
        assert_eq!(to_critical_error!(error), Ok(Err(Error::NonCritical)));
    }

    #[test]
    fn to_critical_error_works_if_trait_is_not_implemented() {
        #[derive(PartialEq, Eq, Debug)]
        enum Error {
            NonCritical,
            Critical(CriticalError),
        }

        let error: Result<(), _> = Err(Error::Critical(CriticalError::BadOrigin));
        assert_eq!(
            to_critical_error!(error),
            Ok(Err(Error::Critical(CriticalError::BadOrigin)))
        );

        let error: Result<(), _> = Err(Error::NonCritical);
        assert_eq!(to_critical_error!(error), Ok(Err(Error::NonCritical)));
    }

    #[test]
    fn to_critical_error_works_with_without_result() {
        let result = ();
        assert_eq!(to_critical_error!(result), Ok(()));
    }
}
