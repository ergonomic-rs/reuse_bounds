#![allow(incomplete_features)]
#![feature(trace_macros)]
#![feature(const_generics,const_evaluatable_checked)]

mod tests;

///``` should_panic
/// let success = true;
/// assert!(success);
/// panic!();
///```
#[macro_export]
macro_rules! reuse_bounds {
    (
      // @TODO Optional two/three? sets of <generics> first: lifetimes, then const generics + non-lifetime generics, then: where bound_pair_tokens*
      {
          $($bound_pairs_token:tt)*
      }
      
      $($item_to_be_bounded_token:tt)*
    ) => {
        $crate::handle_wrapped_all_items_wrapped_bounds! {
            // Re-ordering: items first, bounds second. That's because we'll despatch/act depending
            // on an item's being `reuse_bounds! { ...}` or not, so that we treat any inner
            // reuse_bounds! {...} recursively first.            
            {
                $($item_to_be_bounded_token)*
            }
            // Wrap bounds together. Then the deeper macro_rules! can pass the same set of bounds
            // to each item.
            {
                $($bound_pairs_token)*
            }
        }
    }
}

#[macro_export]
macro_rules! handle_wrapped_all_items_wrapped_bounds {
    (
        {}
        $_wrapped_bound_pairs:tt
    ) => {};
    (
        {
            $($item_to_be_bounded:item)*
        }
        $wrapped_bound_pairs:tt
    ) => {
        $(
            $crate::pass_wrapped_bounds_to_one_item! {
                $wrapped_bound_pairs
                $item_to_be_bounded
            }
        )*
    }
}

#[macro_export]
macro_rules! pass_wrapped_bounds_to_one_item {
    (
        {
            $($bound_pairs_tokens:tt)*
        }
        
        $item:item
    ) => {
        #[reuse_bounds_derive::pass_unwrapped_bounds_to_one_item($($bound_pairs_tokens)*)]
         $item
    }
}
