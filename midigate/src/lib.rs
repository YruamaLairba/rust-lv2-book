use lv2_atom::prelude::*;
use lv2_core::prelude::*;
use lv2_midi::*;
use lv2_units::prelude::*;
use lv2_urid::prelude::*;
use wmidi::*;

#[derive(PortContainer)]
pub struct Ports {
    control: InputPort<AtomPort>,
    input: InputPort<Audio>,
    output: OutputPort<Audio>,
}

#[derive(FeatureCollection)]
pub struct Features<'a> {
    map: &'a Map<'a>,
}

pub struct Midigate {
    n_active_notes: u64,
    program: u8,
    atom_urids: AtomURIDCache,
    midi_urids: MidiURIDCache,
    unit_urids: UnitURIDCache,
}

unsafe impl UriBound for Midigate {
    const URI: &'static [u8] = b"urn:rust-lv2-book:eg-midigate-rs";
}

impl Midigate {
    fn write_output(&mut self, ports: &mut Ports, offset: usize, len: usize) {
        let active = if self.program == 0 {
            self.n_active_notes > 0
        } else {
            self.n_active_notes == 0
        };

        let input = &ports.input[offset..offset + len];
        let output = &mut ports.output[offset..offset + len];

        if active {
            output.copy_from_slice(input);
        } else {
            for frame in output.iter_mut() {
                *frame = 0.0;
            }
        }
    }
}

impl Plugin for Midigate {
    type Ports = Ports;

    type Features = Features<'static>;

    fn new(_plugin_info: &PluginInfo, features: Features<'static>) -> Option<Self> {
        Some(Self {
            n_active_notes: 0,
            program: 0,
            atom_urids: features.map.populate_cache()?,
            midi_urids: features.map.populate_cache()?,
            unit_urids: features.map.populate_cache()?,
        })
    }

    fn activate(&mut self) {
        self.n_active_notes = 0;
        self.program = 0;
    }

    fn run(&mut self, ports: &mut Ports) {
        let mut offset: usize = 0;

        let message_iter = ports
            .control
            .read(self.atom_urids.sequence, self.unit_urids.beat);

        if let Some(message_iter) = message_iter {
            for (timestamp, message) in message_iter {
                if let Some(message) = message.read(self.midi_urids.event, ()) {
                    if let Some(timestamp) = timestamp.as_frames() {
                        match message {
                            MidiMessage::NoteOn(_, _, _) => self.n_active_notes += 1,
                            MidiMessage::NoteOff(_, _, _) => self.n_active_notes -= 1,
                            MidiMessage::ProgramChange(_, program) => {
                                let program: u8 = program.into();
                                if program == 0 || program == 1 {
                                    self.program = program;
                                }
                            }
                            _ => (),
                        }

                        let timestamp = timestamp as usize;
                        self.write_output(ports, offset, timestamp + offset);
                        offset += timestamp;
                    }
                }
            }

            self.write_output(ports, offset, ports.input.len() - offset);
        }
    }
}

lv2_descriptors!(Midigate);
