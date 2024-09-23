use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse, parse2, parse_macro_input, Block, FnArg, Ident, ItemFn, ReturnType, Type};

#[proc_macro_attribute]
pub fn cache(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut new_block = parse::<Block>(cache_block(input.clone())).unwrap();

    let mut input: ItemFn = parse_macro_input!(input as ItemFn);

    let mut args_ident = Vec::new();

    for i in &input.sig.inputs {
        if let FnArg::Typed(ty) = i {
            args_ident.push(ty.pat.to_owned());
        }
    }

    let fn_ident = &input.sig.ident;

    new_block
        .stmts
        .push(parse2(quote! { return #fn_ident(#(#args_ident),*); }).unwrap());

    input.block = Box::new(new_block);

    quote! {
        #input
    }
    .into()
}

#[proc_macro_attribute]
pub fn cache_global(_args: TokenStream, input: TokenStream) -> TokenStream {
    let block = parse::<Block>(cache_block(input)).unwrap();

    let block_body = block.stmts;

    quote! {
        #(#block_body)*
    }
    .into()
}

fn cache_block(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemFn);

    let sig = &input.sig;

    let inputs = &sig.inputs;

    let mut tuple_args = Vec::new();
    let mut args_ident = Vec::new();

    for i in inputs {
        if let FnArg::Typed(ty) = i {
            tuple_args.push(ty.ty.to_owned());
            args_ident.push(ty.pat.to_owned());
        }
    }

    let args_type = quote! {
        (#(#tuple_args),*)
    }
    .into();

    let args_type = parse_macro_input!(args_type as Type);

    let return_type = *(match &input.sig.output {
        ReturnType::Default => {
            let temp = quote! { () }.into();
            Box::new(parse_macro_input!(temp as Type))
        }
        ReturnType::Type(_, t) => t.to_owned(),
    });

    let fn_ident = input.sig.ident.clone().to_string();

    let constructor_ident_string = String::from("_constructor_") + &fn_ident;

    let constructor_ident = Ident::new(&constructor_ident_string, Span::call_site());

    let database_ident_string = String::from("_DATA_BASE_") + &fn_ident;

    let database_ident = Ident::new(&database_ident_string, Span::call_site());

    let inside_block = &input.block;

    let inside_block = quote! {
        {
            if let Some(result) = #database_ident.lock().unwrap().get(&(#(#args_ident),*)) {
                return result.to_owned()
            }

            let result = (move || #inside_block)();

            #database_ident.lock().unwrap().insert((#(#args_ident),*), result.clone());

            result
        }
    }
    .into();

    let inside_block = parse_macro_input!(inside_block as Block);

    input.block = Box::new(inside_block);

    quote! {
        {
            #[allow(non_snake_case)]
            fn #constructor_ident() -> std::sync::Mutex<std::collections::HashMap<#args_type, #return_type>> {
                std::sync::Mutex::new(
                    std::collections::HashMap::new()
                )
            }
            #[allow(non_upper_case_globals)]
            static #database_ident: std::sync::LazyLock<std::sync::Mutex<std::collections::HashMap<#args_type, #return_type>>> =
                std::sync::LazyLock::new(#constructor_ident);

            #input
        }
    }.into()
}
