use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Block, Expr, ExprCall, FnArg, ItemFn, ReturnType, Stmt, Type};

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

    let new_arg = quote! {
        _visited: &mut std::collections::HashMap<#args_type, #return_type>
    }
    .into();

    inside.sig.inputs.push(parse_macro_input!(new_arg as FnArg));

    let fn_ident = &inside.sig.ident;

    for i in &mut inside.block.stmts {
        if let Stmt::Expr(
            Expr::Call(ExprCall {
                attrs: _,
                func,
                paren_token: _,
                args,
            }),
            _,
        ) = i
        {
            if let Expr::Path(expr_path) = func.as_ref() {
                if expr_path.path.get_ident().is_some_and(|ident| ident == fn_ident) {
                    args.push(Expr::Verbatim(quote! { _visited }))
                }
            }
        }
    }

    let new_block = quote! {
        {
            #inside

            let mut data_base = std::collections::HashMap::new();

            let result = #fn_ident(#(#args_ident),* , &mut data_base);
            data_base.insert((#(#args_ident),*), result.clone());

            result
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
