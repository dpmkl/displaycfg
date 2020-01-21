use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Attribute, Data, DataStruct, DeriveInput, Error, Fields, Meta, Result};

pub fn derive(input: &DeriveInput) -> Result<TokenStream> {
    let impls = match &input.data {
        Data::Struct(data) => impl_struct(input, data),
        _ => Err(Error::new_spanned(
            input,
            "Unions and enums are not supported",
        )),
    }?;

    let helpers = specialization();
    let dummy_const = format_ident!("_DERIVE_Display_FOR_{}", input.ident);
    Ok(quote! {
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const #dummy_const: () = {
            #helpers
            #impls
        };
    })
}

fn specialization() -> TokenStream {
    quote! {
        trait DisplayToDisplayCfg {
            fn __displaycfg_display(&self) -> Self;
        }

        impl<T: core::fmt::Display> DisplayToDisplayCfg for &T {
            fn __displaycfg_display(&self) -> Self {
                self
            }
        }

        trait PathToDisplayCfg {
            fn __displaycfg_display(&self) -> std::path::Display<'_>;
        }

        impl PathToDisplayCfg for std::path::Path {
            fn __displaycfg_display(&self) -> std::path::Display<'_> {
                self.display()
            }
        }

        impl PathToDisplayCfg for std::path::PathBuf {
            fn __displaycfg_display(&self) -> std::path::Display<'_> {
                self.display()
            }
        }
    }
}

fn get_docs(attrs: &[Attribute]) -> Result<TokenStream> {
    let mut res = TokenStream::new();
    for attr in attrs {
        if attr.path.is_ident("doc") {
            let meta = attr.parse_meta()?;
            let lit = match meta {
                Meta::NameValue(syn::MetaNameValue {
                    lit: syn::Lit::Str(lit),
                    ..
                }) => lit,
                _ => unimplemented!(),
            }
            .value();
            res.extend(quote! {
                write!(formatter, "#  {}\n", #lit).unwrap();
            });
        }
    }
    res.extend(quote! { write!(formatter, "\n").unwrap();});
    Ok(res)
}

fn impl_struct(input: &DeriveInput, data: &DataStruct) -> Result<TokenStream> {
    let ty = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    #[allow(unused_variables)]
    let header = quote! {
        write!(formatter, "### {}\n", stringify!(#ty)).unwrap();
    };
    let docs = get_docs(&input.attrs)?;

    let fields = match &data.fields {
        Fields::Named(fields) => {
            let mut res = TokenStream::new();
            for field in fields.named.iter() {
                let ty_name = &field.ty;
                let name = &field.ident.as_ref().unwrap();
                res.extend(quote! {
                    write!(formatter, "# name: '{}'\n", stringify!(#name)).unwrap();
                    write!(formatter, "# type: '{}'\n", stringify!(#ty_name)).unwrap();
                    write!(formatter, "# desc:\n").unwrap();
                });
                res.extend(get_docs(&field.attrs)?);
            }
            res
        }
        Fields::Unit | Fields::Unnamed(_) => quote!(_),
    };

    Ok(quote! {
        impl #impl_generics core::fmt::Display for #ty #ty_generics #where_clause {
            fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                #header
                #docs
                #fields
                Ok(())
            }
        }
    })
}
