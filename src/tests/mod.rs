#![cfg(test)]
// @TODO the following made errors (if any) more complicated
// #![reuse_bounds_macros::pass]
use super::reuse_bounds;

/*macro_rules! outer {
    () => {};
    (
        bufo
        $first:item
        $(
            inner! {}
        )*
        //$($second:item)?
    ) => {};
    (
        $($other_item:item)?
        $(
            $(
                inner! {}
            )*
            $(other_item:item)?
        )+
    ) => {}
}

#[test]
fn outer_macro() {
    outer! {
        struct OuterStructEmpty1;
        inner! {}
        //struct OuterStructEmpty2;
    }
}*/

#[test]
fn cant_macro_replaced_repetition_separator() {
    /*macro_rules! mac {
        (
            $($lit:literal),*
        ) => {
            $($lit, "---")*
        }
    }
    //println!("{}, {}, {}, {}", mac!(1, 8));
    let arr = [mac!("1", "8")];
    */
}

#[test]
fn non_tokens_passed_down() {
    macro_rules! inner {
        (
            $left:ty ,
            $right:ty
        ) => {}
    }

    macro_rules! outer {
        (
            $type:ty
        ) => {
            inner! {
                $type, u32
            }
        };
    }
    outer! {usize};
}

// Generic params: First lifetimes, then consts and types
struct GenericVariaties<'a, T, const N: usize, U, const P: usize>
where [(); N+1]: Sized {
    arr_plus_one: [char; N+1],
    arr_p: [u8; P],
    string: &'a str,
    t: T,
    u: U
}

macro_rules! generics {
    (
        $( ^ << {
             $($left_lifetime:tt)*
        } )?
        $( << ^ {
            $($right_lifetime:tt)*
        } )?
        $( ^ >>  {
            $($left_type:tt)*
        } )?
        $( >> ^ {
            $($right_type:tt)*
        } )?
        $( where {
            $($_tt3:tt)*
        } )?
        {   
            // Either :item or :tt is OK within {...}:
            $($_item:item)*
            //$($item_to_be_bounded_token:tt)*
        }
    ) => {};
}

generics! {
    ^ << {
        'a, 'b
    }
    << ^ {
        'c, 'd
    }
    ^ >> {
        T, const N: usize
    }
    >> ^ {
        U, const P: usize
    }
    where {

    }
    {}
}

#[test]
fn accepts_no_bounds_and_items () {
    super::reuse_bounds! {
        {}
    }
}

#[test]
fn accepts_empty_bounds_and_items () {
    super::reuse_bounds! {
        where {}
    }
}

#[test]
fn inner_reuse () {
    //trace_macros!(true);
    reuse_bounds! {
        where {
            [(); 2*N] : Sized,
        }
        struct S<const N: usize> {
            value: [char; 2*N]
        }

        reuse_bounds! {
            //@TODO << ^ ... where
            // additional bounds on top of the outer bounds:
            {
                // @TODO TODO currently we require a trailing comma here:
                [(); 3*N] : Sized,
            }
            struct InnerS<const N: usize>
            {
                twice_fields: [u32;2*N],
                thrice_fields: [u32;3*N],
                b: bool
            }
        }

        enum E<const N: usize> {
            ExactlyOne(S<N>)
        }

    }
    let _s = S::<2> {value: ['a', 'b', 'c', 'd']};
    let _inner_s = InnerS::<1> {
        twice_fields: [1, 2],
        thrice_fields: [4, 5, 6],
        b: true
    };
    let _e = E::<2>::ExactlyOne(_s);
    trace_macros!(false);
}

#[test]
fn apply_on_struct_impl_fn() {
    //trace_macros!(true);
    trait Traity {}
    reuse_bounds! {
        where {
            [(); 4*N] : Sized,
            [(); 5*N] : Sized
        }
        
        struct S<const N: usize>
        {
            four_fields: [u32;4*N],
            five_fields: [u32;5*N],
            x: bool
        }

        impl <const N: usize> S<N> {
            pub fn new() -> S<N> {
                S {
                    four_fields: [0; 4*N],
                    five_fields: [0; 5*N],
                    x: false
                }
            }
        }

        impl <const N: usize> Traity for S<N> {}

        fn f<const N: usize>(_arg: S<N>) -> S<N> {
            let result = S::<N>::new();
            result
        }

        enum E<const N: usize> {
            ExactlyOne(S<N>)
        }

        union U<const N: usize>
        {
            four_fields: [u32;4*N],
            five_fields: [u32;5*N],
            x: bool
        }

        // @TODO negative test - that this, and other "items", fail:
        // See https://docs.rs/syn/*/syn/enum.Item.html.
        //const C:u32 = 1;
    }
    trace_macros!(false);
}
