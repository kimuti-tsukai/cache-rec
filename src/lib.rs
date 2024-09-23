use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Block, FnArg, ItemFn, ReturnType, Type};

#[proc_macro_attribute]
pub fn cacher(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemFn);

    let mut inside = input.clone();

    // dbg!(&input);

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

    let fn_ident = &inside.sig.ident;

    let inside_block = &inside.block;

    let inside_block = quote! {
        {
            if let Some(result) = DATA_BASE.lock().unwrap().get(&(#(#args_ident),*)) {
                return result.to_owned()
            }

            let result = #inside_block;

            DATA_BASE.lock().unwrap().insert((#(#args_ident),*), result.clone());

            result
        }
    }
    .into();

    let inside_block = parse_macro_input!(inside_block as Block);

    inside.block = Box::new(inside_block);

    let new_block = quote! {
        {
            fn constructor() -> std::sync::Mutex<std::collections::HashMap<#args_type, #return_type>> {
                std::sync::Mutex::new(
                    std::collections::HashMap::new()
                )
            }
            static DATA_BASE: std::sync::LazyLock<std::sync::Mutex<std::collections::HashMap<#args_type, #return_type>>>
             = std::sync::LazyLock::new(constructor);

            #inside

            #fn_ident(#(#args_ident),*)
        }
    }
    .into();

    let new_block = parse_macro_input!(new_block as Block);

    input.block = Box::new(new_block);

    quote! {
        #input
    }
    .into()
}
