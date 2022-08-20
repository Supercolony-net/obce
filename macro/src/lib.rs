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

#![cfg_attr(not(feature = "std"), no_std)]

use proc_macro::TokenStream;

use obce_codegen::{
    ChainExtensionDefinition,
    ChainExtensionImplementation,
};

// TODO: Add comments with examples
#[proc_macro_attribute]
pub fn definition(attrs: TokenStream, trait_item: TokenStream) -> TokenStream {
    match ChainExtensionDefinition::generate(attrs.into(), trait_item.into()) {
        Ok(traits) => traits.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

// TODO: Add comments with examples
#[proc_macro_attribute]
pub fn implementation(attrs: TokenStream, impl_item: TokenStream) -> TokenStream {
    match ChainExtensionImplementation::generate(attrs.into(), impl_item.into()) {
        Ok(impls) => impls.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
