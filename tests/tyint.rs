#![cfg(feature = "macros")]
#![no_std]

extern crate typenum;

use core::marker::PhantomData;
use typenum::{
    consts::*,
    macros::{tyint, tyuint},
};

struct Same<Lhs, Rhs> {
    _phantom: PhantomData<(Lhs, Rhs)>,
}

impl<T> Same<T, T> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

type Positive0 = tyint!(0);
type Positive1 = tyint!(1);
type Positive2 = tyint!(2);
type Positive3 = tyint!(3);
type Positive4 = tyint!(4);
type Positive4294967296 = tyint!(4294967296);

type Negative0 = tyint!(-0);
type Negative1 = tyint!(-1);
type Negative2 = tyint!(-2);
type Negative3 = tyint!(-3);
type Negative4 = tyint!(-4);
type Negative4294967296 = tyint!(-4294967296);

type Unsigned0 = tyuint!(0);
type Unsigned1 = tyuint!(1);
type Unsigned2 = tyuint!(2);
type Unsigned3 = tyuint!(3);
type Unsigned4 = tyuint!(4);
type Unsigned4294967296 = tyuint!(4294967296);

#[test]
fn tyint_test() {
    let _ = Same::<Positive0, Z0>::new();
    let _ = Same::<Positive1, P1>::new();
    let _ = Same::<Positive2, P2>::new();
    let _ = Same::<Positive3, P3>::new();
    let _ = Same::<Positive4, P4>::new();
    let _ = Same::<Positive4294967296, P4294967296>::new();

    let _ = Same::<Negative0, Z0>::new();
    let _ = Same::<Negative1, N1>::new();
    let _ = Same::<Negative2, N2>::new();
    let _ = Same::<Negative3, N3>::new();
    let _ = Same::<Negative4, N4>::new();
    let _ = Same::<Negative4294967296, N4294967296>::new();

    let _ = Same::<Unsigned0, U0>::new();
    let _ = Same::<Unsigned1, U1>::new();
    let _ = Same::<Unsigned2, U2>::new();
    let _ = Same::<Unsigned3, U3>::new();
    let _ = Same::<Unsigned4, U4>::new();
    let _ = Same::<Unsigned4294967296, U4294967296>::new();
}
