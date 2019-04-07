extern crate lv2rs as lv2;

use lv2::atom::ports::AtomInputPort;
use lv2::atom::prelude::*;
use lv2::atom::sequence::TimeStamp;
use lv2::core::{self, ports::*, *};
use lv2::midi::{MidiMessage, RawMidiMessage};
use lv2::urid::*;
use std::ffi::CStr;

pub struct Midigate {
    control_port: AtomInputPort<Sequence>,
    in_port: AudioInputPort,
    null: Vec<f32>,
    out_port: AudioOutputPort,

    urid_map: CachedMap,

    n_active_notes: i32,
}

impl Midigate {
    fn assure_null_len(&mut self, min_len: usize) {
        if self.null.len() < min_len {
            let n_new_frames: usize = min_len - self.null.len();
            self.null.reserve(n_new_frames);
            for _ in 0..n_new_frames {
                self.null.push(0.0);
            }
        }
    }
}

impl Plugin for Midigate {
    fn instantiate(
        _descriptor: &Descriptor,
        rate: f64,
        _bundle_path: &CStr,
        features: Option<&FeaturesList>,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        let features = features?;
        // Try to create a `CacheMap`. It maps URIs to integers, called URIDs, and saves the mappings in
        /// a `HashMap`.
        let cached_map = CachedMap::try_from_features(features)?;

        let mut plugin = Self {
            control_port: AtomInputPort::new(),
            in_port: AudioInputPort::new(),
            null: Vec::with_capacity(rate as usize),
            out_port: AudioOutputPort::new(),

            urid_map: cached_map,

            n_active_notes: 0,
        };

        // Allocate null space for one second of frames. This should be enough for most cases.
        plugin.assure_null_len(rate as usize);

        Some(plugin)
    }

    fn activate(&mut self) {
        self.n_active_notes = 0;
    }

    fn connect_port(&mut self, port: u32, data: *mut ()) {
        match port {
            0 => self.control_port.connect_port(data as *const Atom),
            1 => self.in_port.connect(data as *const f32),
            2 => self.out_port.connect(data as *mut f32),
            _ => (),
        }
    }

    fn run(&mut self, n_samples: u32) {
        /// Assure that we have enough null space. Since we are `lv2:hardRTCapable`, we should not
        /// allocate memory, but we previously allocated one second of null-space and therefore we
        /// will never lag. If we're in a real-time environment, the block sizes won't be longer
        /// than a second, and will never allocate new null-space. Iif we're not, than allocation
        /// time does not matter.
        self.assure_null_len(n_samples as usize);

        /// This plugin works through the cycle in chunks starting at offset zero. The offset
        /// represents the current time within this this cycle, so the output from 0 to offset has
        /// already been written.
        ///
        /// This pattern of iterating over input events and writing output along the way is a common
        /// idiom for writing sample accurate output based on event input.
        ///
        /// Note that this simple example simply writes input or zero for each sample based on the
        /// gate. A serious implementation would need to envelope the transition to avoid aliasing.
        let mut offset: usize = 0;

        let events_atom = unsafe { self.control_port.get_atom_body(&mut self.urid_map) }.unwrap();
        let audio_input = unsafe { self.in_port.as_slice(n_samples) }.unwrap();
        let null_input = &self.null.as_slice()[0..(n_samples as usize)];
        let audio_output = unsafe { self.out_port.as_slice(n_samples) }.unwrap();

        for (time_stamp, midi_event) in events_atom.iter(&mut self.urid_map) {
            let midi_event: &RawMidiMessage = {
                match unsafe { midi_event.get_body(&mut self.urid_map) } {
                    Ok(midi_event) => midi_event,
                    Err(_) => continue,
                }
            };

            let midi_event = match midi_event.interpret() {
                Ok(event) => event,
                Err(_) => continue,
            };

            // receiving note-ons and note-offs.
            match midi_event {
                MidiMessage::NoteOn {
                    channel: _,
                    note: _,
                    velocity: _,
                } => {
                    self.n_active_notes += 1;
                }
                MidiMessage::NoteOff {
                    channel: _,
                    note: _,
                    velocity: _,
                } => {
                    self.n_active_notes -= 1;
                }
                _ => (),
            }

            let time: usize = match time_stamp {
                TimeStamp::Frames(frames) => frames as usize,
                TimeStamp::Beats(_) => panic!("We can't handle beats!"),
            };

            let input = if self.n_active_notes > 0 {
                &audio_input[offset..time]
            } else {
                &null_input[offset..time]
            };
            audio_output[offset..time].copy_from_slice(input);

            offset += time;
        }

        let time = n_samples as usize;
        let input = if self.n_active_notes > 0 {
            &audio_input[offset..time]
        } else {
            &null_input[offset..time]
        };
        audio_output[offset..time].copy_from_slice(input);
    }
}

lv2_main!(core, Midigate, b"urn:lv2rs-book:eg-midigate-rs\0");
