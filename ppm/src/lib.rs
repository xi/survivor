use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::FromIterator;

extern crate proc_macro;
use proc_macro::{Delimiter, Group, Literal, Punct, Spacing, TokenStream, TokenTree};

const WIDTH: usize = 18;
const HEIGHT: usize = 24;

fn comma() -> TokenTree {
    return TokenTree::Punct(Punct::new(',', Spacing::Alone));
}

fn group(items: Vec<TokenTree>) -> TokenTree {
    return TokenTree::Group(Group::new(
        Delimiter::Bracket,
        TokenStream::from_iter(items.into_iter()),
    ));
}

#[proc_macro]
pub fn include_ppm(input: TokenStream) -> TokenStream {
    let s = input.into_iter().next().unwrap().to_string();
    let name = s.strip_prefix('"').unwrap().strip_suffix('"').unwrap();
    let p = format!("img/{}.ppm", name);

    let path = std::path::Path::new(&p);
    let file = File::open(path).unwrap();
    let mut lines = BufReader::new(file).lines().map(|l| l.unwrap());

    assert_eq!(lines.next().unwrap(), "P3");
    assert!(lines.next().unwrap().starts_with('#'));
    assert_eq!(lines.next().unwrap(), format!("{} {}", WIDTH, HEIGHT));
    assert_eq!(lines.next().unwrap(), "255");

    let mut rows = vec![];
    for _ in 0..HEIGHT {
        let mut row = vec![];
        for _ in 0..WIDTH {
            let mut color = vec![];
            for _ in 0..3 {
                let value = lines.next().unwrap().parse::<u8>().unwrap();
                color.push(TokenTree::Literal(Literal::u8_unsuffixed(value)));
                color.push(comma());
            }
            row.push(group(color));
            row.push(comma());
        }
        rows.push(group(row));
        rows.push(comma());
    }
    return TokenStream::from(group(rows));
}
