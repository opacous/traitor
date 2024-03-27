extern crate proc_macro;
#[macro_use]
extern crate quote;

use {
    proc_macro2::TokenStream,
    quote::quote,
    std::path::Path,
    syn::{
        parse::Parse, parse_macro_input, parse_quote, parse_str, punctuated::Punctuated,
        GenericArgument, ImplItemType, TraitBoundModifier, Type, TypePath,
    },
};

/// See crate documentation for more information.
#[proc_macro_attribute]
pub fn auto_gen_impl(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    match auto_gen_impl2(args.into(), input.into()) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

fn auto_gen_impl2(args: TokenStream, input: TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    // Try to parse the token stream from the attribute to get a list of proxy
    // types.
    let constraint_name = syn::parse2::<syn::Path>(args)?;

    let mut trait_def = syn::parse2::<syn::ItemTrait>(input)?;

    let constraints = trait_def.supertraits.clone();
    let trait_name = trait_def.ident.clone();

    let new_bound = syn::TypeParamBound::Trait(syn::TraitBound {
        paren_token: None,
        modifier: TraitBoundModifier::None,
        lifetimes: None,
        path: constraint_name.clone(),
    });

    trait_def.supertraits.clear();
    trait_def.supertraits.push(new_bound);

    let impl_constraint = constraint_name.clone();
    Ok(quote!(pub trait #constraint_name = #constraints;
        #trait_def
        impl <A: #constraint_name> #trait_name for A {}
    ))
}

struct OpsArgs {
    name: syn::Ident,
    generic_constraints: Option<syn::AngleBracketedGenericArguments>,
    generic_args: Option<syn::AngleBracketedGenericArguments>,
}

impl Parse for OpsArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: syn::Ident = input.parse()?;
        let generic_constraints: Option<syn::AngleBracketedGenericArguments> = input.parse().ok();

        let generic_args = match generic_constraints.clone() {
            Some(g) => {
                let mut g = g.clone();
                for arg in g.args.iter_mut() {
                    let n_arg = match arg {
                        GenericArgument::Constraint(ref c) => {
                            let mut segments = Punctuated::new();
                            segments.push(c.ident.clone().into());

                            GenericArgument::Type(Type::Path(TypePath {
                                qself: None,
                                path: syn::Path {
                                    leading_colon: None,
                                    segments,
                                },
                            }))
                        }
                        ref x => (*x).clone(),
                    };

                    *arg = n_arg;
                }

                Some(g)
            }
            _ => None,
        };

        Ok(OpsArgs {
            name,
            generic_constraints,
            generic_args,
        })
    }
}

#[proc_macro]
pub fn traitor_ops(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as OpsArgs);
    let name = input.name;

    let generic_constraints = match input.generic_constraints {
        Some(g) => {
            quote! {#g}
        }
        _ => quote! {""},
    };

    let generic_args = match input.generic_args {
        Some(g) => {
            quote! {#g}
        }
        _ => quote! {""},
    };

    println!("{} {}", name, generic_args);

    let impl_type: Type =
        syn::parse_str(&format!("{} {}", name, generic_args)).expect("paase ishu");

    quote! {
        impl #generic_constraints std::ops::Add for #impl_type
        {
            type Output = <Self as ::traitor::ops::Add>::Output;

            fn add(self, rhs: Self) -> Self::Output {
                <Self as ::traitor::ops::Add>::add(self, rhs)
            }
        }

        impl #generic_constraints std::ops::Sub for #impl_type
        {
            type Output = <Self as ::traitor::ops::Sub>::Output;

            fn sub(self, rhs: Self) -> Self::Output {
                <Self as ::traitor::ops::Sub>::sub(self. rhs)
            }
        }

        impl #generic_constraints std::ops::Neg for #impl_type
        {
            type Output = <Self as ::traitor::ops::Neg>::Output;

            fn neg(self) -> Self::Output {
                <Self as ::traitor::ops::Neg>::neg(self)
            }
        }

        impl #generic_constraints std::ops::Mul for #impl_type
        {
            type Output = <Self as ::traitor::ops::Mul>::Output;

            fn mul(self, rhs: Self) -> Self::Output {
                <Self as ::traitor::ops::Mul>::mul(self, rhs)
            }
        }

        impl #generic_constraints std::ops::Div for #impl_type
        {
            type Output = <#name as ::traitor::ops::Div>::Output;

            fn div(self, rhs: Self) -> Self::Output {
                <#name as ::traitor::ops::Div>::div(self, rhs)
            }
        }
    }
    .into()
}

#[test]
fn do_gen_test() {
    let args: proc_macro2::TokenStream = quote!(MyConstraint);
    let input: proc_macro2::TokenStream = quote!(
        pub trait MyTrait: C1 + C2<Inner: C3> {
            fn some_operation(self) -> f64 {
                0.0
            }
        }
    );

    let output = auto_gen_impl2(args, input).unwrap();
    let desired = quote!(
        pub trait MyConstraint = C1 + C2<Inner: C3>;
        pub trait MyTrait: MyConstraint {
            fn some_operation(self) -> f64 {
                0.0
            }
        }
        impl<A: MyConstraint> MyTrait for A {}
    );

    let out_str = format!("{}", output);
    let expect_str = format!("{}", desired);

    assert!(out_str == expect_str, "{} \n!=\n{}", out_str, expect_str);
    println!("{}", output);
}
