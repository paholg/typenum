struct Op {
    token: char,
    operator: &'static str,
    precedence: u8,
}

struct Function {
    symbol: &'static str,
    operator: &'static str,
    n_args: u8,
}


pub fn write_op_macro() -> ::std::io::Result<()> {
    let out_dir = ::std::env::var("OUT_DIR").unwrap();
    let dest = ::std::path::Path::new(&out_dir).join("op.rs");
    let mut f = ::std::fs::File::create(&dest).unwrap();

    use std::io::Write;
    write!(f, "
/**
Convenient type operations.

The following operations are supported: `*`, `/`, `+`, `-`, `sq`, `pow`, `min`, `max`, as well as parentheses.

# Example
```rust
#[macro_use] extern crate typenum;
use typenum::{{P1, P2, P3, P4, P5, N3, N7}};

fn main() {{
    type Test = op!(min(P5 * (P3 + P4), (P1 - P2) * (N3 + N7)));
    use typenum::Integer;
    assert_eq!(Test::to_i64(), 10);
}}
```
*/
#[macro_export]
macro_rules! op {{
    ($($tail:tt)*) => ( __op_internal__!($($tail)*) );
}}
")?;
    write!(f, "
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __op_internal__ {{
")?;

    let mul = Op { token: '*', operator: "Prod", precedence: 3 };
    let div = Op { token: '/', operator: "Div", precedence: 3 };
    let add = Op { token: '+', operator: "Sum", precedence: 2 };
    let sub = Op { token: '-', operator: "Diff", precedence: 2 };
    let ops = &[mul, div, add, sub];

    let square = Function { symbol: "sq", operator: "Square", n_args: 1 };
    let pow = Function { symbol: "pow", operator: "Exp", n_args: 2 };
    let min = Function { symbol: "min", operator: "Minimum", n_args: 2 };
    let max = Function { symbol: "max", operator: "Maximum", n_args: 2 };
    let funs = &[square, pow, min, max];

    // We first us the shunting-yard algorithm to produce our tokens in Polish notation.
    // See: https://en.wikipedia.org/wiki/Shunting-yard_algorithm

    // Note: ue to macro asymmetry, "the top of the stack" refers to the first element, not the last

    // Stage 1: There are tokens to be read:

    // Token is an operator, o1:
    for o1 in ops {
        // if top of stack is operator o2 with o1.precedence <= o2.precedence,
        // then pop o2 off stack onto queue:
        for o2 in ops.iter().filter(|o2| o1.precedence <= o2.precedence) {
            write!(f, "
    (@stack[{o2_op}, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: {o1_token} $($tail:tt)*) => (
        __op_internal__!(@stack[$($stack,)*] @queue[{o2_op}, $($queue,)*] @tail: {o1_token} $($tail)*)
    );", o2_op = o2.operator, o1_token = o1.token)?;
        }
        // Base case: push o1 onto stack
        write!(f, "
    (@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: {o1_token} $($tail:tt)*) => (
        __op_internal__!(@stack[{o1_op}, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
    );", o1_op = o1.operator, o1_token = o1.token)?;
    }

    // Token is a function, push it onto the stack:
    for fun in funs {
        write!(f, "
    (@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: {f_sym} $($tail:tt)*) => (
        __op_internal__!(@stack[{f_op}, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
    );", f_sym = fun.symbol, f_op = fun.operator)?;
    }

    // Token is a comma: until the top of the stack is a LParen, pop operators from stack to queue
    // Base case: top of stack is LParen
    write!(f, "
    (@stack[LParen, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: , $($tail:tt)*) => (
        __op_internal__!(@stack[LParen, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
    );")?;
    // Recursive case: not LParen, pop from stack to queue
    write!(f, "
    (@stack[$stack_top:ident, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: , $($tail:tt)*) => (
        __op_internal__!(@stack[$($stack,)*] @queue[$stack_top, $($queue,)*] @tail: , $($tail)*)
    );")?;

    // Token is "(": push it onto stack as "LParen". Also convert the ")" to "RParen" to appease the macro gods:
    write!(f, "
    (@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: ( $($stuff:tt)* ) $($tail:tt)* ) => (
        __op_internal__!(@stack[LParen, $($stack,)*] @queue[$($queue,)*] @tail: $($stuff)* RParen $($tail)*)
    );")?;

    // Token is "RParen":
    //     1. pop from stack to queue until we see an "LParen",
    //     2. kill the "LParen",
    //     3. if the top of the stack is a function, pop it onto the queue
    // 2. Base case:
    write!(f, "
    (@stack[LParen, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: RParen $($tail:tt)*) => (
        __op_internal__!(@rp3 @stack[$($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
    );")?;
    // 1. Recursive case:
    write!(f, "
    (@stack[$stack_top:ident, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: RParen $($tail:tt)*) => (
        __op_internal__!(@stack[$($stack,)*] @queue[$stack_top, $($queue,)*] @tail: RParen $($tail)*)
    );")?;
    // 3. Check for function:
    for fun in funs {
        write!(f, "
    (@rp3 @stack[{fun_sym}, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: $($tail:tt)*) => (
        __op_internal__!(@rp3 @stack[$($stack,)*] @queue[{fun_op}, $($queue,)*] @tail: $($tail)*)
    );", fun_sym = fun.symbol, fun_op = fun.operator)?;
    }
    // 3. Base case:
    write!(f, "
    (@rp3 @stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: $($tail:tt)*) => (
        __op_internal__!(@stack[$($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
    );")?;

    // Token is a number: Push it onto the queue
    write!(f, "
    (@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: $num:ident $($tail:tt)*) => (
        __op_internal__!(@stack[$($stack,)*] @queue[$num, $($queue,)*] @tail: $($tail)*)
    );")?;

    // Out of tokens:
    // Base case: Stack empty: Start evaluating
    write!(f, "
    (@stack[] @queue[$($queue:ident,)*] @tail: ) => (
        __op_internal__!(@reverse[] @input: $($queue,)*)
        // __op_internal__!(@eval @stack[] @input[$($queue,)*])
    );")?;
    // Recursive case: Pop stack to queue
    write!(f, "
    (@stack[$stack_top:ident, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail:) => (
        __op_internal__!(@stack[$($stack,)*] @queue[$stack_top, $($queue,)*] @tail: )
    );")?;

    // Stage 2: Reverse so we have RPN
    write!(f, "
    (@reverse[$($revved:ident,)*] @input: $head:ident, $($tail:ident,)* ) => (
        __op_internal__!(@reverse[$head, $($revved,)*] @input: $($tail,)*)
    );")?;
    write!(f, "
    (@reverse[$($revved:ident,)*] @input: ) => (
        __op_internal__!(@eval @stack[] @input[$($revved,)*])
    );")?;

    // Stage 3: Evaluate in Reverse Polish Notation
    // Functions / Operators with 2 args:
    for op in ops.iter().map(|op| op.operator).chain(funs.iter().filter(|f| f.n_args == 2).map(|f| f.operator)) {
        // Note: We have to switch $a and $b here, otherwise non-commutative functions are backwards
        write!(f, "
    (@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[{op}, $($tail:ident,)*]) => (
        __op_internal__!(@eval @stack[$crate::{op}<$b, $a>, $($stack,)*] @input[$($tail,)*])
    );", op = op)?;
    }
    // Functions with 1 arg:
    for fun in funs.iter().filter(|f| f.n_args == 1) {
        write!(f, "
    (@eval @stack[$a:ty, $($stack:ty,)*] @input[{op}, $($tail:ident,)*]) => (
        __op_internal__!(@eval @stack[$crate::{op}<$a>, $($stack,)*] @input[$($tail,)*])
    );", op = fun.operator)?;
    }

    // Wasn't a function or operator, so must be a value => push onto stack
    write!(f, "
    (@eval @stack[$($stack:ty,)*] @input[$head:ident, $($tail:ident,)*]) => (
        __op_internal__!(@eval @stack[$head, $($stack,)*] @input[$($tail,)*])
    );")?;

    // No input left:
    write!(f, "
    (@eval @stack[$stack:ty,] @input[]) => (
        $stack
    );")?;

    // Stage 0: Get it started
    write!(f, "
    ($($tail:tt)* ) => (
        __op_internal__!(@stack[] @queue[] @tail: $($tail)*)
    );")?;


    write!(f, "}}")?;

    Ok(())
}
