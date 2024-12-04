// Test calling into another macro inside a macro (first macro
// defined in the same file than where it's called, the second
// macro defined in the same file where it is are called).
// The second macro is called right at the beginning of the first macro
// (stepping into the inner macro and braking in the inner macro).
module macro_same_different_files::m;

use macro_same_different_files::m_dep::baz;

public macro fun bar($param1: u64, $f: |u64| -> u64): u64 {
    let mut ret = baz!($param1);
    ret = ret + $f(ret);
    ret
}

public fun foo(): u64 {
    let v = bar!(
        1,
        |x| x + x
    );
    bar!(v, |x| x + x)
}

#[test]
public fun test() {
    foo();
}
