module 0x42::M {

    // A function with an acceptable number of parameters
    public fun goodFunction(_a: u64, _b: u64, _c: u64) {
        // Function body
    }

    // A function that exceeds the maximum recommended number of parameters
    public fun badFunction(_a: u64, _b: u64, _c: u64, _d: u64, _e: u64, _f: u64) {
        // Function body
    }

}