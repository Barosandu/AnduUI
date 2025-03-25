extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Expr, Data};
use quote::quote;
use std::any::type_name;

#[proc_macro_derive(ToDict)]
pub fn derive_to_todict(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // The name of the struct or enum we're deriving for
    let struct_name = &input.ident;

    let fields = match &input.data {
        Data::Struct(s) => &s.fields,
        _ => panic!("ToHashMap can only be derived for structs"),
    };

    let field_entries = fields.iter().filter_map(|f| {
        let field_name = f.ident.as_ref()?;
        let field_name_str = field_name.to_string();
        Some(quote! {
            map.insert(#field_name_str.to_string(), (self.#field_name.to_string(), String::from("")));
        })
    });



    TokenStream::from(quote! {

        impl ToDict for #struct_name {
            fn to_dict(&self) -> HashMap<String, (String, String)> {
                let mut map = HashMap::new();
                #(#field_entries)*
                map
            }
        }
    })
}
#[proc_macro_derive(Stateful, attributes(state_type))]
pub fn derive_stateful(input: TokenStream) -> TokenStream {
    // Parse the input token stream as DeriveInput (this represents the struct or enum)
    let input = parse_macro_input!(input as DeriveInput);

    // The name of the struct or enum we're deriving for
    let struct_name = &input.ident;

    // Default state_type to 'S'

    // Look through the attributes for 'state_type'


    let mut ss_type: Option<Expr> = None;
    for attr in input.attrs.iter() {
        if attr.path().is_ident("state_type") {
            ss_type = Some(attr.parse_args().unwrap());
        }
    }

    let s_type = ss_type.unwrap();
    // let mut state: Path = syn::parse_str(attribute_ident.as_str()).unwrap();
    // Generate the implementation of the Widget trait for the struct
    let expanded = quote! {
        impl Stateful<#s_type> for #struct_name {
            fn get_state(&mut self) -> #s_type {
                return mem::take(&mut self.state);
            }

            fn set_state(&mut self, state: #s_type) {
                self.state = state;
            }
        }
    };

    // Convert the generated code into a TokenStream and return it
    TokenStream::from(expanded)
}
