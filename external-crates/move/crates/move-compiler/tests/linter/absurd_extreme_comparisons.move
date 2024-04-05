module 0x42::M {

    fun func1(x: u64) {
        let u64_max: u64 = 18446744073709551615;
        let u64_min = 0;
        let u32_max: u32 = 4294967295;
        let u32_min = 0;
        let u16_max: u16 = 65535;
        let u16_min = 0;
        let u8_max: u8 = 255;
        let u8_min = 0;
        let u128_max: u128 = 340282366920938463463374607431768211455;
        let u128_min = 0;

        if (x > u64_max){

        };

        if (x < u64_min){

        };
    }
}