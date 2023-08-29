use std::collections::{HashMap, HashSet};

use proc_macro::TokenStream;
use quote::quote;
use syn::{fold::Fold, punctuated::Punctuated, spanned::Spanned, *};

#[derive(Copy, Clone)]
// All conversions we support.  Check references to this type for an idea how to add more.
enum Conversion<'a> {
    Into(&'a Type),
    AsRef(&'a Type),
    AsMut(&'a Type),
}

impl<'a> Conversion<'a> {
    fn target_type(&self) -> Type {
        match *self {
            Conversion::Into(ty) => ty.clone(),
            Conversion::AsRef(ty) => parse_quote!(&#ty),
            Conversion::AsMut(ty) => parse_quote!(&mut #ty),
        }
    }

    fn conversion_expr(&self, i: Ident) -> Expr {
        match *self {
            Conversion::Into(_) => parse_quote!(#i.into()),
            Conversion::AsRef(_) => parse_quote!(#i.as_ref()),
            Conversion::AsMut(_) => parse_quote!(#i.as_mut()),
        }
    }
}

fn parse_bounded_type(ty: &Type) -> Option<Ident> {
    if let Type::Path(TypePath { qself: None, ref path }) = ty {
        if path.segments.len() == 1 {
            return Some(path.segments[0].ident.clone());
        }
    }
    None
}

fn parse_bounds(bounds: &Punctuated<TypeParamBound, Token![+]>) -> Option<Conversion> {
    if bounds.len() != 1 {
        return None;
    }
    if let TypeParamBound::Trait(ref tb) = bounds.first().unwrap() {
        if let Some(seg) = tb.path.segments.iter().last() {
            if let PathArguments::AngleBracketed(ref gen_args) = seg.arguments {
                if let GenericArgument::Type(ref arg_ty) = gen_args.args.first().unwrap() {
                    if seg.ident == "Into" {
                        return Some(Conversion::Into(arg_ty));
                    } else if seg.ident == "AsRef" {
                        return Some(Conversion::AsRef(arg_ty));
                    } else if seg.ident == "AsMut" {
                        return Some(Conversion::AsMut(arg_ty));
                    }
                }
            }
        }
    }
    None
}

// create a map from generic type to Conversion
fn parse_generics(decl: &Signature) -> (HashMap<Ident, Conversion<'_>>, Generics) {
    let mut ty_conversions = HashMap::new();
    let mut params = Punctuated::new();
    for gp in decl.generics.params.iter() {
        if let GenericParam::Type(ref tp) = gp {
            if let Some(conversion) = parse_bounds(&tp.bounds) {
                ty_conversions.insert(tp.ident.clone(), conversion);
                continue;
            }
        }
        params.push(gp.clone());
    }
    let where_clause = if let Some(ref wc) = decl.generics.where_clause {
        let mut idents_to_remove = HashSet::new();
        let mut predicates = Punctuated::new();
        for wp in wc.predicates.iter() {
            if let WherePredicate::Type(ref pt) = wp {
                if let Some(ident) = parse_bounded_type(&pt.bounded_ty) {
                    if let Some(conversion) = parse_bounds(&pt.bounds) {
                        idents_to_remove.insert(ident.clone());
                        ty_conversions.insert(ident, conversion);
                        continue;
                    }
                }
            }
            predicates.push(wp.clone());
        }
        params = params
            .into_iter()
            .filter(|param| {
                if let GenericParam::Type(type_param) = param {
                    !idents_to_remove.contains(&type_param.ident)
                } else {
                    true
                }
            })
            .collect();
        Some(WhereClause {
            predicates,
            ..wc.clone()
        })
    } else {
        None
    };
    (
        ty_conversions,
        Generics {
            params,
            where_clause,
            ..decl.generics.clone()
        },
    )
}

fn pat_to_ident(pat: &Pat) -> Ident {
    if let Pat::Ident(ref pat_ident) = *pat {
        return pat_ident.ident.clone();
    }
    unimplemented!("No non-ident patterns for now!");
}

fn pat_to_expr(pat: &Pat) -> Expr {
    let ident = pat_to_ident(pat);
    parse_quote!(#ident)
}

fn convert<'a>(
    inputs: &'a Punctuated<FnArg, Token![,]>,
    ty_conversions: &HashMap<Ident, Conversion<'a>>,
) -> (
    Punctuated<FnArg, Token![,]>,
    Conversions,
    Punctuated<Expr, Token![,]>,
    bool,
) {
    let mut argtypes = Punctuated::new();
    let mut conversions = Conversions {
        intos: Vec::new(),
        as_refs: Vec::new(),
        as_muts: Vec::new(),
    };
    let mut argexprs = Punctuated::new();
    let mut has_self = false;
    inputs.iter().for_each(|input| match input {
        FnArg::Receiver(..) => {
            has_self = true;
            argtypes.push(input.clone());
        }
        FnArg::Typed(PatType {
            ref pat,
            ref ty,
            ref colon_token,
            ..
        }) => match **ty {
            Type::ImplTrait(TypeImplTrait { ref bounds, .. }) => {
                if let Some(conv) = parse_bounds(bounds) {
                    argtypes.push(FnArg::Typed(PatType {
                        attrs: Vec::new(),
                        pat: pat.clone(),
                        colon_token: *colon_token,
                        ty: Box::new(conv.target_type()),
                    }));
                    let ident = pat_to_ident(pat);
                    conversions.add(ident.clone(), conv);
                    argexprs.push(conv.conversion_expr(ident));
                } else {
                    argtypes.push(input.clone());
                    argexprs.push(pat_to_expr(pat));
                }
            }
            Type::Path(..) => {
                if let Some(conv) = parse_bounded_type(ty).and_then(|ident| ty_conversions.get(&ident)) {
                    argtypes.push(FnArg::Typed(PatType {
                        attrs: Vec::new(),
                        pat: pat.clone(),
                        colon_token: *colon_token,
                        ty: Box::new(conv.target_type()),
                    }));
                    let ident = pat_to_ident(pat);
                    conversions.add(ident.clone(), *conv);
                    argexprs.push(conv.conversion_expr(ident));
                } else {
                    argtypes.push(input.clone());
                    argexprs.push(pat_to_expr(pat));
                }
            }
            _ => {
                argtypes.push(input.clone());
                argexprs.push(pat_to_expr(pat));
            }
        },
    });
    (argtypes, conversions, argexprs, has_self)
}

struct Conversions {
    intos: Vec<Ident>,
    as_refs: Vec<Ident>,
    as_muts: Vec<Ident>,
}

impl Conversions {
    fn add(&mut self, ident: Ident, conv: Conversion) {
        match conv {
            Conversion::Into(_) => self.intos.push(ident),
            Conversion::AsRef(_) => self.as_refs.push(ident),
            Conversion::AsMut(_) => self.as_muts.push(ident),
        }
    }
}

fn has_conversion(idents: &[Ident], expr: &Expr) -> bool {
    if let Expr::Path(ExprPath { ref path, .. }) = *expr {
        if path.segments.len() == 1 {
            let seg = path.segments.iter().last().unwrap();
            return idents.iter().any(|i| i == &seg.ident);
        }
    }
    false
}

#[allow(clippy::collapsible_if)]
impl Fold for Conversions {
    fn fold_expr(&mut self, expr: Expr) -> Expr {
        //TODO: Also catch `Expr::Call` with suitable paths & args
        match expr {
            Expr::MethodCall(mc) if mc.args.is_empty() => match &*mc.method.to_string() {
                "into" if has_conversion(&self.intos, &mc.receiver) => *mc.receiver,

                "as_ref" if has_conversion(&self.as_refs, &mc.receiver) => *mc.receiver,
                "as_mut" if has_conversion(&self.as_muts, &mc.receiver) => *mc.receiver,

                _ => syn::fold::fold_expr(self, Expr::MethodCall(mc)),
            },
            Expr::Call(call) if call.args.len() == 1 => match &*call.func {
                Expr::Path(ExprPath {
                    path: Path { segments, .. },
                    ..
                }) if segments.len() == 2 => match (&*segments[0].ident.to_string(), &*segments[1].ident.to_string()) {
                    ("Into", "into") if has_conversion(&self.intos, &call.args[0]) => call.args[0].clone(),

                    ("AsRef", "as_ref") if matches!(&call.args[0], Expr::Reference(ExprReference { expr, mutability: None, .. }) if has_conversion(&self.as_refs, expr)) => {
                        if let Expr::Reference(ExprReference { expr, .. }) = &call.args[0] {
                            (**expr).clone()
                        } else {
                            panic!("expr must be Reference")
                        }
                    }
                    ("AsMut", "as_mut") if matches!(&call.args[0], Expr::Reference(ExprReference { expr, mutability: Some(_), .. }) if has_conversion(&self.as_muts, expr)) => {
                        if let Expr::Reference(ExprReference { expr, .. }) = &call.args[0] {
                            (**expr).clone()
                        } else {
                            panic!("expr must be Reference")
                        }
                    }

                    _ => syn::fold::fold_expr(self, Expr::Call(call)),
                },
                _ => syn::fold::fold_expr(self, Expr::Call(call)),
            },
            _ => syn::fold::fold_expr(self, expr),
        }
    }
}

fn contains_self_type_path(path: &Path) -> bool {
    path.segments.iter().any(|segment| {
        segment.ident == "Self"
            || match &segment.arguments {
                PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => {
                    args.iter().any(|generic_arg| match generic_arg {
                        GenericArgument::Type(ty) => contains_self_type(ty),
                        GenericArgument::Const(expr) => contains_self_type_expr(expr),
                        _ => false,
                    })
                }
                PathArguments::Parenthesized(ParenthesizedGenericArguments { inputs, output, .. }) => {
                    inputs.iter().any(contains_self_type)
                        || matches!(output, ReturnType::Type(_, ty) if contains_self_type(ty))
                }
                _ => false,
            }
    })
}

fn contains_self_type_expr(expr: &Expr) -> bool {
    match expr {
        Expr::Path(ExprPath { qself: Some(_), .. }) => true,
        Expr::Path(ExprPath { path, .. }) => contains_self_type_path(path),
        _ => false,
    }
}

fn contains_self_type(input: &Type) -> bool {
    match input {
        Type::Array(TypeArray { elem, len, .. }) => {
            // Call `matches!` first so that we can do tail call here
            // as an optimization.
            contains_self_type_expr(len) || contains_self_type(elem)
        }
        Type::Group(TypeGroup { elem, .. }) => contains_self_type(elem),
        Type::Paren(TypeParen { elem, .. }) => contains_self_type(elem),
        Type::Ptr(TypePtr { elem, .. }) => contains_self_type(elem),
        Type::Reference(TypeReference { elem, .. }) => contains_self_type(elem),
        Type::Slice(TypeSlice { elem, .. }) => contains_self_type(elem),

        Type::Tuple(TypeTuple { elems, .. }) => elems.iter().any(contains_self_type),

        Type::Path(TypePath { qself: Some(_), .. }) => true,
        Type::Path(TypePath { path, .. }) => contains_self_type_path(path),

        _ => false,
    }
}

fn has_self_type(input: &FnArg) -> bool {
    match input {
        FnArg::Receiver(_) => true,
        FnArg::Typed(PatType { ty, .. }) => contains_self_type(ty),
    }
}

/// Generate lightweight monomorphized wrapper around main implementation.
/// May be applied to functions and methods.
#[proc_macro_attribute]
pub fn momo(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    //TODO: alternatively parse ImplItem::Method
    momo_inner(input.into()).into()
}

fn momo_inner(code: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let fn_item: Item = match syn::parse2(code.clone()) {
        Ok(input) => input,
        Err(err) => return err.to_compile_error(),
    };

    if let Item::Fn(item_fn) = fn_item {
        let (ty_conversions, generics) = parse_generics(&item_fn.sig);
        let (argtypes, mut conversions, argexprs, has_self) = convert(&item_fn.sig.inputs, &ty_conversions);

        let uses_self = has_self
            || item_fn.sig.inputs.iter().any(has_self_type)
            || matches!(&item_fn.sig.output, ReturnType::Type(_, ty) if contains_self_type(ty));

        let inner_ident = Ident::new(
            // Use long qualifier to avoid name colision.
            &format!("_{}_inner_generated_by_gix_macro_momo", item_fn.sig.ident),
            proc_macro2::Span::call_site(),
        );

        let new_inner_item = Item::Fn(ItemFn {
            // Remove doc comment since they will increase compile-time and
            // also generates duplicate warning/error messages for the doc,
            // especially if it contains doc-tests.
            attrs: {
                let mut attrs = item_fn.attrs.clone();
                attrs.retain(|attr| {
                    let segments = &attr.path().segments;
                    !(segments.len() == 1 && segments[0].ident == "doc")
                });
                attrs
            },
            vis: Visibility::Inherited,
            sig: Signature {
                ident: inner_ident.clone(),
                generics,
                inputs: argtypes,
                ..item_fn.sig.clone()
            },
            block: Box::new(conversions.fold_block(*item_fn.block)),
        });

        if uses_self {
            // We can use `self` or `Self` inside function defined within
            // the impl-fn, so instead declare two separate functions.
            //
            // Since it's an impl block, it's unlikely to have name conflict,
            // though this won't work for impl-trait.
            //
            // This approach also make sure we can call the right function
            // using `Self` qualifier.
            let new_item = Item::Fn(ItemFn {
                attrs: item_fn.attrs,
                vis: item_fn.vis,
                sig: item_fn.sig,
                block: if has_self {
                    parse_quote!({ self.#inner_ident(#argexprs) })
                } else {
                    parse_quote!({ Self::#inner_ident(#argexprs) })
                },
            });
            quote!(#new_item #[allow(unused_mut)] #new_inner_item)
        } else {
            // Put the new inner function within the function block
            // to avoid duplicate function name and support associated
            // function that doesn't use `self` or `Self`.
            let new_item = Item::Fn(ItemFn {
                attrs: item_fn.attrs,
                vis: item_fn.vis,
                sig: item_fn.sig,
                block: parse_quote!({
                    #[allow(unused_mut)]
                    #new_inner_item

                    #inner_ident(#argexprs)
                }),
            });
            quote!(#new_item)
        }
    } else {
        Error::new(fn_item.span(), "expect a function").to_compile_error()
    }
}
