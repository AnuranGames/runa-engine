// use crate::components::audio_source::AudioSource;
// use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};
// use std::collections::HashMap;
// use std::io::Cursor;
// use std::sync::{Arc, Mutex};

// pub struct AudioEngine {
//     _stream: OutputStream,
//     stream_handle: rodio::OutputStream,
//     sinks: Mutex<HashMap<usize, rodio::Sink>>,
//     next_id: usize,
// }

// impl AudioEngine {
//     pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
//         let (_stream, stream_handle) = OutputStream::try_default()?;

//         Ok(Self {
//             _stream,
//             sinks: Mutex::new(HashMap::new()),
//             next_id: 0,
//         })
//     }
// }
