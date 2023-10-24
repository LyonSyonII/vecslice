macro_rules! test {
    ( $start:ident = $slit:literal; $end:ident = $elit:literal; $i:ident, $v:ident; $( $(#[ $meta:meta ])* $name:ident => $code:expr )* ) => {
        $(
            #[test]
            $( #[$meta] )*
            fn $name() {
                let $start = $slit;
                let $end = $elit;
                for $i in $start..$end {
                    #[allow(unused)]
                    let mut $v = ($start..$i).collect::<Vec<_>>();
                    if ! $code {
                        panic!("`{}` failed: start = {}, end = {}, i = {}", stringify!($name), $start, $end, $i);
                    }
                }
            }
        )*
    }
}

use vecslice::{Slice, VecSlice};

test! {
    start = 0usize;
    end = 10usize;
    i, v;
    
    #[allow(clippy::comparison_to_empty)]
    new_tail => v.vecslice_at_tail() == [] && v.vecslice_at_tail().is_empty()
    
    #[allow(clippy::comparison_to_empty)]
    new_head => v.vecslice_at_head() == [] && v.vecslice_at_head().is_empty()
    
    new_range_unbound => v.clone().vecslice(..) == v
    
    new_range_unbound_left => v.clone().vecslice(..i) == v
    new_range_unbound_right => v.clone().vecslice(0..) == v
    
    new_range_excluded => v.clone().vecslice(0..i) == &v[0..i]
    
    new_range_included => match (v.clone().try_vecslice(0..=i), v.get(0..=i)) {
        (Some(s), Some(v)) => s == v,
        (None, None) => true,
        _ => false
    }
    
    new_recursive => {
        // Should compile
        let mut v = vec![0i32; 0];
        let mut s: VecSlice<'_, i32> = v.vecslice_at_head();
        let mut s: VecSlice<'_, i32> = s.vecslice_at_head();
        let _: VecSlice<'_, i32> = s.vecslice_at_head();
        true
    }
    
    len => v.vecslice(..).len() == v.len()

    empty => v.vecslice(..).is_empty() == v.is_empty()

    push_back_tail => {
        let mut s = v.vecslice_at_tail();
        for i in i..end {
            s.push_back(i);
        }
        
        s == (i..end).collect::<Vec<_>>() && v == (start..end).collect::<Vec<_>>()
    }

    push_back_head => {
        let mut v = (i..end).collect::<Vec<_>>();
        let mut s = v.vecslice_at_head();
        for i in start..i {
            s.push_back(i);
        }
        
        assert_eq!(s, (start..i).collect::<Vec<_>>());
        assert_eq!(v, (start..end).collect::<Vec<_>>());
        true
    }
}
