use crate::configopt_type::parse::{FieldType, ParsedField, ParsedVariant, StructOptTy};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Ident};

pub fn take_for_struct(fields: &[ParsedField], other: &Ident) -> TokenStream {
    fields
        .iter()
        .map(|field| {
            let field_ident = field.ident();
            let span = field.span();
            let self_field = quote! {self.#field_ident};
            let other_field = quote! {#other.#field_ident};
            if field.structopt_flatten() {
                quote_spanned! {span=>
                    #self_field.take(&mut #other_field);
                }
            } else {
                match field.structopt_ty() {
                    StructOptTy::Vec => quote_spanned! {span=>
                        if !#other_field.is_empty() {
                            ::std::mem::swap(&mut #self_field, &mut #other_field);
                        }
                    },
                    StructOptTy::Bool
                    | StructOptTy::Option
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

pub fn patch_for_struct(fields: &[ParsedField], other: &Ident) -> TokenStream {
    fields
        .iter()
        .map(|field| {
            let field_ident = field.ident();
            let span = field.span();
            let self_field = quote! {self.#field_ident};
            let other_field = quote! {#other.#field_ident};
            if field.structopt_flatten() {
                quote_spanned! {span=>
                    #self_field.patch(&mut #other_field);
                }
            } else {
                match field.structopt_ty() {
                    StructOptTy::Vec => quote_spanned! {span=>
                        if #self_field.is_empty() {
                            ::std::mem::swap(&mut #self_field, &mut #other_field);
                        }
                    },
                    StructOptTy::Bool
                    | StructOptTy::Option
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

pub fn take_for_for_struct(fields: &[ParsedField], other: &Ident) -> TokenStream {
    fields
        .iter()
        .map(|field| {
            let field_ident = field.ident();
            let span = field.span();
            let self_field = quote! {self.#field_ident};
            let other_field = quote! {#other.#field_ident};
            if field.structopt_flatten() {
                quote_spanned! {span=>
                    #self_field.take_for(&mut #other_field);
                }
            } else if field.subcommand() {
                quote_spanned! {span=>
                    // TODO: handle subcommands
                }
            } else {
                match field.structopt_ty() {
                    StructOptTy::Vec => quote_spanned! {span=>
                        if !#self_field.is_empty() {
                            ::std::mem::swap(&mut #other_field, &mut #self_field);
                        }
                    },
                    StructOptTy::Option | StructOptTy::OptionOption | StructOptTy::OptionVec => {
                        quote_spanned! {span=>
                            if #self_field.is_some() {
                                #other_field = #self_field.take();
                            }
                        }
                    }
                    StructOptTy::Bool | StructOptTy::Other => {
                        quote_spanned! {span=>
                            if let Some(value) = #self_field.take() {
                                #other_field = value;
                            }
                        }
                    }
                }
            }
        })
        .collect()
}

pub fn patch_for_for_struct(fields: &[ParsedField], other: &Ident) -> TokenStream {
    fields
        .iter()
        .map(|field| {
            let field_ident = field.ident();
            let span = field.span();
            let self_field = quote! {self.#field_ident};
            let other_field = quote! {#other.#field_ident};
            if field.structopt_flatten() {
                quote_spanned! {span=>
                    #self_field.patch_for(&mut #other_field);
                }
            } else if field.subcommand() {
                quote_spanned! {span=>
                    // TODO: handle subcommands
                }
            } else {
                match field.structopt_ty() {
                    StructOptTy::Vec => quote_spanned! {span=>
                        if #other_field.is_empty() {
                            ::std::mem::swap(&mut #other_field, &mut #self_field);
                        }
                    },
                    StructOptTy::Option | StructOptTy::OptionOption | StructOptTy::OptionVec => {
                        quote_spanned! {span=>
                            if #other_field.is_none() {
                                #other_field = #self_field.take();
                            }
                        }
                    }
                    StructOptTy::Bool | StructOptTy::Other => {
                        quote_spanned! {span=>}
                    }
                }
            }
        })
        .collect()
}

pub fn is_empty_for_struct(fields: &[ParsedField]) -> TokenStream {
    let field_tokens = fields.iter().map(|field| {
        let field_ident = field.ident();
        let span = field.span();
        let self_field = quote! {self.#field_ident};
        if field.structopt_flatten() {
            quote_spanned! {span=>
                #self_field.is_empty()
            }
        } else if field.subcommand() {
            quote_spanned! {span=>
                #self_field.is_none()
            }
        } else {
            match field.structopt_ty() {
                StructOptTy::Vec => quote_spanned! {span=>
                    // TODO: how to handle vectors
                    // #self_field.is_empty()
                },
                _ => {
                    quote_spanned! {span=>
                        #self_field.is_none()
                    }
                }
            }
        }
    });
    let field_tokens = field_tokens.filter(|p| !p.is_empty());
    quote! {
        #(#field_tokens)&&*
    }
}

pub fn is_complete_for_struct(fields: &[ParsedField]) -> TokenStream {
    let field_tokens = fields.iter().map(|field| {
        let field_ident = field.ident();
        let span = field.span();
        let self_field = quote! {self.#field_ident};
        if field.structopt_flatten() {
            quote_spanned! {span=>
                #self_field.is_complete()
            }
        } else if field.subcommand() {
            quote_spanned! {span=>
                #self_field.as_ref().map_or(false, |val| val.is_complete())
            }
        } else {
            match field.structopt_ty() {
                StructOptTy::Vec => quote_spanned! {span=>
                    // TODO: how to handle vectors
                    // !#self_field.is_empty()
                },
                _ => {
                    quote_spanned! {span=>
                        #self_field.is_some()
                    }
                }
            }
        }
    });
    let field_tokens = field_tokens.filter(|p| !p.is_empty());
    quote! {
        #(#field_tokens)&&*
    }
}

pub fn from_for_struct(fields: &[ParsedField], other: &Ident) -> TokenStream {
    let field_tokens = fields.iter().map(|field| {
        let field_ident = field.ident();
        let span = field.span();
        let other_field = quote! {#other.#field_ident};
        if field.structopt_flatten() {
            quote_spanned! {span=>
                #field_ident: #other_field.into(),
            }
        } else if field.subcommand() {
            quote_spanned! {span=>
                #field_ident: Some(#other_field.into()),
            }
        } else {
            match field.structopt_ty() {
                StructOptTy::Bool | StructOptTy::Other => quote_spanned! {span=>
                    #field_ident: Some(#other_field),
                },
                _ => {
                    quote_spanned! {span=>
                        #field_ident: #other_field,
                    }
                }
            }
        }
    });
    quote! {
        Self {
            #(#field_tokens)*
        }
    }
}

pub fn try_from_for_struct(fields: &[ParsedField]) -> TokenStream {
    let field_tokens = fields.iter().map(|field| {
        let field_ident = field.ident();
        let span = field.span();
        let self_field = quote! {configopt.#field_ident};
        // We check upfront if the type `is_complete` so all these `unwrap`'s are ok
        if field.structopt_flatten() {
            quote_spanned! {span=>
                #field_ident: ::std::convert::TryInto::try_into(#self_field).unwrap(),
            }
        } else if field.subcommand() {
            quote_spanned! {span=>
                #field_ident: ::std::convert::TryInto::try_into(#self_field.unwrap()).unwrap(),
            }
        } else {
            match field.structopt_ty() {
                StructOptTy::Bool | StructOptTy::Other => quote_spanned! {span=>
                    #field_ident: #self_field.unwrap(),
                },
                _ => {
                    quote_spanned! {span=>
                        #field_ident: #self_field,
                    }
                }
            }
        }
    });
    let create = quote! {
        Self {
            #(#field_tokens)*
        }
    };
    quote! {
        if !configopt.is_complete() {
            return Err(configopt);
        }
        Ok(#create)
    }
}

pub fn is_complete_for_enum(variants: &[ParsedVariant]) -> TokenStream {
    variants
        .iter()
        .map(|variant| match variant.field_type() {
            FieldType::Unnamed => {
                let full_configopt_ident = variant.full_configopt_ident();
                quote! {
                    (#full_configopt_ident(inner)) => {
                        inner.is_complete()
                    }
                }
            }
            FieldType::Named | FieldType::Unit => {
                quote! {
                    // TODO
                }
            }
        })
        .collect()
}

pub fn take_for_enum(variants: &[ParsedVariant]) -> TokenStream {
    variants
        .iter()
        .map(|variant| match variant.field_type() {
            FieldType::Unnamed => {
                let full_ident = variant.full_ident();
                let full_configopt_ident = variant.full_configopt_ident();
                quote! {
                    (#full_ident(self_variant), #full_configopt_ident(other_variant)) => {
                        self_variant.take(other_variant);
                    }
                }
            }
            FieldType::Named | FieldType::Unit => {
                quote! {
                    // TODO
                }
            }
        })
        .collect()
}

pub fn from_for_enum(variants: &[ParsedVariant]) -> TokenStream {
    variants
        .iter()
        .map(|variant| match variant.field_type() {
            FieldType::Unnamed => {
                let full_ident = variant.full_ident();
                let full_configopt_ident = variant.full_configopt_ident();
                quote! {
                    #full_ident(inner) => {
                        Ok(#full_configopt_ident(::std::convert::TryInto::try_into(inner).unwrap()))
                    }
                }
            }
            FieldType::Named | FieldType::Unit => {
                quote! {
                    // TODO
                }
            }
        })
        .collect()
}

pub fn try_from_for_enum(variants: &[ParsedVariant]) -> TokenStream {
    // We check upfront if the type `is_complete` so all these `unwrap`'s are ok
    variants
        .iter()
        .map(|variant| match variant.field_type() {
            FieldType::Unnamed => {
                let full_ident = variant.full_ident();
                let full_configopt_ident = variant.full_configopt_ident();
                quote! {
                    #full_configopt_ident(inner) => {
                        #full_ident(::std::convert::TryInto::try_into(inner).unwrap())
                    }
                }
            }
            FieldType::Named | FieldType::Unit => {
                quote! {
                    // TODO
                }
            }
        })
        .collect()
}
