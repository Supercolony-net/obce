// Copyright (c) 2012-2022 727.Ventures
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

use syn::{
    ext::IdentExt,
    parse::{
        Parse,
        ParseStream,
    },
    NestedMeta,
};

pub struct AttributeArgs(Vec<NestedMeta>);

impl Parse for AttributeArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attrs = Vec::new();
        while input.peek(syn::Ident::peek_any) {
            attrs.push(input.parse()?);
            if input.is_empty() {
                break
            }
            let _: syn::token::Comma = input.parse()?;
        }
        Ok(AttributeArgs { 0: attrs })
    }
}

impl std::ops::Deref for AttributeArgs {
    type Target = Vec<NestedMeta>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for AttributeArgs {
    fn deref_mut(&mut self) -> &mut Vec<NestedMeta> {
        &mut self.0
    }
}
