use core::marker::PhantomData;

/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
    /// The parts to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self) -> Self::Parts;
}

/// Marker trait for pin mode detection.
pub trait Mode<MODE> {}

// Unlocked Pull-Up Quasi-Bidirectional
pub struct Initial;

/// Input mode (type state)
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Floating input (type state)
pub struct Floating;
/// Pulled down input (type state)
pub struct PullDown;
/// Pulled up input (type state)
pub struct PullUp;

/// Output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Push pull output (type state)
pub struct PushPull;
/// Open drain output (type state)
pub struct OpenDrain;

/// Analog mode (type state)
pub struct Analog;

/// Alternate function
pub struct Alternate<MODE> {
    _mode: PhantomData<MODE>,
}
/*
pub enum State {
    High,
    Low,
}*/

macro_rules! gpio {
    ($GPX:ident, $GPX_BITS:ident, $gpiox:ident, $gpioy:ident, $PXx:ident, [
        $($PXi:ident: ($pxi:ident, $i:expr),)+
    ]) => {
        /// GPIO
        pub mod $gpiox {
            use core::convert::Infallible;
            use core::marker::PhantomData;

            use crate::hal::digital::v2::{/*InputPin, */OutputPin, StatefulOutputPin, toggleable};
            use crate::pac::{/*$gpioy, */$GPX, $GPX_BITS};

            use super::{
                /*Alternate, Floating, */GpioExt, /*Input,*/
                Initial,
                // OpenDrain,
                Output,
                //PullDown,
                //PullUp,
                PushPull,
                //Analog,
                //State,
                //Active,
                //Debugger,
                //Pxx,
                Mode,
                //Edge,
                //ExtiPin
            };

            /// GPIO parts
            pub struct Parts {
                $(
                    /// Pin
                    pub $pxi: $PXi<Initial>,
                )+
            }

            impl GpioExt for $GPX {
                type Parts = Parts;

                fn split(self) -> Parts {
                    Parts {
                        $(
                            $pxi: $PXi { _mode: PhantomData },
                        )+
                    }
                }
            }

            $(
                /// Pin
                pub struct $PXi<MODE> {
                    _mode: PhantomData<MODE>,
                }

                impl<MODE> Mode<MODE> for $PXi<MODE> {}

                impl<MODE> $PXi<MODE> {
                    /// Configures the pin to operate as an push pull output pin
                    pub fn into_push_pull_output(self) -> $PXi<Output<PushPull>> {
                        unsafe{ (*$GPX::ptr()).pmd.modify(|_,w| w.pmd($i).output()); }
                        unsafe{ (*$GPX::ptr()).dmask.modify(|_,w| w.dmask($i).updatable()); }
                        $PXi { _mode: PhantomData }
                    }
                }

                impl<MODE> OutputPin for $PXi<Output<MODE>> {
                    type Error = Infallible;

                    fn set_high(&mut self) -> Result<(), Self::Error> {
                        Ok(unsafe { (*$GPX_BITS::ptr()).dout[$i].write(|w| w.dout().high()) })
                    }

                    fn set_low(&mut self) -> Result<(), Self::Error> {
                        Ok(unsafe { (*$GPX_BITS::ptr()).dout[$i].write(|w| w.dout().low()) })
                    }
                }

                impl<MODE> StatefulOutputPin for $PXi<Output<MODE>> {
                    #[inline(always)]
                    fn is_set_high(&self) -> Result<bool, Self::Error> {
                        self.is_set_low().map(|b| !b)
                    }

                    fn is_set_low(&self) -> Result<bool, Self::Error> {
                        // NOTE(unsafe) atomic read with no side effects
                        Ok(unsafe { (*$GPX_BITS::ptr()).dout[$i].read().dout().is_low() })
                    }
                }

                impl<MODE> toggleable::Default for $PXi<Output<MODE>> {}
            )+
        }
    }
}

gpio!(GPA, GPA_BITS, gpa, gpa, PAx, [
    PA0: (pa0, 0),
    PA1: (pa1, 1),
    PA2: (pa2, 2),
    PA3: (pa3, 3),
    PA4: (pa4, 4),
    PA5: (pa5, 5),
    PA6: (pa6, 6),
    PA7: (pa7, 7),
    PA8: (pa8, 8),
    PA9: (pa9, 9),
    PA10: (pa10, 10),
    PA11: (pa11, 11),
    PA12: (pa12, 12),
    PA13: (pa13, 13),
    PA14: (pa14, 14),
    PA15: (pa15, 15),
]);
gpio!(GPB, GPB_BITS, gpb, gpa, PBx, [
    PB0: (pb0, 0),
    PB1: (pb1, 1),
    PB2: (pb2, 2),
    PB3: (pb3, 3),
    PB4: (pb4, 4),
    PB5: (pb5, 5),
    PB6: (pb6, 6),
    PB7: (pb7, 7),
    PB8: (pb8, 8),
    PB9: (pb9, 9),
    PB10: (pb10, 10),
    PB11: (pb11, 11),
    PB12: (pb12, 12),
    PB13: (pb13, 13),
    PB14: (pb14, 14),
    PB15: (pb15, 15),
]);
gpio!(GPC, GPC_BITS, gpc, gpa, PCx, [
    PC0: (pc0, 0),
    PC1: (pc1, 1),
    PC2: (pc2, 2),
    PC3: (pc3, 3),
    PC4: (pc4, 4),
    PC5: (pc5, 5),
    PC6: (pc6, 6),
    PC7: (pc7, 7),
    PC8: (pc8, 8),
    PC9: (pc9, 9),
    PC10: (pc10, 10),
    PC11: (pc11, 11),
    PC12: (pc12, 12),
    PC13: (pc13, 13),
    PC14: (pc14, 14),
    PC15: (pc15, 15),
]);
gpio!(GPD, GPD_BITS, gpd, gpa, PDx, [
    PD0: (pd0, 0),
    PD1: (pd1, 1),
    PD2: (pd2, 2),
    PD3: (pd3, 3),
    PD4: (pd4, 4),
    PD5: (pd5, 5),
    PD6: (pd6, 6),
    PD7: (pd7, 7),
    PD8: (pd8, 8),
    PD9: (pd9, 9),
    PD10: (pd10, 10),
    PD11: (pd11, 11),
    PD12: (pd12, 12),
    PD13: (pd13, 13),
    PD14: (pd14, 14),
    PD15: (pd15, 15),
]);
gpio!(GPE, GPE_BITS, gpe, gpa, PEx, [
    PE0: (pe0, 0),
    PE1: (pe1, 1),
    PE2: (pe2, 2),
    PE3: (pe3, 3),
    PE4: (pe4, 4),
    PE5: (pe5, 5),
    PE6: (pe6, 6),
    PE7: (pe7, 7),
    PE8: (pe8, 8),
    PE9: (pe9, 9),
    PE10: (pe10, 10),
    PE11: (pe11, 11),
    PE12: (pe12, 12),
    PE13: (pe13, 13),
    PE14: (pe14, 14),
    PE15: (pe15, 15),
]);
