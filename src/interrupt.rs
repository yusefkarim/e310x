#[doc = r" Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    #[doc = "1 - WATCHDOG"]
    WATCHDOG,
    #[doc = "2 - RTC"]
    RTC,
    #[doc = "3 - UART0"]
    UART0,
    #[doc = "4 - UART1"]
    UART1,
    #[doc = "5 - QSPI0"]
    QSPI0,
    #[doc = "6 - QSPI1"]
    QSPI1,
    #[doc = "7 - QSPI2"]
    QSPI2,
    #[doc = "8 - GPIO0"]
    GPIO0,
    #[doc = "9 - GPIO1"]
    GPIO1,
    #[doc = "10 - GPIO2"]
    GPIO2,
    #[doc = "11 - GPIO3"]
    GPIO3,
    #[doc = "12 - GPIO4"]
    GPIO4,
    #[doc = "13 - GPIO5"]
    GPIO5,
    #[doc = "14 - GPIO6"]
    GPIO6,
    #[doc = "15 - GPIO7"]
    GPIO7,
    #[doc = "16 - GPIO8"]
    GPIO8,
    #[doc = "17 - GPIO9"]
    GPIO9,
    #[doc = "18 - GPIO10"]
    GPIO10,
    #[doc = "19 - GPIO11"]
    GPIO11,
    #[doc = "20 - GPIO12"]
    GPIO12,
    #[doc = "21 - GPIO13"]
    GPIO13,
    #[doc = "22 - GPIO14"]
    GPIO14,
    #[doc = "23 - GPIO15"]
    GPIO15,
    #[doc = "24 - GPIO16"]
    GPIO16,
    #[doc = "25 - GPIO17"]
    GPIO17,
    #[doc = "26 - GPIO18"]
    GPIO18,
    #[doc = "27 - GPIO19"]
    GPIO19,
    #[doc = "28 - GPIO20"]
    GPIO20,
    #[doc = "29 - GPIO21"]
    GPIO21,
    #[doc = "30 - GPIO22"]
    GPIO22,
    #[doc = "31 - GPIO23"]
    GPIO23,
    #[doc = "32 - GPIO24"]
    GPIO24,
    #[doc = "33 - GPIO25"]
    GPIO25,
    #[doc = "34 - GPIO26"]
    GPIO26,
    #[doc = "35 - GPIO27"]
    GPIO27,
    #[doc = "36 - GPIO28"]
    GPIO28,
    #[doc = "37 - GPIO29"]
    GPIO29,
    #[doc = "38 - GPIO30"]
    GPIO30,
    #[doc = "39 - GPIO31"]
    GPIO31,
    #[doc = "40 - PWM0CMP0"]
    PWM0CMP0,
    #[doc = "41 - PWM0CMP1"]
    PWM0CMP1,
    #[doc = "42 - PWM0CMP2"]
    PWM0CMP2,
    #[doc = "43 - PWM0CMP3"]
    PWM0CMP3,
    #[doc = "44 - PWM1CMP0"]
    PWM1CMP0,
    #[doc = "45 - PWM1CMP1"]
    PWM1CMP1,
    #[doc = "46 - PWM1CMP2"]
    PWM1CMP2,
    #[doc = "47 - PWM1CMP3"]
    PWM1CMP3,
    #[doc = "48 - PWM2CMP0"]
    PWM2CMP0,
    #[doc = "49 - PWM2CMP1"]
    PWM2CMP1,
    #[doc = "50 - PWM2CMP2"]
    PWM2CMP2,
    #[doc = "51 - PWM2CMP3"]
    PWM2CMP3,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::WATCHDOG => 1,
            Interrupt::RTC => 2,
            Interrupt::UART0 => 3,
            Interrupt::UART1 => 4,
            Interrupt::QSPI0 => 5,
            Interrupt::QSPI1 => 6,
            Interrupt::QSPI2 => 7,
            Interrupt::GPIO0 => 8,
            Interrupt::GPIO1 => 9,
            Interrupt::GPIO2 => 10,
            Interrupt::GPIO3 => 11,
            Interrupt::GPIO4 => 12,
            Interrupt::GPIO5 => 13,
            Interrupt::GPIO6 => 14,
            Interrupt::GPIO7 => 15,
            Interrupt::GPIO8 => 16,
            Interrupt::GPIO9 => 17,
            Interrupt::GPIO10 => 18,
            Interrupt::GPIO11 => 19,
            Interrupt::GPIO12 => 20,
            Interrupt::GPIO13 => 21,
            Interrupt::GPIO14 => 22,
            Interrupt::GPIO15 => 23,
            Interrupt::GPIO16 => 24,
            Interrupt::GPIO17 => 25,
            Interrupt::GPIO18 => 26,
            Interrupt::GPIO19 => 27,
            Interrupt::GPIO20 => 28,
            Interrupt::GPIO21 => 29,
            Interrupt::GPIO22 => 30,
            Interrupt::GPIO23 => 31,
            Interrupt::GPIO24 => 32,
            Interrupt::GPIO25 => 33,
            Interrupt::GPIO26 => 34,
            Interrupt::GPIO27 => 35,
            Interrupt::GPIO28 => 36,
            Interrupt::GPIO29 => 37,
            Interrupt::GPIO30 => 38,
            Interrupt::GPIO31 => 39,
            Interrupt::PWM0CMP0 => 40,
            Interrupt::PWM0CMP1 => 41,
            Interrupt::PWM0CMP2 => 42,
            Interrupt::PWM0CMP3 => 43,
            Interrupt::PWM1CMP0 => 44,
            Interrupt::PWM1CMP1 => 45,
            Interrupt::PWM1CMP2 => 46,
            Interrupt::PWM1CMP3 => 47,
            Interrupt::PWM2CMP0 => 48,
            Interrupt::PWM2CMP1 => 49,
            Interrupt::PWM2CMP2 => 50,
            Interrupt::PWM2CMP3 => 51,
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            1 => Ok(Interrupt::WATCHDOG),
            2 => Ok(Interrupt::RTC),
            3 => Ok(Interrupt::UART0),
            4 => Ok(Interrupt::UART1),
            5 => Ok(Interrupt::QSPI0),
            6 => Ok(Interrupt::QSPI1),
            7 => Ok(Interrupt::QSPI2),
            8 => Ok(Interrupt::GPIO0),
            9 => Ok(Interrupt::GPIO1),
            10 => Ok(Interrupt::GPIO2),
            11 => Ok(Interrupt::GPIO3),
            12 => Ok(Interrupt::GPIO4),
            13 => Ok(Interrupt::GPIO5),
            14 => Ok(Interrupt::GPIO6),
            15 => Ok(Interrupt::GPIO7),
            16 => Ok(Interrupt::GPIO8),
            17 => Ok(Interrupt::GPIO9),
            18 => Ok(Interrupt::GPIO10),
            19 => Ok(Interrupt::GPIO11),
            20 => Ok(Interrupt::GPIO12),
            21 => Ok(Interrupt::GPIO13),
            22 => Ok(Interrupt::GPIO14),
            23 => Ok(Interrupt::GPIO15),
            24 => Ok(Interrupt::GPIO16),
            25 => Ok(Interrupt::GPIO17),
            26 => Ok(Interrupt::GPIO18),
            27 => Ok(Interrupt::GPIO19),
            28 => Ok(Interrupt::GPIO20),
            29 => Ok(Interrupt::GPIO21),
            30 => Ok(Interrupt::GPIO22),
            31 => Ok(Interrupt::GPIO23),
            32 => Ok(Interrupt::GPIO24),
            33 => Ok(Interrupt::GPIO25),
            34 => Ok(Interrupt::GPIO26),
            35 => Ok(Interrupt::GPIO27),
            36 => Ok(Interrupt::GPIO28),
            37 => Ok(Interrupt::GPIO29),
            38 => Ok(Interrupt::GPIO30),
            39 => Ok(Interrupt::GPIO31),
            40 => Ok(Interrupt::PWM0CMP0),
            41 => Ok(Interrupt::PWM0CMP1),
            42 => Ok(Interrupt::PWM0CMP2),
            43 => Ok(Interrupt::PWM0CMP3),
            44 => Ok(Interrupt::PWM1CMP0),
            45 => Ok(Interrupt::PWM1CMP1),
            46 => Ok(Interrupt::PWM1CMP2),
            47 => Ok(Interrupt::PWM1CMP3),
            48 => Ok(Interrupt::PWM2CMP0),
            49 => Ok(Interrupt::PWM2CMP1),
            50 => Ok(Interrupt::PWM2CMP2),
            51 => Ok(Interrupt::PWM2CMP3),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
#[doc = r" Assigns a handler to an interrupt"]
#[doc = r""]
#[doc = r" This macro takes two arguments: the name of an interrupt and the path to the"]
#[doc = r" function that will be used as the handler of that interrupt. That function"]
#[doc = r" must have signature `fn()`."]
#[doc = r""]
#[doc = r" Optionally, a third argument may be used to declare interrupt local data."]
#[doc = r" The handler will have exclusive access to these *local* variables on each"]
#[doc = r" invocation. If the third argument is used then the signature of the handler"]
#[doc = r" function must be `fn(&mut $NAME::Locals)` where `$NAME` is the first argument"]
#[doc = r" passed to the macro."]
#[doc = r""]
#[doc = r" # Example"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!(TIM2, periodic);"]
#[doc = r""]
#[doc = r" fn periodic() {"]
#[doc = r#"     print!(".");"#]
#[doc = r" }"]
#[doc = r""]
#[doc = r" interrupt!(TIM3, tick, locals: {"]
#[doc = r"     tick: bool = false;"]
#[doc = r" });"]
#[doc = r""]
#[doc = r" fn tick(locals: &mut TIM3::Locals) {"]
#[doc = r"     locals.tick = !locals.tick;"]
#[doc = r""]
#[doc = r"     if locals.tick {"]
#[doc = r#"         println!("Tick");"#]
#[doc = r"     } else {"]
#[doc = r#"         println!("Tock");"#]
#[doc = r"     }"]
#[doc = r" }"]
#[doc = r" ```"]
macro_rules ! interrupt { ( $ NAME : ident , $ path : path , locals : { $ ( $ lvar : ident : $ lty : ty = $ lval : expr ; ) * } ) => { # [ allow ( non_snake_case ) ] mod $ NAME { pub struct Locals { $ ( pub $ lvar : $ lty , ) * } } # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ( $ lvar : $ lval , ) * } ; let f : fn ( & mut self :: $ NAME :: Locals ) = $ path ; f ( unsafe { & mut LOCALS } ) ; } } ; ( $ NAME : ident , $ path : path ) => { # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn ( ) = $ path ; f ( ) ; } } }
