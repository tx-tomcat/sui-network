module 0x42::M {

    #[allow(lint(constant_naming))]
    const Another_BadName: u64 = 42; // Should trigger a warning

    #[allow(lint(empty_if_no_else))]
    public fun trigger_empty_if_no_else(x: u8) {
        // This should trigger the linter warning
        // as it's an empty `if` with no `else`
        if (x > 10) {
        }
    }
}
