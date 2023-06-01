#![feature(async_fn_in_trait)]

use std::path::Path;

use proc_macro::TokenStream;
use quote::quote;
use serde::Serialize;
use syn::{
    parse_macro_input,
    punctuated::Punctuated,
    token::{Gt, Lt},
    AngleBracketedGenericArguments, ItemEnum, ItemStruct, ItemTrait, PathSegment, ReturnType,
    Signature, Token, TraitItem, TraitItemMethod, TypePath,
};

#[derive(Debug, Serialize)]
struct MethodMeta {
    name: String,
    output: String,
}

struct ParsedModule {
    structs: Vec<ItemStruct>,
    enums: Vec<ItemEnum>,
    traits: Vec<ItemTrait>,
}

#[proc_macro_attribute]
pub fn service(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;

    let parsed_input: syn::Item = parse_macro_input!(input);

    let module = match parsed_input {
        syn::Item::Mod(module) => module,
        _ => unimplemented!(),
    };

    let extracted_modules = extract_module_items(module);

    let structs = extracted_modules.structs;

    let traits = extracted_modules
        .traits
        .into_iter()
        .map(extract_function_metadata)
        .collect::<Vec<_>>();

    let expanded = quote! {
       #(#structs)*

       #(#traits)*
    };

    expanded.into()
}

fn extract_module_items(module: syn::ItemMod) -> ParsedModule {
    let module = module.content.unwrap().1;

    let mut structs = vec![];
    let mut enums = vec![];
    let mut traits = vec![];

    for item in module {
        match item {
            syn::Item::Enum(e) => enums.push(e),
            syn::Item::Struct(s) => structs.push(s),
            syn::Item::Trait(t) => traits.push(t),
            _ => unimplemented!(),
        }
    }

    ParsedModule {
        enums,
        structs,
        traits,
    }
}

fn extract_function_metadata(t: ItemTrait) -> ItemTrait {
    let items = t.items.clone();
    let mut methods = vec![];

    for item in items {
        match item {
            TraitItem::Method(method) => methods.push(TraitItemMethod {
                attrs: vec![],
                ..method
            }),
            _ => unimplemented!(),
        }
    }

    let items = methods.into_iter().map(TraitItem::Method).collect();
    ItemTrait { items, ..t }
}

// let content = module.content.unwrap().1;

// let trait_block = content
//     .iter()
//     .find_map(|i| match i {
//         syn::Item::Trait(t) => Some(t),
//         _ => None,
//     })
//     .unwrap()
//     .to_owned();

// let trait_ident = trait_block.ident;

// let signatures = trait_block
//     .items
//     .into_iter()
//     .map(|item| match item {
//         TraitItem::Method(method) => method.sig,
//         _ => unimplemented!(),
//     })
//     .collect::<Vec<_>>();

// let metadata_folder = Path::new("service-metadata");

// if !metadata_folder.exists() {
//     std::fs::create_dir_all(metadata_folder).unwrap();
// };

// let method_metadata = signatures
//     .iter()
//     .map(|method_signature| {
//         let args = method_signature.output.to_owned();
//         let output_str = match args {
//             syn::ReturnType::Default => String::from("null"),
//             syn::ReturnType::Type(_, ty) => match *ty {
//                 syn::Type::Path(p_type) => p_type.path.get_ident().unwrap().to_string(),
//                 _ => todo!(),
//             },
//         };

//         MethodMeta {
//             name: method_signature.ident.to_string(),
//             output: output_str,
//         }
//     })
//     .collect::<Vec<_>>();

// for method in method_metadata.iter() {
//     let folder = &metadata_folder.join(&module_ident.to_string().to_ascii_lowercase());

//     if !folder.exists() {
//         std::fs::create_dir_all(folder).unwrap();
//     };

//     let meta_path = folder.join(format!("{}.json", &method.name));
//     std::fs::write(meta_path, serde_json::to_string_pretty(&method).unwrap()).unwrap();
// }
