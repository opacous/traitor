extern crate proc_macro;
#[macro_use]
extern crate quote;

use {proc_macro2::TokenStream, std::path::Path, syn::TraitBoundModifier};

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
