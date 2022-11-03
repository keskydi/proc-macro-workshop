use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Lit, Meta, MetaNameValue};

#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = ast.ident;

    let fields = if let Data::Struct(data) = ast.data {
        if let Fields::Named(fields) = data.fields {
            fields.named
        } else {
            unimplemented!();
        }
    } else {
        unimplemented!();
    };

    let field_list = fields.iter().map(|field| {
        let ident = &field.ident;
        if let Some(attr) = field.attrs.first() {
            // eprintln!("{:#?}", &attr.parse_meta());
            if let Ok(Meta::NameValue(MetaNameValue {
                path,
                lit,
                ..
            })) = attr.parse_meta()
            {
                // if a
                if path.segments.len() == 1 && path.segments.first().unwrap().ident == "debug" {
                    if let Lit::Str(s) = lit {
                        let fmt = s.value();
                        return quote!(
                        .field(stringify!(#ident), &format_args!(#fmt, &self.#ident))
                        );
                    }
                }
            }
        }

        quote!(
            .field(stringify!(#ident), &self.#ident)
        )
    });

    let expanded = quote!(
        impl<> std::fmt::Debug for #ident<> {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                fmt.debug_struct(stringify!(#ident))
                  #(#field_list)*
                  .finish()
            }
        }
    );

    expanded.into()
}
