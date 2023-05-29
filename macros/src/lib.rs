use std::path::Path;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use serde::Serialize;
use syn::{
    parse_macro_input,
    punctuated::Punctuated,
    token::{Async, Gt, Lt},
    AngleBracketedGenericArguments, PathSegment, ReturnType, Signature, Token, TraitItem, TypePath,
};

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

    let module_ident = module.ident;
    let content = module.content.unwrap().1;

    let trait_block = content
        .iter()
        .find_map(|i| match i {
            syn::Item::Trait(t) => Some(t),
            _ => None,
        })
        .unwrap()
        .to_owned();

    let trait_ident = trait_block.ident;

    let signatures = trait_block
        .items
        .into_iter()
        .map(|item| match item {
            TraitItem::Method(method) => method.sig,
            _ => unimplemented!(),
        })
        .collect::<Vec<_>>();

    let metadata_folder = Path::new("service-metadata");

    if !metadata_folder.exists() {
        std::fs::create_dir_all(metadata_folder).unwrap();
    };

    let method_metadata = signatures
        .iter()
        .map(|method_signature| {
            let args = method_signature.output.to_owned();
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
        })
        .collect::<Vec<_>>();

    for method in method_metadata.iter() {
        let folder = &metadata_folder.join(&module_ident.to_string().to_ascii_lowercase());

        if !folder.exists() {
            std::fs::create_dir_all(folder).unwrap();
        };

        let meta_path = folder.join(format!("{}.json", &method.name));
        std::fs::write(meta_path, serde_json::to_string_pretty(&method).unwrap()).unwrap();
    }

    let async_signatures = signatures
        .into_iter()
        .map(|t| {
            let new_output = match t.output.clone() {
                ReturnType::Default => ReturnType::Default,
                ReturnType::Type(r_arrow, ty) => match *ty {
                    syn::Type::Path(path) => {
                        // let path_str = format!(
                        //     "Pin<Box<dyn Future<Output = {}>>>",
                        //     path.path.get_ident().map(|i| i.to_string()).unwrap()
                        // );

                        let mut path_segments: Punctuated<PathSegment, Token![::]> =
                            Punctuated::new();

                        let mut box_generics = Punctuated::new();
                        box_generics.push(syn::GenericArgument::Type(syn::Type::Path(TypePath {
                            qself: None,
                            path: path.path,
                        })));

                        path_segments.push(PathSegment {
                            ident: proc_macro2::Ident::new("Box", proc_macro2::Span::call_site()),
                            arguments: syn::PathArguments::AngleBracketed(
                                AngleBracketedGenericArguments {
                                    colon2_token: None,
                                    lt_token: Lt::default(),
                                    args: box_generics,
                                    gt_token: Gt::default(),
                                },
                            ),
                        });

                        let new_path = syn::Path {
                            leading_colon: None,
                            segments: path_segments,
                        };

                        ReturnType::Type(
                            r_arrow,
                            Box::new(syn::Type::Path(TypePath {
                                qself: None,
                                path: new_path,
                            })),
                        )
                    }
                    _ => todo!(),
                },
            };

            Signature {
                asyncness: None,
                ..t
            }
        })
        .collect::<Vec<_>>();

    let foo = async_signatures[0].clone();

    let expanded = quote! {
        pub trait #trait_ident {
            //fn print_message(&self, message: Message) -> Message;
            #foo
        }
    };

    expanded.into()
}
