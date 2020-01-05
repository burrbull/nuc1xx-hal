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
