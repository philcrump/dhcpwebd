
use timeago::{Language, TimeUnit};

#[derive(Default)]
pub struct EnglishShort;
impl Language for EnglishShort {
    fn clone_boxed(&self) -> timeago::BoxedLanguage { Box::new(Self{}) }
    fn too_low (&self) -> &'static str { "now" }
    fn too_high(&self) -> &'static str { "old" }
    fn ago(&self)      -> &'static str { "ago" }
    fn get_word(&self, tu: TimeUnit, x: u64) -> &'static str {
        use TimeUnit::*;
        if x == 1 {
            match tu {
                Nanoseconds   =>  "ns",
                Microseconds  =>  "us",
                Milliseconds  =>  "ms",
                Seconds       =>  "s",
                Minutes       =>  "m",
                Hours         =>  "h",
                Days          =>  "d",
                Weeks         =>  "w",
                Months        =>  "mo",
                Years         =>  "y",
            }
        } else {
            match tu {
                Nanoseconds   =>  "ns",
                Microseconds  =>  "us",
                Milliseconds  =>  "ms",
                Seconds       =>  "s",
                Minutes       =>  "m",
                Hours         =>  "h",
                Days          =>  "d",
                Weeks         =>  "w",
                Months        =>  "mo",
                Years         =>  "y",
            }
        }
    }
}