module 0x42::M {
    public fun div_before_mul_trigger(): u64 {
        let a: u64 = 100;
        let b: u64 = 5;
        let c: u64 = 10;

        // Division before multiplication, likely to cause rounding errors
        let result: u64 = (a / b) * c;
        result
    }


    public fun mul_before_div_no_trigger(): u64 {
        let a: u64 = 100;
        let b: u64 = 5;
        let c: u64 = 10;

        // Multiplication before division, less likely to cause rounding errors
        let result: u64 = (a * c) / b;
        result
    }


    public fun separated_operations_no_trigger(): u64 {
        let a: u64 = 100;
        let b: u64 = 5;
        let c: u64 = 10;
        let d: u64 = 2;

        let result: u64 = ((a / b) * c) / d;
        result
    }

}