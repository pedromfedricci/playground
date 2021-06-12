#![feature(iter_map_while)]
#![feature(trait_alias)]
#![feature(dropck_eyepatch)]
#![feature(option_result_unwrap_unchecked)]
#![feature(ptr_internals, allocator_api)]

pub mod arc;
pub mod binary_tree;
pub mod drop;
pub mod dropck;
pub mod from_into;
pub mod into_iter;
pub mod io;
pub mod iterators;
pub mod lifetimes;
pub mod linked_list;
pub mod rc;
pub mod regex;
pub mod string;
pub mod variance;

#[macro_use]
pub mod macros;
