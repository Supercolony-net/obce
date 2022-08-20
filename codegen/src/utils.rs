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

#[macro_export]
macro_rules! format_err_spanned {
    ($tokens:expr, $($msg:tt)*) => {
        ::syn::Error::new_spanned(
            &$tokens,
            format_args!($($msg)*)
        )
    }
}

pub fn into_u16(ident: &syn::Ident) -> u16 {
    let mut output = [0; 32];
    blake2b_256(ident.to_string().as_bytes(), &mut output);
    u16::from_le_bytes([output[0], output[1]])
}

pub fn into_u32(ident: &syn::Ident) -> u32 {
    let mut output = [0; 32];
    blake2b_256(ident.to_string().as_bytes(), &mut output);
    u32::from_le_bytes([output[0], output[1], output[2], output[3]])
}

pub fn blake2b_256(input: &[u8], output: &mut [u8; 32]) {
    use ::blake2::digest::{consts::U32, Digest as _};

    type Blake2b256 = blake2::Blake2b<U32>;

    let mut blake2 = Blake2b256::new();
    blake2.update(input);
    let result = blake2.finalize();
    output.copy_from_slice(&result);
}
