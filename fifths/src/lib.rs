use lv2_atom::prelude::*;
use lv2_core::prelude::*;
use lv2_midi::*;
use lv2_units::prelude::*;
use lv2_urid::prelude::*;
use wmidi::*;

#[derive(PortContainer)]
pub struct Ports {
    input: InputPort<AtomPort>,
    output: OutputPort<AtomPort>,
}

#[derive(FeatureCollection)]
pub struct Features<'a> {
    map: Map<'a>,
}

pub struct Fifths {
    atom_urids: AtomURIDCache,
    midi_urids: MidiURIDCache,
    unit_urids: UnitURIDCache,
}

unsafe impl UriBound for Fifths {
    const URI: &'static [u8] = b"urn:rust-lv2-book:eg-fifths-rs\0";
}

impl Plugin for Fifths {
    type Ports = Ports;

    type Features = Features<'static>;

    fn new(_plugin_info: &PluginInfo, features: Features<'static>) -> Option<Self> {
        Some(Self {
            atom_urids: features.map.populate_cache()?,
            midi_urids: features.map.populate_cache()?,
            unit_urids: features.map.populate_cache()?,
        })
    }

    fn run(&mut self, ports: &mut Ports) {
        // Get the reading handle of the input sequence.
        let input_sequence = ports
            .input
            .read(self.atom_urids.sequence, self.unit_urids.beat)
            .unwrap();

        // Initialise the output sequence and get the writing handle.
        let mut output_sequence = ports
            .output
            .init(
                self.atom_urids.sequence,
                TimeStampURID::Frames(self.unit_urids.frame),
            )
            .unwrap();

        for (timestamp, atom) in input_sequence {
            // Forward message to output.
            output_sequence.forward(timestamp, atom);

            // Retrieve the message.
            let message = if let Some(message) = atom.read(self.midi_urids.event, ()) {
                message
            } else {
                continue;
            };

            match message {
                MidiMessage::NoteOn(channel, note, velocity) => {
                    // Make a note 5th (7 semitones) higher than input.
                    if let Ok(note) = note.step(7) {
                        // Write the fifth. Writing is done after initialization.
                        output_sequence
                            .init(
                                timestamp,
                                self.midi_urids.event,
                                MidiMessage::NoteOn(channel, note, velocity),
                            )
                            .unwrap();
                        println!("Wrote a note-on");
                    }
                }
                MidiMessage::NoteOff(channel, note, velocity) => {
                    // Do the same thing for `NoteOff`.
                    if let Ok(note) = note.step(7) {
                        output_sequence
                            .init(
                                timestamp,
                                self.midi_urids.event,
                                MidiMessage::NoteOff(channel, note, velocity),
                            )
                            .unwrap();
                            println!("Wrote a note-off");
                    }
                }
                _ => println!("Something different!"),
            }
        }
    }
}

lv2_descriptors!(Fifths);
