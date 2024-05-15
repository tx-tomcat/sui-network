module 0x42::M {

    fun func1() {
        let foo = 1;
        let bar = 2;
        // let temp;

        // Direct swap without a temporary variable (should trigger the linter)
        foo = bar;
        bar = foo;

        // Proper swap using a temporary variable (should not trigger the linter)
        temp = foo;
        foo = bar;
        bar = temp;
    }
}