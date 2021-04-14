use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item, Token, parse::Parse, punctuated::Punctuated, WhereClause, WherePredicate,parse::ParseStream, Error, Macro, braced, parse::ParseBuffer};
use proc_macro2::{TokenTree, TokenStream as TokenStream2};
//use proc_macro::TokenTree;
use syn::spanned::Spanned;

struct WherePredicatesWithFlexibleCommas {
    predicates: Vec<WherePredicate>,
}

//@TODO consider making this generic on the item type
impl Parse for WherePredicatesWithFlexibleCommas {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let mut predicates = Vec::<WherePredicate>::new();
        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(Token![,]) {
                let _comma: Token![,] = input.parse()?;
            } else {
                //input.parse().map(WherePredicate)?;
                predicates.push( input.parse()? );
            }
        }
        Ok(WherePredicatesWithFlexibleCommas { predicates })
    }
}

// Just the content/body of the inner reuse_bounds! {...}
struct Group {
    inner_tokens: TokenStream,
    //where_bounds: ParseBuffer<'a>,
    //items: ParseBuffer<'a>
}

impl Parse for Group {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        //let inner_tokens;
        //braced!(inner_tokens in input);
        let token_tree = TokenTree::parse(input)?;
        if let TokenTree::Group(inner_tokens) = token_tree {
            Ok( Self {
                inner_tokens: proc_macro::TokenStream::from(inner_tokens.stream())
                //inner_tokens: TokenTree::parse(input)?
                //where_bounds: where_bounds.fork(),
                //items: input.fork()
            })
        } else {
            panic!();
        }
    }
}

// Testing accepting multiple commas
// https://docs.rs/syn/1.0.68/syn/parse/index.html -> Punctuated may or may not allow trailing punctuation
#[proc_macro_attribute]
pub fn accept_where_predicates_with_flexible_commas(unwrapped_bound_pairs: TokenStream, item: TokenStream) -> TokenStream {
    // The following two have failed with: the trait `CustomToken` is not implemented for `WherePredicate`
    //// 1.
    //let bound_pairs: Punctuated<Option<WherePredicate>,Token![,]> = parse_macro_input!(unwrapped_bound_pairs with Punctuated::parse_terminated);
    //// 2.
    //let parser = Punctuated::<Option<WherePredicate>, Token![,]>::parse_terminated;
    //let _args = parser(unwrapped_bound_pairs);

    let where_predicates_with_commas = parse_macro_input!(unwrapped_bound_pairs as WherePredicatesWithFlexibleCommas);
    let gen = quote! {
        //#item
    };
    gen.into()
}

fn unwrap_braced_tokens(input: TokenStream) -> TokenStream {
    //let parsed = parse_macro_input!(tokens as InnerMacroLikeCall);
    //let parsed:TokenTree = syn::parse(input).unwrap();
    //parsed.where_bounds.into()
    let group = parse_macro_input!(input as Group);
    group.inner_tokens
}

#[proc_macro_attribute]
pub fn pass_unwrapped_bounds_to_one_item(unwrapped_bound_pairs: TokenStream, item: TokenStream) -> TokenStream {
    let bound_pairs: Punctuated<WherePredicate,Token![,]> = parse_macro_input!(unwrapped_bound_pairs with Punctuated::parse_terminated);
    
    let mut item = parse_macro_input!(item as Item);
    if let Item::Macro(item_macro) = item {
        //let Item::Macro(mut item_macro)
        //@TODO compare path - that it's reuse_bounds! only

        // When I translated TokenStream2 to TokenStream (for the following), it consumed the TokenStream2!
        // But I can use quote! {... #variable-name...} only on TokenStream2!
        //let inner_macro_like_call_content_tokens: TokenStream = item_macro.mac.tokens.clone().into();
        // Instead, use TokenStream2.into_iterator() -> Iterator over TokenTree.
        let mut token_tree_it = item_macro.mac.tokens.into_iter();
        let first_group = token_tree_it.next().unwrap();

        // Not efficient. If you know a much better way, please create a merge request.
        // @TODO Consider parsing all the items here, and generate #[pass_unwrapped_bounds_to_one_item(..)] calls here, one per item?
        let rest_of_macro_content: TokenStream2 = token_tree_it.collect();

        if let TokenTree::Group(first_group) = first_group {
            let first_group_unwrapped = first_group.stream();

            let gen = quote! {
                reuse_bounds! {
                    where {
                        #bound_pairs
                        #first_group_unwrapped
    
                    }
                    {
                        #rest_of_macro_content
                    }
                }
            };
    
            return gen.into();
        } else {
            panic!();
        }

        //let mut inner_macro_like_call = parse_macro_input!(inner_macro_like_call_content_tokens as InnerMacroLikeCall);
        //if let Some(inner_macro_like_call) = InnerMacroLikeCall()
        //let unwrapped_first_inner_group = unwrap_braced_tokens(inner_macro_like_call_content_tokens);
        //parse_macro_input!(inner_macro_like_call_content_tokens as Group);
    }

    let generics = match &mut item {
        Item::Enum(item) => {
            &mut item.generics
        }
        Item::Struct(item) => {
            &mut item.generics
        }
        Item::Trait(item) => {
            &mut item.generics
        }
        Item::Union(item) => {
            &mut item.generics
        }
        Item::Impl(item) => {
            &mut item.generics
        }
        Item::Fn(item) => {
            &mut item.sig.generics
        }
        _ => {
            panic!("Expecting the bounded item to be an enum/macro invocation/struct/trait/union, standalone impl (for a enum/struct/trait/union), or impl of a trait (for a enum/struct/trait/union), or a fn. However, received {:?}.", item);
        }
    };
    if let Some(where_clause) = &mut generics.where_clause {
        for bound_pair in bound_pairs {
            where_clause.predicates.push(bound_pair);
        }
    } else {
        generics.where_clause = Some(WhereClause {
            where_token: Token![where](bound_pairs.span()),
            predicates: bound_pairs
        });
    }
    let gen = quote! {
        #item
    };
    gen.into()
}

// @TODO remove
#[proc_macro_attribute]
pub fn pass(params: TokenStream, content: TokenStream) -> TokenStream {
    content
}
