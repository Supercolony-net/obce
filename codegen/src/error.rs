use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse2,
    Error,
    ItemEnum,
    Meta,
    NestedMeta,
};

use crate::{
    format_err_spanned,
    types::AttributeArgs,
};

pub struct ChainExtensionError;

impl ChainExtensionError {
    pub fn generate(_attrs: TokenStream, input: TokenStream) -> Result<TokenStream, Error> {
        let mut enum_item: ItemEnum = parse2(input)?;
        let ident = enum_item.ident.clone();
        let (impl_generics, ty_generics, where_clause) = enum_item.generics.split_for_impl();

        let mut critical_variant = None;

        for variant in enum_item.variants.iter_mut() {
            let (obce_attrs, mut other_attrs) = variant
                .attrs
                .iter()
                .cloned()
                .partition::<Vec<_>, _>(|attr| attr.path.is_ident("obce"));

            for attr in obce_attrs {
                let args: AttributeArgs = attr.parse_args()?;

                for arg in args.iter() {
                    let critical = matches!(
                        arg,
                        NestedMeta::Meta(Meta::Path(value))
                        if value.is_ident("critical")
                    );

                    if critical {
                        other_attrs.push(syn::parse_quote! {
                            #[cfg(feature = "substrate")]
                        });

                        let variant = &variant.ident;

                        let previous_critical_variant = critical_variant.replace(quote! {
                            #[cfg(feature = "substrate")]
                            impl #impl_generics ::obce::substrate::SupportCriticalError for #ident #ty_generics #where_clause {
                                fn try_to_critical(self) -> Result<::obce::substrate::CriticalError, Self> {
                                    match self {
                                        Self::#variant(error) => Ok(error),
                                        _ => Err(self)
                                    }
                                }
                            }
                        });

                        if let Some(variant) = previous_critical_variant {
                            return Err(format_err_spanned!(
                                variant,
                                "only one enum variant can be marked as `#[obce(critical)]`",
                            ))
                        }
                    }
                }
            }

            variant.attrs = other_attrs;
        }

        Ok(quote! {
            #[derive(Debug, Copy, Clone, PartialEq, Eq, ::scale::Encode, ::scale::Decode)]
            #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
            #enum_item

            #critical_variant
        })
    }
}
