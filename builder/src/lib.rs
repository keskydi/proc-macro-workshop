#![allow(warnings)]
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input,
    punctuated::Punctuated,
    token::{Comma, Struct},
    Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, Ident, Meta, Path, Type, spanned::Spanned,
};

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    // Abstrat Syntax Tree => ast
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let bname = &format!("{}Builder", name);
    let bident = &Ident::new(bname, name.span());

    let fields = match &ast.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(ref fields) => &fields.named,
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };

    //for f in fields {
    //    for attr in &f.attrs {
    //        eprintln!("{:#?}", attr.parse_meta());
    //    }
    //}

    //eprintln!("{:#?}", &ast);
    let builder_struct = impl_builder_struct(bident, fields);
    //let builder_struct = impl_builder_struct(&builder_name, fields);
    let build_struct_setters = impl_builder_struct_setters(bident, fields);
    let build_struct_builder = build_struct_builder(name, bident, fields);

    let impl_builder_fn = impl_builder_fn(name, bident, fields);

    let expanded = quote! {

        #builder_struct
        #impl_builder_fn
        #build_struct_setters
        #build_struct_builder

    };
    expanded.into()
}

//     impl Command {
//         pub fn builder() -> CommandBuilder {
//             CommandBuilder {
//                 executable: None,
//                 args: None,
//                 env: None,
//                 current_dir: None,
//             }
//         }
//     }
fn impl_builder_struct(
    name: &Ident,
    fields: &Punctuated<Field, Comma>,
) -> proc_macro2::TokenStream {
    let new_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        if get_inner_ty("Option", ty).is_some() || builder_of(f).is_some() {
            quote! {
             #name: #ty
            }
        } else {
            quote! {
             #name: std::option::Option<#ty>
            }
        }
    });
    let expanded = quote! {
        pub struct #name{
            #(
                #new_fields
            ),*
        }

    };
    expanded
}

//     impl Command {
//         pub fn builder() -> CommandBuilder {
//             CommandBuilder {
//                 executable: None,
//                 args: None,
//                 env: None,
//                 current_dir: None,
//             }
//         }
//     }
fn impl_builder_fn(
    name: &Ident,
    bname: &Ident,
    fields: &Punctuated<Field, Comma>,
) -> proc_macro2::TokenStream {
    let new_fields = fields.iter().map(|f| {
        let name = &f.ident;
        if builder_of(&f).is_some(){
            quote! {
             #name: Vec::new()
            }
        }else{
            quote! {
             #name: std::option::Option::None
            }
        }
    });
    let expanded = quote! {
        impl #name{
            pub fn builder() -> #bname {
                #bname {
                    #( #new_fields ),*
                }
            }
        }

    };
    expanded
}

fn impl_builder_struct_setters(
    bname: &Ident,
    fields: &Punctuated<Field, Comma>,
) -> proc_macro2::TokenStream {
    let new_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;

        
        //handle setter
        let setter = if let Some(t) = get_inner_ty("Option", ty) {
            quote! {
                fn #name(&mut self, #name: #t) -> &mut Self {
                            self.#name = Some(#name);
                            self
                }
            }
        } else if builder_of(f).is_some(){
            quote! {
                fn #name(&mut self, #name: #ty) -> &mut Self {
                            self.#name = #name;
                            self
                }
            }
        } else {
            quote! {
                fn #name(&mut self, #name: #ty) -> &mut Self {
                            self.#name = Some(#name);
                            self
                }
            }
        };
        //handle attr
        //let attr = f.attrs.iter().map(|attr|{
        //    let meta_attr = attr.parse_meta().unwrap();
        //    quote! {
        //        ()
        //    }
        //});
        match extended_methods(f){
            Some((true,extended)) => quote! {
                //#setter
                #extended
            },
            Some((false,extended)) => quote! {
                #setter
                #extended
            },
            _ => quote! {
                #setter
            }
        }
    });

    let expanded = quote! {
        impl #bname{
            #( #new_fields )*
        }
    };
    expanded
}

fn extended_methods(field: &syn::Field) -> Option<(bool,proc_macro2::TokenStream)> {
    let ident = &field.ident;
    let name = field.ident.as_ref().unwrap();

    if let Some(ty) = get_inner_ty("Vec", &field.ty) {

        for attr in &field.attrs {
            let meta = attr.parse_meta().unwrap();
            if let Meta::List(meta_list) = meta {
                if meta_list.path.is_ident("builder") {
                    if let Some(syn::NestedMeta::Meta((syn::Meta::NameValue((meta_name_value))))) =
                        meta_list.nested.first()
                    {
                        if !&meta_name_value.path.is_ident("each"){
                            return Some((false,syn::Error::new_spanned(meta_list, r#"expected `builder(each = "...")`"#).to_compile_error()));
                        }
                        if let syn::Lit::Str(s) = &meta_name_value.lit{

                        

                            let arg = &Ident::new(&s.value(), s.span());

                            return Some((arg == name,quote! {
                                fn #arg(&mut self, #arg: #ty) -> &mut Self {
                                            self.#ident.push(#arg);
                                            self
                                }
                            }));
                        }
                        
                        //return None
                    }
                }
            }
        }
    }

    
    None
}

fn build_struct_builder(
    name: &Ident,
    bname: &Ident,
    fields: &Punctuated<Field, Comma>,
) -> proc_macro2::TokenStream {
    let test_field_iter = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;

        if get_inner_ty("Option", ty).is_some() || builder_of(f).is_some() {
            quote! {
                #name:self.#name.clone()
            }
        } else {
            quote! {
                #name:self.#name.clone().ok_or(concat!(stringify!(#name), " is not set"))?
            }
        }
    });

    let expanded = quote!(
        impl #bname{
            pub fn build(&mut self) -> std::result::Result<#name, std::boxed::Box<dyn std::error::Error>> {
                Ok(#name{
                    #(#test_field_iter),*
                })
            }
        }
    );

    expanded
}

fn get_inner_ty<'a>(tty: &str, ty: &'a Type) -> Option<&'a Type> {
    if let syn::Type::Path(ref p) = ty {
        if p.path.segments.len() != 1 || p.path.segments[0].ident != tty {
            return None;
        }
        if let syn::PathArguments::AngleBracketed(ref inner_ty) = p.path.segments[0].arguments {
            if inner_ty.args.len() != 1 {
                return None;
            }
            let inner = inner_ty.args.first().unwrap();

            if let syn::GenericArgument::Type(ref t) = inner {
                return Some(t);
            }
        }

        return None;
    };
    None
}

fn impl_builder_attr(name: &Ident, fields: &Punctuated<Field, Comma>) -> proc_macro2::TokenStream {
    let new_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        if get_inner_ty("Option", ty).is_some() {
            quote! {
             #name: #ty
            }
        } else {
            quote! {
             #name: std::option::Option<#ty>
            }
        }
    });
    let expanded = quote! {
        pub struct #name{
            #(
                #new_fields
            ),*
        }

    };
    expanded
}

fn builder_of(field: &syn::Field) -> Option<&syn::Attribute> {
    for attr in &field.attrs {
        let meta = attr.parse_meta().unwrap();
        if let Meta::List(meta_list) = meta {
            if meta_list.path.is_ident("builder") {
                return Some(&attr)
            }
        }
    }
    None
}