
macro_rules! op {

    // operators:
    (@stack: $($funs:ident,)* @queue: $($nums:ident,)* @tail: $num:ident $($tail:tt)*) => (
        op!(@stack: $($funs,)* @queue: $($nums,)* $num, @tail: $($tail:tt)*)
    );

    (@stack: $($funs:ident,)* @queue: $($nums:ident,)* @tail: $num:ident $($tail:tt)*) => (
        op!(@stack: $($funs,)* @queue: $($nums,)* $num, @tail: $($tail:tt)*)
    );


    ($($tail:tt)* ) => (
        op!(@stack: @queue: @tail: $($tail)*)
    );
}
