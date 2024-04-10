module 0x42::M {

    fun func1() {
        let a = 5;
        let b = 10;

        // Comparison operations with equal operands (should trigger lint)
        let _ = a == a;
        let _ = b != b;
        let _ = a > a;
        let _ = a >= a;

        // Logical operations with equal operands (should not trigger lint)
        let _ = a && a;
        let _ = b || b;

        // Bitwise operations with equal operands (should trigger lint)
        let _ = a & a;
        let _ = b | b;
        let _ = a ^ a;

        // Difference and division operations with equal operands (should trigger lint)
        let _ = a - a;
        let _ = b / b;

        // Operations with unequal operands (should not trigger lint)
        let _ = a == b;
        let _ = a != b;
        let _ = a > b;
        let _ = a < b;
        let _ = a && (b > a);
        let _ = a || (a < b);
        let _ = a & b;
        let _ = a | b;
        let _ = a ^ b;
        let _ = a - b;
        let _ = a / b;
 
    }
}