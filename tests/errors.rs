use obce::{
    substrate::CriticalError,
    to_critical_error,
};
use scale::Encode;

fn assert_encode_holds<T: Encode>(_: T) {}

#[test]
fn error_macro_works() {
    #[obce::error]
    pub enum Error {
        SimpleError,
        AnotherSimpleError(u32),
    }

    assert_encode_holds(Error::SimpleError);
}

#[test]
fn error_macro_with_generics_works() {
    #[obce::error]
    pub enum Error<A, B> {
        SimpleError(A),
        AnotherSimpleError(B),
    }

    assert_encode_holds(Error::<_, u32>::SimpleError(123));
}

#[test]
fn error_macro_with_critical_works() {
    #[obce::error]
    pub enum Error<T> {
        NonCritical(T),

        #[obce(critical)]
        Critical(CriticalError),
    }

    let error: Result<(), _> = Err(Error::<u32>::Critical(CriticalError::BadOrigin));
    assert_eq!(to_critical_error!(error), Err(CriticalError::BadOrigin));

    let error: Result<(), _> = Err(Error::NonCritical(123));
    assert_eq!(to_critical_error!(error), Ok(Err(Error::NonCritical(123))));
}
