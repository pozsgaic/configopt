use crate::configopt_type::parse::{ParsedField, StructOptTy};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Ident};

pub fn take(fields: &[ParsedField], other: &Ident) -> TokenStream {
    fields
        .iter()
        .map(|field| {
            let field_ident = field.ident();
            let span = field.span();
            let self_field = quote! {self.#field_ident};
            let other_field = quote! {#other.#field_ident};
            if field.flatten() {
                quote_spanned! {span=>
                    #self_field.take(&mut #other_field);
                }
            } else {
                match field.structopt_ty() {
                    StructOptTy::Bool => quote_spanned! {span=>
                        // TODO: Should it be wrapped in `Option`?
                        #self_field = #other_field;
                    },
                    StructOptTy::Vec => quote_spanned! {span=>
                        // TODO: Should it be wrapped in `Option`?
                        if !#other_field.is_empty() {
                            ::std::mem::swap(&mut #self_field, &mut #other_field);
                        }
                    },
                    StructOptTy::Option
                    | StructOptTy::OptionOption
                    | StructOptTy::OptionVec
                    | StructOptTy::Other => {
                        quote_spanned! {span=>
                            if #other_field.is_some() {
                                #self_field = #other_field.take();
                            }
                        }
                    }
                }
            }
        })
        .collect()
}

pub fn patch(fields: &[ParsedField], other: &Ident) -> TokenStream {
    fields
        .iter()
        .map(|field| {
            let field_ident = field.ident();
            let span = field.span();
            let self_field = quote! {self.#field_ident};
            let other_field = quote! {#other.#field_ident};
            if field.flatten() {
                quote_spanned! {span=>
                    #self_field.patch(&mut #other_field);
                }
            } else {
                match field.structopt_ty() {
                    StructOptTy::Bool => quote_spanned! {span=>
                        // TODO: Should it be wrapped in `Option`?
                        // #self_field = #other_field;
                    },
                    StructOptTy::Vec => quote_spanned! {span=>
                        // TODO: Should it be wrapped in `Option`?
                        if #self_field.is_empty() {
                            ::std::mem::swap(&mut #self_field, &mut #other_field);
                        }
                    },
                    StructOptTy::Option
                    | StructOptTy::OptionOption
                    | StructOptTy::OptionVec
                    | StructOptTy::Other => {
                        quote_spanned! {span=>
                            if #self_field.is_none() {
                                #self_field = #other_field.take();
                            }
                        }
                    }
                }
            }
        })
        .collect()
}

// fn merge(fields: &[ParsedField]) -> TokenStream {
//     fields
//         .iter()
//         .map(|field| {
//             let ident = field.ident();
//             let span = field.span();
//             if field.flatten() {
//                 quote_spanned! {span=>
//                     if let Some(mut val) = self.#ident.take() {
//                         val.merge(&mut other.#ident)
//                     }
//                 }
//             } else {
//                 quote_spanned! {span=>
//                     if let Some(val) = self.#ident.take() {
//                         other.#ident = val;
//                     }
//                 }
//             }
//         })
//         .collect()
// }

// fn clear(fields: &[ParsedField]) -> TokenStream {
//     fields
//         .iter()
//         .map(|field| {
//             let ident = field.ident();
//             let span = field.span();
//             quote_spanned! {span=>
//                 self.#ident = None;
//             }
//         })
//         .collect()
// }

// fn is_empty(fields: &[ParsedField]) -> TokenStream {
//     let field_tokens = fields.iter().map(|field| {
//         let ident = field.ident();
//         let span = field.span();
//         quote_spanned! {span=>
//             self.#ident.is_none()
//         }
//     });
//     quote! {
//         #(#field_tokens)&&*
//     }
// }

// fn is_complete(fields: &[ParsedField]) -> TokenStream {
//     let field_tokens = fields.iter().map(|field| {
//         let ident = field.ident();
//         let span = field.span();
//         if field.flatten() {
//             quote_spanned! {span=>
//                 self.#ident.as_ref().map_or(false, |val| val.is_complete())
//             }
//         } else {
//             quote_spanned! {span=>
//                 self.#ident.is_some()
//             }
//         }
//     });
//     quote! {
//         #(#field_tokens)&&*
//     }
// }

// fn from(fields: &[ParsedField]) -> TokenStream {
//     let field_tokens = fields.iter().map(|field| {
//         let ident = field.ident();
//         let span = field.span();
//         if field.flatten() {
//             quote_spanned! {span=>
//                 #ident: Some(other.#ident.into()),
//             }
//         } else {
//             quote_spanned! {span=>
//                 #ident: Some(other.#ident),
//             }
//         }
//     });
//     quote! {
//         Self {
//             #(#field_tokens)*
//         }
//     }
// }

// fn try_from(fields: &[ParsedField]) -> TokenStream {
//     let field_tokens = fields.iter().map(|field| {
//         let ident = field.ident();
//         let span = field.span();
//         // We check upfront if the type `is_complete` so all these `unwrap`'s are ok
//         if field.flatten() {
//             quote_spanned! {span=>
//                 #ident: ::std::convert::TryInto::try_into(partial.#ident.unwrap()).unwrap(),
//             }
//         } else {
//             quote_spanned! {span=>
//                 #ident: partial.#ident.unwrap(),
//             }
//         }
//     });
//     let create = quote! {
//         Self {
//             #(#field_tokens)*
//         }
//     };
//     quote! {
//         if !partial.is_complete() {
//             return Err(partial);
//         }
//         Ok(#create)
//     }
// }