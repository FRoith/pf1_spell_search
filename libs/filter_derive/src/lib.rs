extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Data, Fields};

#[proc_macro_derive(FilterReprMacro, attributes(name))]
pub fn filter_repr_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_filter_repr(&ast)
}

fn impl_filter_repr(input: &syn::DeriveInput) -> TokenStream {
    // get enum name
    let name = &input.ident;
    let data = &input.data;

    let mut variant_checker_functions;

    // data is of type syn::Data
    // See https://doc.servo.org/syn/enum.Data.html
    return match data {
        // Only if data is an enum, we do parsing
        Data::Enum(data_enum) => {
            // data_enum is of type syn::DataEnum
            // https://doc.servo.org/syn/struct.DataEnum.html

            variant_checker_functions = TokenStream2::new();

            // Iterate over enum variants
            // `variants` if of type `Punctuated` which implements IntoIterator
            //
            // https://doc.servo.org/syn/punctuated/struct.Punctuated.html
            // https://doc.servo.org/syn/struct.Variant.html
            let idents: Vec<&proc_macro2::Ident> = data_enum
                .variants
                .iter()
                .map(|variant| &variant.ident)
                .collect();

            let othercases: Vec<proc_macro2::TokenStream> = data_enum
                .variants
                .iter()
                .map(|variant| {
                    let name_ident = &variant.ident;
                    let _fields = match &variant.fields {
                        Fields::Unnamed(_) => quote_spanned! {variant.span()=> (..) },
                        Fields::Unit => quote_spanned! { variant.span()=> },
                        Fields::Named(_) => quote_spanned! {variant.span()=> {..} },
                    };

                    let namestr = &variant
                        .attrs
                        .iter()
                        .filter_map(|a| a.meta.require_name_value().ok())
                        .filter(|a| a.path.is_ident("name"))
                        .filter_map(|a| match &a.value {
                            syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
                                syn::Lit::Str(lit_str) => Some(lit_str.value()),
                                _ => None,
                            },
                            _ => None,
                        })
                        .next()
                        .unwrap_or(
                            variant
                                .ident
                                .to_string()
                                .trim_start_matches('_')
                                .replace("_", " ")
                                .to_string(),
                        );
                    quote! {
                        Self::#name_ident (filter_state) => filter_state.test(spell, #namestr),
                    }
                })
                .collect();

            let othercases2: Vec<proc_macro2::TokenStream> = data_enum
                .variants
                .iter()
                .map(|variant| {
                    let name_ident = &variant.ident;
                    let _fields = match &variant.fields {
                        Fields::Unnamed(_) => quote_spanned! {variant.span()=> (..) },
                        Fields::Unit => quote_spanned! { variant.span()=> },
                        Fields::Named(_) => quote_spanned! {variant.span()=> {..} },
                    };

                    let namestr = &variant
                        .attrs
                        .iter()
                        .filter_map(|a| a.meta.require_name_value().ok())
                        .filter(|a| a.path.is_ident("name"))
                        .filter_map(|a| match &a.value {
                            syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
                                syn::Lit::Str(lit_str) => Some(lit_str.value()),
                                _ => None,
                            },
                            _ => None,
                        })
                        .next()
                        .unwrap_or(
                            variant
                                .ident
                                .to_string()
                                .trim_start_matches('_')
                                .replace("_", " ")
                                .to_string(),
                        );
                    quote! {
                        Self::#name_ident (filter_state) => filter_state.test_exact(spell, #namestr),
                    }
                })
                .collect();

            let cases: Vec<proc_macro2::TokenStream> = data_enum
                .variants
                .iter()
                .map(|variant| {
                    let name_ident = &variant.ident;
                    let _fields = match &variant.fields {
                        Fields::Unnamed(_) => quote_spanned! {variant.span()=> (..) },
                        Fields::Unit => quote_spanned! { variant.span()=> },
                        Fields::Named(_) => quote_spanned! {variant.span()=> {..} },
                    };
                    let namestr = &variant
                        .attrs
                        .iter()
                        .filter_map(|a| a.meta.require_name_value().ok())
                        .filter(|a| a.path.is_ident("name"))
                        .filter_map(|a| match &a.value {
                            syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
                                syn::Lit::Str(lit_str) => Some(lit_str.value()),
                                _ => None,
                            },
                            _ => None,
                        })
                        .next()
                        .unwrap_or(
                            variant
                                .ident
                                .to_string()
                                .trim_start_matches('_')
                                .replace("_", " ")
                                .to_string(),
                        );
                    quote! {
                        Self::#name_ident (FilterState::None) => {
                            let resp = ui.add(egui::Button::new(#namestr));
                            if resp.clicked() {
                                Self::#name_ident (FilterState::Positive)
                            } else if resp.secondary_clicked() {
                                Self::#name_ident (FilterState::Negative)
                            } else {
                                self.clone()
                            }
                        },
                        Self::#name_ident (state) => {
                            let resp = ui.add(egui::Button::new(#namestr).fill(state.get_color()));
                            if resp.clicked() {
                                Self::#name_ident (state.n())
                            } else if resp.secondary_clicked() {
                                Self::#name_ident (state.p())
                            } else {
                                self.clone()
                            }
                        },
                    }
                })
                .collect();

            // Here we construct the function for the current variant
            variant_checker_functions.extend(quote! {
                fn create_btn(&self, ui: &mut egui::Ui) -> Self {
                    match self {
                        #(#cases)*
                    }
                }
            });

            variant_checker_functions.extend(quote! {
                fn test(&self, spell: &str) -> bool {
                    match self {
                        #(#othercases)*
                    }
                }
            });

            variant_checker_functions.extend(quote! {
                fn test_exact(&self, spell: &str) -> bool {
                    match self {
                        #(#othercases2)*
                    }
                }
            });

            variant_checker_functions.extend(quote! {
                fn get_all() -> Vec<Self> {
                    [
                        #(Self::#idents (FilterState::None),)*
                    ].into()
                }
            });

            variant_checker_functions.extend(quote! {
                fn some_filter(&self) -> bool {
                    match self {
                        #(Self::#idents (FilterState::None) => false,)*
                        _ => true,
                    }
                }
            });

            let expanded = quote! {
                impl FilterRepr for #name {
                    // variant_checker_functions gets replaced by all the functions
                    // that were constructed above
                    #variant_checker_functions
                }
            };

            TokenStream::from(expanded)
        }
        _ => panic!("FilterRepr is only implemented for enums"),
    };
}
