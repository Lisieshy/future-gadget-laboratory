extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemFn};

/// Your D-Mail. Be careful of what you send. El Psy Kongroo.
#[proc_macro]
pub fn dmail(item: TokenStream) -> TokenStream {
    item
}


/// You MUST turn on the 42 inch CRT TV in Misuta Braun's shop
/// otherwise the D-Mails will not be able to go to SERN's laboratories.
#[proc_macro_attribute]
pub fn turn_on_42inch_crt(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let ItemFn { attrs, vis, sig, block } = input;

    let mut stored_dmails: Vec<String> = Vec::new();

    for stmt in block.stmts.iter() {
        let tokens: Vec<_> = stmt.to_token_stream().into_iter().collect();


        tokens.windows(3).for_each(|window| {
            // println!("{:?}\n", window);

            match window {
                [x, y, z] if x.to_string() == "dmail".to_string() && y.to_string() == "!".to_string() => {
                    stored_dmails.push(trim(z.to_string().as_str()).to_string())
                }
                _ => {
                }
            }
        });
    }

    let dmail_prints = stored_dmails.iter().map(|dmail| {
        quote! {
            println!("{}", #dmail);
        }
    });

    let result = quote! {
        #(#attrs)*
        #vis #sig {
            #(#dmail_prints)*
            #block
        }
    };
    TokenStream::from(result)
}

fn trim(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.next();
    chars.next_back();
    chars.as_str()
}