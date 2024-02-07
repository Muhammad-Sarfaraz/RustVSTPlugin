#[macro_use] extern crate vst2;

use vst2::buffer::AudioBuffer;
use vst2::plugin::{Plugin, Info};

#[derive(Default)]
struct FirstAttempt;

impl Plugin for FirstAttempt {
    fn get_info(&self) -> Info {
        Info {
            name: "FirstAttempt".to_string(),
            vendor: "Sarfaraz".to_string(),
            unique_id: 25032017,

            ..Info::default()
        }
    }
}

plugin_main!(FirstAttempt);