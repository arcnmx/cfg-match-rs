use cfg_match::*;

cfg_match! {
    foo =>
        /// meta
        fn item() -> bool { false }
    any(bar, test) => {
        fn item() -> bool { true }
    }
    _ => fn item() -> bool { false }
}

#[test]
fn smoke_test() {
    assert!(item());
}

cfg_match! {
    foo => fn f() -> bool { false }
    _ => fn f() -> bool { true }
}

#[test]
fn fallback() {
    assert!(f());
}

#[test]
fn expr() {
    let f = cfg_match! {
        foo => ({ false }),
        test => ({
            println!("whee");
            true
        }),
        _ => false,
    };
    assert!(f);
}

#[test]
fn no_fallback() {
    let f = cfg_match! {
        foo => ({ false }),
        test => ({
            println!("whee");
            true
        }),
    };
    assert!(f);
}

cfg_match! {
    #[cfg(foo)]
    /// whee
    fn c() -> bool { false }

    #[cfg('do)]
    /// whee
    fn c2() -> bool { false }

    #[cfg(_)]
    /// fallback
    fn c() -> bool { true }

    #[cfg(_)]
    /// multiple fallbacks
    fn c2() -> bool { true }
}

#[test]
fn cascade() {
    assert!(c());
    assert!(c2());
}
