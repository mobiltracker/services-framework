use std::path::Path;

use proc_macro::TokenStream;
use quote::quote;
use serde::Serialize;
use syn::{parse_macro_input, ItemImpl};

#[derive(Debug, Serialize)]
struct MethodMeta {
    name: String,
    output: String,
}

#[proc_macro_attribute]
pub fn service(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;

    let parsed_input: syn::Item = parse_macro_input!(input);

    let module = match parsed_input {
        syn::Item::Mod(module) => module,
        _ => unimplemented!(),
    };

    let c = module.clone();

    let module_ident = module.ident;
    let content = module.content.unwrap().1;

    let impl_block = content
        .iter()
        .find(|c| matches!(c, syn::Item::Impl(_)))
        .map(|i| match i {
            syn::Item::Impl(i) => i,
            _ => todo!(),
        })
        .unwrap()
        .to_owned();

    let ItemImpl { items, .. } = impl_block;

    let signatures = items
        .into_iter()
        .map(|item| match item {
            syn::ImplItem::Method(method) => method.sig,
            _ => todo!(),
        })
        .collect::<Vec<_>>();

    let metadata_folder = Path::new("service-metadata");

    if !metadata_folder.exists() {
        std::fs::create_dir_all(metadata_folder).unwrap();
    };

    let method_metadata = signatures.into_iter().map(|method_signature| {
        let args = method_signature.output;
        let output_str = match args {
            syn::ReturnType::Default => String::from("null"),
            syn::ReturnType::Type(_, ty) => match *ty {
                syn::Type::Path(p_type) => p_type.path.get_ident().unwrap().to_string(),
                _ => todo!(),
            },
        };

        MethodMeta {
            name: method_signature.ident.to_string(),
            output: output_str,
        }
    });

    for method in method_metadata {
        let folder = &metadata_folder.join(&module_ident.to_string().to_ascii_lowercase());

        if !folder.exists() {
            std::fs::create_dir_all(folder).unwrap();
        };

        let meta_path = folder.join(format!("{}.json", &method.name));
        std::fs::write(meta_path, serde_json::to_string_pretty(&method).unwrap()).unwrap();
    }

    let expanded = quote!(#c);
    expanded.into()
}
