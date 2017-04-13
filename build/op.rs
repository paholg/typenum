#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum OpType {
    Operator,
    Function,
}

use self::OpType::*;

struct Op {
    token: &'static str,
    operator: &'static str,
    example: (&'static str, &'static str),
    precedence: u8,
    n_args: u8,
    op_type: OpType,
}

pub fn write_op_macro() -> ::std::io::Result<()> {
    let out_dir = ::std::env::var("OUT_DIR").unwrap();
    let dest = ::std::path::Path::new(&out_dir).join("op.rs");
    let mut f = ::std::fs::File::create(&dest).unwrap();

    let ops = &[Op {
                    token: "*",
                    operator: "Prod",
                    example: ("P3 * P2", "P6"),
                    precedence: 3,
                    n_args: 2,
                    op_type: Operator,
                },
                Op {
                    token: "/",
                    operator: "Quot",
                    example: ("P6 / P2", "P3"),
                    precedence: 3,
                    n_args: 2,
                    op_type: Operator,
                },
                Op {
                    token: "%",
                    operator: "Mod",
                    example: ("P5 % P3", "P2"),
                    precedence: 3,
                    n_args: 2,
                    op_type: Operator,
                },
                Op {
                    token: "+",
                    operator: "Sum",
                    example: ("P2 + P3", "P5"),
                    precedence: 2,
                    n_args: 2,
                    op_type: Operator,
                },
                Op {
                    token: "-",
                    operator: "Diff",
                    example: ("P2 - P3", "N1"),
                    precedence: 2,
                    n_args: 2,
                    op_type: Operator,
                },
                Op {
                    token: "^",
                    operator: "Xor",
                    example: ("U5 ^ U3", "U6"),
                    precedence: 1,
                    n_args: 2,
                    op_type: Operator,
                },
                Op {
                    token: "|",
                    operator: "Or",
                    example: ("U5 | U3", "U7"),
                    precedence: 1,
                    n_args: 2,
                    op_type: Operator,
                },
                Op {
                    token: "&",
                    operator: "And",
                    example: ("U5 & U3", "U1"),
                    precedence: 1,
                    n_args: 2,
                    op_type: Operator,
                },
                Op {
                    token: "sqr",
                    operator: "Square",
                    example: ("sqr(P2)", "P4"),
                    precedence: !0,
                    n_args: 1,
                    op_type: Function,
                },
                Op {
                    token: "cube",
                    operator: "Cube",
                    example: ("cube(P2)", "P8"),
                    precedence: !0,
                    n_args: 1,
                    op_type: Function,
                },
                Op {
                    token: "pow",
                    operator: "Exp",
                    example: ("pow(P2, P3)", "P8"),
                    precedence: !0,
                    n_args: 2,
                    op_type: Function,
                },
                Op {
                    token: "min",
                    operator: "Minimum",
                    example: ("min(P2, P3)", "P2"),
                    precedence: !0,
                    n_args: 2,
                    op_type: Function,
                },
                Op {
                    token: "max",
                    operator: "Maximum",
                    example: ("max(P2, P3)", "P3"),
                    precedence: !0,
                    n_args: 2,
                    op_type: Function,
                }];

    use std::io::Write;
    write!(f,
           "
/**
Convenient type operations.

Any types representing values must be able to be expressed as `ident`s. That means they need to be
in scope.

For example, `P5` is okay, but `typenum::P5` is not.

You may combine operators arbitrarily.


# Example
```rust
#[macro_use] extern crate typenum;
use typenum::{{P1, P2, P3, P4, P5, P10, N3, N7}};

fn main() {{
    type Result = cmp!(P10 == op!(min(P5 * (P3 + P4), (P1 - P2) * (N3 + N7))));
    use typenum::Bit;
    assert!(Result::to_bool());
}}
```

The full list of supported operators is as follows. They all expand to type aliases defined in the
`operator_aliases` module.

")?;

    //write!(f, "Token | Alias | Example\n ===|===|===\n")?;

    for op in ops.iter() {
        write!(f,
               "---\nOperator `{token}`. Expands to `{operator}`.

```rust
# #[macro_use] extern crate typenum;
# use typenum::consts::*;
# use typenum::Bit;
# fn main() {{
type Result = cmp!({ex1} == op!({ex0}));    assert!(Result::to_bool());
# }}
```\n
",
               token = op.token,
               operator = op.operator,
               ex0 = op.example.0,
               ex1 = op.example.1)?;
    }

    write!(f,
           "*/
#[macro_export]
macro_rules! op {{
    ($($tail:tt)*) => ( __op_internal__!($($tail)*) );
}}

    #[doc(hidden)]
    #[macro_export]
    macro_rules! __op_internal__ {{
")?;

    // We first us the shunting-yard algorithm to produce our tokens in Polish notation.
    // See: https://en.wikipedia.org/wiki/Shunting-yard_algorithm

    // Note: ue to macro asymmetry, "the top of the stack" refers to the first element, not the last

    // Stage 1: There are tokens to be read:

    // Token is an operator, o1:
    for o1 in ops.iter().filter(|op| op.op_type == Operator) {
        // if top of stack is operator o2 with o1.precedence <= o2.precedence,
        // then pop o2 off stack onto queue:
        for o2 in ops.iter()
                .filter(|op| op.op_type == Operator)
                .filter(|o2| o1.precedence <= o2.precedence) {
            write!(f,
                   "
(@stack[{o2_op}, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: {o1_token} $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[{o2_op}, $($queue,)*] @tail: {o1_token} $($tail)*)
);",
                   o2_op = o2.operator,
                   o1_token = o1.token)?;
        }
        // Base case: push o1 onto stack
        write!(f,
               "
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: {o1_token} $($tail:tt)*) => (
    __op_internal__!(@stack[{o1_op}, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);",
               o1_op = o1.operator,
               o1_token = o1.token)?;
    }

    // Token is a function, push it onto the stack:
    for fun in ops.iter().filter(|f| f.op_type == Function) {
        write!(f,
               "
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: {f_sym} $($tail:tt)*) => (
    __op_internal__!(@stack[{f_op}, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);",
               f_sym = fun.token,
               f_op = fun.operator)?;
    }

    // Token is a comma: until the top of the stack is a LParen, pop operators from stack to queue
    // Base case: top of stack is LParen
    write!(f,
           "
(@stack[LParen, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: , $($tail:tt)*) => (
    __op_internal__!(@stack[LParen, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);")?;
    // Recursive case: not LParen, pop from stack to queue
    write!(f,
           "
(@stack[$stack_top:ident, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: , $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[$stack_top, $($queue,)*] @tail: , $($tail)*)
);")?;

    // Token is "(": push it onto stack as "LParen". Also convert the ")" to "RParen" to appease
    // the macro gods:
    write!(f,
           "
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: ( $($stuff:tt)* ) $($tail:tt)* )
 => (
    __op_internal__!(@stack[LParen, $($stack,)*] @queue[$($queue,)*]
                     @tail: $($stuff)* RParen $($tail)*)
);")?;

    // Token is "RParen":
    //     1. pop from stack to queue until we see an "LParen",
    //     2. kill the "LParen",
    //     3. if the top of the stack is a function, pop it onto the queue
    // 2. Base case:
    write!(f,
           "
(@stack[LParen, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: RParen $($tail:tt)*) => (
    __op_internal__!(@rp3 @stack[$($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);")?;
    // 1. Recursive case:
    write!(f,
           "
(@stack[$stack_top:ident, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: RParen $($tail:tt)*)
 => (
    __op_internal__!(@stack[$($stack,)*] @queue[$stack_top, $($queue,)*] @tail: RParen $($tail)*)
);")?;
    // 3. Check for function:
    for fun in ops.iter().filter(|f| f.op_type == Function) {
        write!(f,
               "
(@rp3 @stack[{fun_sym}, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: $($tail:tt)*) => (
    __op_internal__!(@rp3 @stack[$($stack,)*] @queue[{fun_op}, $($queue,)*] @tail: $($tail)*)
);",
               fun_sym = fun.token,
               fun_op = fun.operator)?;
    }
    // 3. Base case:
    write!(f,
           "
(@rp3 @stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);")?;

    // Token is a number: Push it onto the queue
    write!(f,
           "
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: $num:ident $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[$num, $($queue,)*] @tail: $($tail)*)
);")?;

    // Out of tokens:
    // Base case: Stack empty: Start evaluating
    write!(f,
           "
(@stack[] @queue[$($queue:ident,)*] @tail: ) => (
    __op_internal__!(@reverse[] @input: $($queue,)*)
);")?;
    // Recursive case: Pop stack to queue
    write!(f,
           "
(@stack[$stack_top:ident, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail:) => (
    __op_internal__!(@stack[$($stack,)*] @queue[$stack_top, $($queue,)*] @tail: )
);")?;

    // Stage 2: Reverse so we have RPN
    write!(f,
           "
(@reverse[$($revved:ident,)*] @input: $head:ident, $($tail:ident,)* ) => (
    __op_internal__!(@reverse[$head, $($revved,)*] @input: $($tail,)*)
);")?;
    write!(f,
           "
(@reverse[$($revved:ident,)*] @input: ) => (
    __op_internal__!(@eval @stack[] @input[$($revved,)*])
);")?;

    // Stage 3: Evaluate in Reverse Polish Notation
    // Operators / Operators with 2 args:
    for op in ops.iter().filter(|op| op.n_args == 2) {
        // Note: We have to switch $a and $b here, otherwise non-commutative functions are backwards
        write!(f,
               "
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[{op}, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::{op}<$b, $a>, $($stack,)*] @input[$($tail,)*])
);",
               op = op.operator)?;
    }
    // Operators with 1 arg:
    for op in ops.iter().filter(|op| op.n_args == 1) {
        write!(f,
               "
(@eval @stack[$a:ty, $($stack:ty,)*] @input[{op}, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::{op}<$a>, $($stack,)*] @input[$($tail,)*])
);",
               op = op.operator)?;
    }

    // Wasn't a function or operator, so must be a value => push onto stack
    write!(f,
           "
(@eval @stack[$($stack:ty,)*] @input[$head:ident, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$head, $($stack,)*] @input[$($tail,)*])
);")?;

    // No input left:
    write!(f,
           "
(@eval @stack[$stack:ty,] @input[]) => (
    $stack
);")?;

    // Stage 0: Get it started
    write!(f,
           "
($($tail:tt)* ) => (
    __op_internal__!(@stack[] @queue[] @tail: $($tail)*)
);")?;


    write!(f,
           "
}}")?;

    Ok(())
}
