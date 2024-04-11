module 0x42::M {
    use std::vector;

    fun func1() {
        let arr = vector[1, 2, 3, 4, 5];
        vector::borrow(&arr, 10);
    }
}