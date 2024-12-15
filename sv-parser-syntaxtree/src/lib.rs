#![recursion_limit = "256"]
#![allow(
    clippy::module_inception,
    clippy::large_enum_variant,
    clippy::type_complexity
)]

pub mod any_node;
pub mod behavioral_statements;
pub mod declarations;
pub mod expressions;
pub mod general;
pub mod instantiations;
pub mod preprocessor;
pub mod primitive_instances;
pub mod source_text;
pub mod special_node;
pub mod specify_section;
pub mod udp_declaration_and_instantiation;
pub use any_node::*;
pub use behavioral_statements::*;
pub use declarations::*;
pub use expressions::*;
pub use general::*;
pub use instantiations::*;
pub use preprocessor::*;
pub use primitive_instances::*;
pub use source_text::*;
pub use special_node::*;
pub use specify_section::*;
pub use udp_declaration_and_instantiation::*;

pub(crate) use sv_parser_macros::*;

// -----------------------------------------------------------------------------

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Locate {
    pub offset: usize,
    pub line: u32,
    pub len: usize,
    #[cfg(feature = "quickcheck")]
    pub val: test::ImmutableString,
}

impl Locate {
    pub fn str<'a, 'b>(&'a self, s: &'b str) -> &'b str {
        #[cfg(feature = "quickcheck")]
        {
            if self.val.len() > 0 {
                return unsafe { std::mem::transmute::<&'a str, &'b str>(&self.val) }
            }
        }

        &s[self.offset..self.offset + self.len]
    }
}

// -----------------------------------------------------------------------------

pub trait Node<'a> {
    fn next(&'a self) -> RefNodes<'a>;
}

impl<'a> Node<'a> for Locate {
    fn next(&'a self) -> RefNodes<'a> {
        vec![].into()
    }
}

impl<'a> IntoIterator for &'a Locate {
    type Item = RefNode<'a>;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let nodes: RefNodes = self.into();
        Iter { next: nodes }
    }
}

// -----------------------------------------------------------------------------

#[cfg(feature = "quickcheck")]
mod test {
    use std::{
        ops::Deref,
        sync::atomic::{AtomicUsize, Ordering},
    };
 
    #[derive(Copy)]
    pub struct ImmutableString {
        ptr: *mut u8,
        len: usize,
        capacity: usize,
        count: *mut usize,
    }

    impl From<&str> for ImmutableString {
        fn from(s: &str) -> Self {
            let mut x = s.to_owned();
            let s: ImmutableString = Self {
                ptr: x.as_mut_ptr(),
                len: x.len(),
                capacity: x.capacity(),
                count: Box::into_raw(Box::new(1)),
            };
            std::mem::forget(x);
            s
        }
    }

    impl Deref for ImmutableString {
        type Target = str;
        fn deref(&self) -> &Self::Target {
            unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.ptr, self.len)) }
        }
    }

    impl std::fmt::Debug for ImmutableString {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let x: &str = self;
            x.fmt(f)
        }
    }

    impl PartialEq for ImmutableString {
        fn eq(&self, other: &Self) -> bool {
            let x: &str = self;
            let y: &str = other;
            x == y
        }
    }

    impl Clone for ImmutableString {
        fn clone(&self) -> Self {
            let count = unsafe { AtomicUsize::from_ptr(self.count) };
            count.fetch_add(1, Ordering::Relaxed);
            Self {
                ptr: self.ptr,
                len: self.len,
                capacity: self.capacity,
                count: self.count,
            }
        }
    }

    impl std::default::Default for ImmutableString {
        fn default() -> Self {
            Self::from("")
        }
    }

    // impl Drop for ImmutableString {
    //     fn drop(&mut self) {
    //         let count = unsafe{AtomicUsize::from_ptr(self.count)};
    //         if count.fetch_sub(1, Ordering::Relaxed) == 0 {
    //             unsafe {
    //                 drop(String::from_raw_parts(self.ptr, self.len, self.capacity));
    //                 drop(Box::from_raw(self.count));
    //             }

    //         }
    //     }
    // }
}