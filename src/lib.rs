#![deny(missing_docs)]
#![doc(html_root_url = "http://docs.rs/cfg-match/0.2.1")]
#![no_std]

//! # cfg_match!
///
/// `cfg_match!` provides a more ergonomic approach to chaining conditionals,
/// like the similar [cfg-if](https://github.com/alexcrichton/cfg-if) crate. In
/// addition to items, `cfg_match!` can also be used for expressions (though a
/// block will require parenthesis wrapping like `=> ({ ... })`).

/// Compile-time conditionals
///
/// The macro stops at the first matching branch, just like a traditional match:
///
/// ```rust
/// # use cfg_match::cfg_match;
///
/// cfg_match! {
///     feature = "foo" => {
///         fn bar() {
///             println!("have foo");
///         }
///     }
///     _ =>
///         fn bar() {
///             println!("no foo :(");
///         }
/// }
/// ```
///
/// Alternatively, the above can be written as:
///
/// ```rust
/// # use cfg_match::cfg_match;
///
/// cfg_match! {
///     #[cfg(feature = "foo")]
///     /// Does a thing because of foo.
///     fn bar() {
///         println!("have foo");
///     }
///
///     #[cfg(_)]
///     fn bar() {
///         println!("no foo :(");
///     }
/// }
/// ```
#[macro_export]
macro_rules! cfg_match {
    (_ => { $($i:item)* }) => {
        $($i)*
    };
    (_ => $(#[$m:meta])+ $i:item) => {
        $(#[$m])*
        $i
    };
    (_ => $e:expr$(,)?) => {
        $e
    };
    (_ => $i:item) => {
        $i
    };
    ($cfg:meta => { $($i:item)* } $($t:tt)*) => {
        $(#[cfg($cfg)] $i)*
        #[cfg(not($cfg))] $crate::cfg_match! { $($t)* }
    };
    ($cfg:meta => $(#[$m:meta])+ $i:item $($t:tt)*) => {
        #[cfg($cfg)] $(#[$m])* $i
        #[cfg(not($cfg))] $crate::cfg_match! { $($t)* }
    };
    ($cfg:meta => $e:expr, $($t:tt)*) => {
        match () {
            #[cfg($cfg)] _ => $e,
            #[cfg(not($cfg))] _ => $crate::cfg_match!($($t)*)
        }
    };
    ($cfg:meta => $i:item $($t:tt)*) => {
        #[cfg($cfg)] $i
        #[cfg(not($cfg))] $crate::cfg_match! { $($t)* }
    };
    (#[cfg(_)] $i:item $($t:tt)*) => {
        $i
        $crate::cfg_match! { @not(_) $($t)* }
    };
    (#[cfg($cfg:meta)] $i:item $($t:tt)*) => {
        #[cfg($cfg)] $i
        $crate::cfg_match! { @not($cfg) $($t)* }
    };
    (@not(_) #[cfg('do)] $i:item $($t:tt)*) => {
        $i
        $crate::cfg_match! { @not(_) $($t)* }
    };
    (@not(_) #[cfg(_)] $i:item $($t:tt)*) => {
        $i
        $crate::cfg_match! { @not(_) $($t)* }
    };
    (@not(_)) => { };
    (@not($not:meta)) => {
        #[cfg(not($not))] $crate::cfg_match! { }
    };
    (@not($cfg:meta) #[cfg('do)] $i:item $($t:tt)*) => {
        #[cfg($cfg)] $i
        $crate::cfg_match! { @not($cfg) $($t)* }
    };
    (@not($not:meta) $($t:tt)*) => {
        #[cfg(not($not))] $crate::cfg_match! { $($t)* }
    };
    () => {
        compile_error! { "Unsupported configuration" }
    }
}
