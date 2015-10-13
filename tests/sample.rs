extern crate typenum;

use typenum::build;

#[test]
fn sample_runtime_tests() {
    let tests = &[
        &*build::uint_binary_test(7, "Div", 2, 3),
        &*build::uint_unary_test("SizeOf", 17, 5),

        &*build::int_binary_test(7, "Add", 2, 9),
        &*build::int_unary_test("Neg", 3, -3)
            ];
    build::run_tests(tests);
}
