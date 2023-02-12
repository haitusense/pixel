use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item, ItemFn, Stmt};

#[proc_macro_attribute]
pub fn trace(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let mut ast = parse_macro_input!(item as ItemFn);

  let new_stmt = quote! {
      println!("trace");
  };
  let new_stmt: TokenStream = new_stmt.into();
  let new_stmt = parse_macro_input!(new_stmt as Stmt);

  ast.block.stmts.clear();
  ast.block.stmts.push(new_stmt);

  let gen = quote! {
      #ast
  };
  gen.into()
}

#[proc_macro_attribute]
pub fn dummy(_attr: TokenStream, item: TokenStream) -> TokenStream {
  item
}