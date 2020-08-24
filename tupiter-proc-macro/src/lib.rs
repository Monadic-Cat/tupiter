use ::core::str::FromStr;
use proc_macro::{TokenStream, TokenTree};

#[proc_macro]
pub fn decl_for_range(input: TokenStream) -> TokenStream {
    let mut trees = input.into_iter();
    let n: usize = match trees.next().expect("expected integer literal") {
        TokenTree::Literal(x) => x.to_string().parse().expect("expected integer literal"),
        _ => panic!("expected integer literal"),
    };
    let mut code = String::new();
    for x in 1..n {
        let name = format!("Tuple{}Iter", x);
        code.push_str(&format!(
            "decl_tuple_iter_struct!({} | {});",
            &name,
            (0..x).map(|x| x.to_string()).collect::<Vec<_>>().join(", ")
        ));
    }
    TokenStream::from_str(&code).unwrap()
}
