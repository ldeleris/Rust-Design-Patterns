extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(DataDownloader)]
pub fn download(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_download(&ast);
    
    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_download(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl DataDownloader for #name {
            fn download(&self, url: String) -> String {
                println!("Downloading from: {}", url);
                String::from("ew0KICAgICJuYW1lIjogIkl2YW4iLA0KICAgICJhZ2UiOiAyNg0KfQ==")
            }
        }
    }
}

#[proc_macro_derive(DataDecoder)]
pub fn decode(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_decode(&ast);
    
    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_decode(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl DataDecoder for #name {
            fn decode(&self, data: String) -> String {
                println!("Decoding from: {}", data);
                String::from("{\n\t\"name\": \"Laurent Deleris\",\n\t\"age\": 50\n}")
            }
        }
    }
}

#[proc_macro_derive(DataDeserializer)]
pub fn parse(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_parse(&ast);
    
    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_parse(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl DataDeserializer for #name {
            fn parse(&self, data: String) -> Option<Person> {
                println!("Parsing from: {}", data);
                Some(
                    Person {
                    name: Some(String::from("Laurent Deleris")),
                    age: Some(50),
                    }
                )            
            }
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
