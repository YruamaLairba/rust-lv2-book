use lv2_atom::prelude::*;
use lv2_atom::space::*;
use lv2_atom::Atom;
use lv2_core::prelude::*;
use lv2_units::prelude::*;
use lv2_urid::prelude::*;
use std::f64;
use std::f64::consts::PI;

#[allow(unused_imports)]
use lv2_sys::{
    LV2_TIME__Position, LV2_TIME__Rate, LV2_TIME__Time, LV2_TIME__bar, LV2_TIME__barBeat,
    LV2_TIME__beat, LV2_TIME__beatUnit, LV2_TIME__beatsPerBar, LV2_TIME__beatsPerMinute,
    LV2_TIME__frame, LV2_TIME__framesPerSecond, LV2_TIME__position, LV2_TIME__speed,
    LV2_TIME_PREFIX, LV2_TIME_URI,
};

struct TimePosition;
unsafe impl UriBound for TimePosition {
    const URI: &'static [u8] = LV2_TIME__Position;
}

struct TimeBarBeat;
unsafe impl UriBound for TimeBarBeat {
    const URI: &'static [u8] = LV2_TIME__barBeat;
}

struct TimeBeatPerMinute;
unsafe impl UriBound for TimeBeatPerMinute {
    const URI: &'static [u8] = LV2_TIME__beatsPerMinute;
}

struct TimeSpeed;
unsafe impl UriBound for TimeSpeed {
    const URI: &'static [u8] = LV2_TIME__speed;
}

pub struct Blank;

unsafe impl UriBound for Blank {
    const URI: &'static [u8] = lv2_sys::LV2_ATOM__Blank;
}

impl URIDBound for Blank {
    type CacheType = URID<Blank>;

    fn urid(urid: &URID<Blank>) -> URID<Blank> {
        *urid
    }
}

impl<'a, 'b> Atom<'a, 'b> for Blank
where
    'a: 'b,
{
    type ReadParameter = <Object as Atom<'a, 'b>>::ReadParameter;
    type ReadHandle = <Object as Atom<'a, 'b>>::ReadHandle;
    type WriteParameter = <Object as Atom<'a, 'b>>::WriteParameter;
    type WriteHandle = <Object as Atom<'a, 'b>>::WriteHandle;

    fn read(body: Space<'a>, parameter: Self::ReadParameter) -> Option<Self::ReadHandle> {
        Object::read(body, parameter)
    }

    fn init(
        frame: FramedMutSpace<'a, 'b>,
        parameter: Self::WriteParameter,
    ) -> Option<Self::WriteHandle> {
        Object::init(frame, parameter)
    }
}

#[derive(PortContainer)]
pub struct Ports {
    control: InputPort<AtomPort>,
    output: OutputPort<Audio>,
}

#[derive(FeatureCollection)]
pub struct Features<'a> {
    map: Map<'a>,
}

const ATTACK_S: f64 = 0.005;
const DECAY_S: f64 = 0.075;

enum State {
    Attack,
    Decay,
    Off,
}

pub struct Metro {
    atom_urids: AtomURIDCache,
    unit_urids: UnitURIDCache,
    blank: URID<Blank>,
    time_position_urid: URID<TimePosition>,
    time_barBeat_urid: URID<TimeBarBeat>,
    time_beatPerMinute_urid: URID<TimeBeatPerMinute>,
    time_speed_urid: URID<TimeSpeed>,

    rate: f64,  // Sample rate
    bpm: f32,   // Beat per minute (tempo)
    speed: f32, // Transport speed (usually 0=stop, 1=play)

    elapsed_len: u32,   // Frames since the start of the last click
    wave_offset: usize, // Current play offset in the wave
    state: State,       // current play state

    // One cycle of a sine wave
    wave: Vec<f32>,

    // Envelope parameters
    attack_len: u32,
    decay_len: u32,
}

unsafe impl UriBound for Metro {
    const URI: &'static [u8] = b"urn:rust-lv2-book:eg-metro-rs\0";
}

impl Metro {
    fn play(&mut self, ports: &mut Ports, begin: usize, end: usize) {
        let frames_per_beat: f32 = 60f32 / self.bpm * self.rate as f32;

        if self.speed == 0f32 {
            ports.output.iter_mut().for_each(|x| *x = 0f32);
            return;
        }

        for i in begin..end {
            match self.state {
                State::Attack => {
                    //Amplitude increase until attack_len
                    ports.output[i] = self.wave[self.wave_offset] * self.elapsed_len as f32
                        / self.attack_len as f32;
                    if self.elapsed_len >= self.attack_len {
                        self.state = State::Decay;
                    }
                }
                State::Decay => {
                    //Amplitude decrease until attack_len + decay_len
                    ports.output[i] = self.wave[self.wave_offset]
                        * (1f32
                            - ((self.elapsed_len - self.attack_len) as f32
                                / self.decay_len as f32));
                    if self.elapsed_len >= self.attack_len + self.decay_len {
                        self.state = State::Off;
                    }
                }
                State::Off => {
                    ports.output[i] = 0f32;
                }
            }
            //We continuously play the sine wave regardless of the envelope
            self.wave_offset = (self.wave_offset + 1) % self.wave.len();

            //Update elapsed time and start attack if necessary
            self.elapsed_len += 1;
            if self.elapsed_len == frames_per_beat as u32 {
                self.state = State::Attack;
                self.elapsed_len = 0;
            }
        }
    }

    fn update_position(&mut self, object_reader: lv2_atom::object::ObjectReader) {
        //Received new transport position/speed
        for (property_header, atom) in object_reader {
            if property_header.key == self.time_beatPerMinute_urid {
                if let Some(val) = atom.read(self.atom_urids.float, ()) {
                    self.bpm = val;
                }
            }
            if property_header.key == self.time_speed_urid {
                if let Some(val) = atom.read(self.atom_urids.float, ()) {
                    self.speed = val;
                }
            }
            if property_header.key == self.time_barBeat_urid {
                if let Some(val) = atom.read(self.atom_urids.float, ()) {
                    // Receveid a beat position, synchronise
                    // This hard sync may cause clicks, a real plugin would
                    // be more graceful
                    let frames_per_beat = 60f32 / self.bpm * self.rate as f32;
                    let bar_beats = val;
                    let beat_beats = bar_beats - bar_beats.floor();
                    self.elapsed_len = (beat_beats * frames_per_beat) as u32;
                    if self.elapsed_len < self.attack_len {
                        self.state = State::Attack;
                    } else if self.elapsed_len < self.attack_len + self.decay_len {
                        self.state = State::Decay;
                    } else {
                        self.state = State::Off;
                    }
                }
            }
        }
    }
}

impl Plugin for Metro {
    type Ports = Ports;

    type Features = Features<'static>;

    fn new(_plugin_info: &PluginInfo, features: Features<'static>) -> Option<Self> {
        // Generate one cycle of a sine wave at the desired frequency
        let rate: f64 = _plugin_info.sample_rate();
        let freq: f64 = 440.0 * 2.0;
        let amp: f64 = 0.5;
        let wave_len: usize = (_plugin_info.sample_rate() / freq) as usize;
        let mut wave: Vec<f32> = Vec::with_capacity(wave_len);
        for i in 0..wave_len {
            wave.push((f64::sin(i as f64 * 2f64 * PI * freq / rate) * amp) as f32);
        }

        let res = Self {
            atom_urids: features.map.populate_cache()?,
            unit_urids: features.map.populate_cache()?,
            blank: features.map.map_type()?,
            time_position_urid: features.map.populate_cache()?,
            time_barBeat_urid: features.map.populate_cache()?,
            time_beatPerMinute_urid: features.map.populate_cache()?,
            time_speed_urid: features.map.populate_cache()?,

            rate: _plugin_info.sample_rate(),
            bpm: 120f32,
            speed: 0f32,

            elapsed_len: 0,
            wave_offset: 0,
            state: State::Off,

            wave: wave,

            attack_len: (ATTACK_S * _plugin_info.sample_rate()) as u32,
            decay_len: (DECAY_S * _plugin_info.sample_rate()) as u32,
        };
        Some(res)
    }

    fn run(&mut self, ports: &mut Ports) {
        // Get the reading handle of the input sequence.
        let sequence = ports
            .control
            .read(self.atom_urids.sequence, self.unit_urids.beat)
            .unwrap();
        // Work forwards in time frame by frame, handling events as we go
        let mut last_t = 0;
        for event in sequence {
            let (timestamp, atom): (TimeStamp, UnidentifiedAtom) = event;

            // Play the click for the time slice from last_t until now
            let frames = timestamp.as_frames().unwrap() as usize;
            self.play(ports, last_t, frames);

            // Update time for next iteration
            last_t = frames;

            let (header, reader) =
                if let Some((header, reader)) = atom.read(self.atom_urids.object, ()) {
                    (header, reader)
                } else if let Some((header, reader)) = atom.read(self.blank, ()) {
                    (header, reader)
                } else {
                    continue;
                };

            if header.otype == self.time_position_urid {
                self.update_position(reader);
            }
        }

        // Play for remainder of cycle
        self.play(ports, last_t, ports.output.len());
    }
}

lv2_descriptors!(Metro);
