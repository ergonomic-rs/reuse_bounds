#![allow(incomplete_features)]
#![feature(trace_macros)]
#![feature(const_generics,const_evaluatable_checked)]
#![feature(custom_inner_attributes)]
#![feature(proc_macro_hygiene)]

mod tests;

///``` should_panic
/// let success = true;
/// assert!(success);
/// panic!();
///```
#[macro_export]
macro_rules! reuse_bounds {
    (
        {}
    ) => {}; //@TODO
    (
      // @TODO Optional two/three? sets of <generics> first: lifetimes, then const generics + non-lifetime generics, then: where bound_pair_tokens*
      where {
          $($bound_pairs_token:tt)*
      }
      
      $($item_to_be_bounded_token:item)*
    ) => {
        $crate::handle_wrapped_all_items_wrapped_bounds! {
            // Wrap bounds together. Then the deeper macro_rules! can pass the same set of bounds
            // to each item.
            {
                $($bound_pairs_token)*
            }
            {
                $($item_to_be_bounded_token)*
            }
        }
    }
}

#[macro_export]
macro_rules! handle_wrapped_all_items_wrapped_bounds {
    (
        $wrapped_bound_pairs:tt
        {
            $($item_to_be_bounded:item)*
        }
    ) => {
        $(
            $crate::handle_one_item_wrapped_bounds! {
                $wrapped_bound_pairs
                $item_to_be_bounded
            }
        )*
    }
}

#[macro_export]
macro_rules! handle_one_item_wrapped_bounds {
    (
        {
            $($bound_pairs_tokens:tt)*
        }
        
        $item:item
    ) => {
        #[reuse_bounds_macros::pass_unwrapped_bounds_to_one_item($($bound_pairs_tokens)*)]
         $item
    }
}
