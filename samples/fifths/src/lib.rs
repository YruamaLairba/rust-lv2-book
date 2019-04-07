extern crate lv2rs as lv2;
extern crate ux;

use lv2::atom::ports::*;
use lv2::core::*;
use lv2::midi::{MidiMessage, RawMidiMessage};
use lv2::prelude::*;
use lv2::urid::*;
use std::ffi::CStr;

/// The Fifths plugin.
///
/// It forwards every midi message, but also adds the fifth of every note on/note off message it
/// receives.
struct Fifths {
    urids: CachedMap,

    input: AtomInputPort<Sequence>,
    output: AtomOutputPort<Sequence>,
}

/// Little helper function to shift a note up a fifth or cap it at maximum.
fn shift_note(note: u7) -> u7 {
    let note: u8 = note.into();
    let note: u8 = note + 7;
    if note > u7::MAX.into() {
        u7::MAX
    } else {
        u7::new(note)
    }
}

impl Plugin for Fifths {
    fn instantiate(
        _descriptor: &Descriptor,
        _rate: f64,
        _bundle_path: &CStr,
        features: Option<&FeaturesList>,
    ) -> Option<Self> {
        Some(Self {
            urids: CachedMap::try_from_features(features?)?,

            input: AtomInputPort::new(),
            output: AtomOutputPort::new(),
        })
    }

    fn connect_port(&mut self, port: u32, data: *mut ()) {
        match port {
            0 => self.input.connect_port(data as *const Atom),
            1 => self.output.connect_port(data as *mut Atom),
            _ => (),
        }
    }

    fn run(&mut self, _n_samples: u32) {
        // Getting the input sequence, the used time unit and the writing frame for the output.
        let input_sequence = unsafe { self.input.get_atom_body(&mut self.urids) }.unwrap();
        let time_unit = input_sequence.time_unit(&mut self.urids);
        let mut output_frame =
            unsafe { self.output.write_atom_body(&time_unit, &mut self.urids) }.unwrap();

        // Iterating over all input events.
        for (time_stamp, atom) in input_sequence.iter(&mut self.urids) {
            // Get the midi event.
            let midi_event: &RawMidiMessage = {
                match atom.get_body(&mut self.urids) {
                    Ok(event) => event,
                    Err(_) => continue,
                }
            };

            // Interpret it (wrap it into the `MidiMessage` enum).
            let message = match midi_event.interpret() {
                Ok(message) => message,
                Err(_) => continue,
            };

            // Forward the original message.
            match output_frame.push_event::<RawMidiMessage>(
                time_stamp.clone(),
                &message,
                &mut self.urids,
            ) {
                Ok(_) => (),
                Err(_) => return, // The host didn't give us enough space, we need to give up.
            }

            // Construct the second message.
            let second_message: Option<MidiMessage> = match message {
                MidiMessage::NoteOn {
                    channel,
                    note,
                    velocity,
                } => Some(MidiMessage::NoteOn {
                    channel: channel,
                    note: shift_note(note),
                    velocity: velocity,
                }),
                MidiMessage::NoteOff {
                    channel,
                    note,
                    velocity,
                } => Some(MidiMessage::NoteOff {
                    channel: channel,
                    note: shift_note(note),
                    velocity: velocity,
                }),
                _ => None,
            };

            // Unwrap the second message.
            let second_message = match second_message {
                Some(message) => message,
                None => continue,
            };

            // Write the second message.
            match output_frame.push_event::<RawMidiMessage>(
                time_stamp,
                &second_message,
                &mut self.urids,
            ) {
                Ok(_) => (),
                Err(_) => return, // Again, we have not enough space.
            }
        }
    }
}

use lv2::core as lv2_core;
lv2_main!(lv2_core, Fifths, b"urn:lv2rs-book:eg-fifths-rs\0");
