# Programming LV2 Plugins - Rust Edition

## Foreword

This book is an effort to translate the [LV2 Book by David Robillard](http://lv2plug.in/book/) for the [`rust-lv2`](https://github.com/RustAudio/rust-lv2.git) ecosystem. As such, the examples in this book as well as the README's and comments are copied from the original, but the book itself has been altered to adapt for the differences between C and Rust.

## Introduction

This is a series of well-documented example plugins that demonstrate the various features of LV2. Starting with the most basic plugin possible, each adds new functionality and explains the features used from a high level perspective.

API and vocabulary reference documentation explains details, but not the ``big picture''. This book is intended to complement the reference documentation by providing good reference implementations of plugins, while also conveying a higher-level understanding of LV2.

The chapters/plugins are arranged so that each builds incrementally on its predecessor. Reading this book front to back is a good way to become familiar with modern LV2 programming. The reader is expected to be familiar with Rust, but otherwise no special knowledge is required; the first plugin describes the basics in detail.

Each chapter corresponds to executable plugin code which can be found in the `samples` directory of the book's [Github Repository](https://github.com/RustAudio/rust-lv2-book). If you prefer to read actual source code, all the content here is also available in the source code as comments.
 
## Simple Amplifier

This plugin is a simple example of a basic LV2 plugin with no additional features. It has audio ports which contain an array of float, and a control port which contains a single float.

LV2 plugins are defined in two parts: code and data. The code provides an interface to the host written in C, but it can be written in any C-compatible language. Static data is described separately in the human and machine friendly Turtle syntax.

Generally, the goal is to keep code minimal, and describe as much as possible in the static data. There are several advantages to this approach:

* Hosts can discover and inspect plugins without loading or executing any plugin code.
* Plugin data can be used from a wide range of generic tools like scripting languages and command line utilities.
* The standard data model allows the use of existing vocabularies to describe plugins and related information.
* The language is extensible, so authors may describe any data without requiring changes to the LV2 specification.
* Labels and documentation are translatable, and available to hosts for display in user interfaces.

### amp/eg-amp-rs.lv2/manifest.ttl

LV2 plugins are installed in a `bundle`, a directory with a standard structure. Each bundle has a
Turtle file named `manifest.ttl` which lists the contents of the bundle.

Hosts typically read the manifest of every installed bundle to discover plugins on start-up, so it
should be as small as possible for performance reasons. Details that are only useful if the host
chooses to load the plugin are stored in other files and linked to from `manifest.ttl`.

In a crate, this should be a special folder that contains the Turtle files. After the crate was
build, the resulting shared object should also be copied into this folder.

#### URIs

LV2 makes use of URIs as globally-unique identifiers for resources. For example, the ID of the
plugin described here is `<urn:rust-lv2-book:eg-amp-rs>`. Note that URIs are only used as identifiers
and don't necessarily imply that something can be accessed at that address on the web (though that
may be the case).

#### Namespace Prefixes

Turtle files contain many URIs, but prefixes can be defined to improve readability. For example,
with the `lv2:` prefix below, `lv2:Plugin` can be written instead of 
`<http://lv2plug.in/ns/lv2core#Plugin>`.

```ttl

@prefix lv2:  <http://lv2plug.in/ns/lv2core#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

```

#### Describing a Plugin

Turtle files contain a set of statements which describe resources. This file contains 3 statements:

| Subject                         | Predicate      | Object       |
|---------------------------------|----------------|--------------|
| `<urn:rust-lv2-book:eg-amp-rs>` | `a `           | `lv2:Plugin` |
| `<urn:rust-lv2-book:eg-amp-rs>` | `lv2:binary`   | `<amp.so>`   |
| `<urn:rust-lv2-book:eg-amp-rs>` | `rdfs:seeAlso` | `<amp.ttl> ` |

Firstly, `<urn:rust-lv2-book:eg-amp-rs>` is an LV2 plugin:

```ttl

<urn:rust-lv2-book:eg-amp-rs> a lv2:Plugin .

```

The predicate `a` is a Turtle shorthand for `rdf:type`.

The binary of that plugin can be found at `<amp.ext>`:

```ttl

<urn:rust-lv2-book:eg-amp-rs> lv2:binary <libamp.so> .

```

This line is platform-dependet since it assumes that shared objects have the `.so` ending. On Windows, it should be ending with `.dll`.
Relative URIs in manifests are relative to the bundle directory, so this refers to a binary with
the given name in the same directory as this manifest.

Finally, more information about this plugin can be found in `<amp.ttl>`:

```ttl

<urn:rust-lv2-book:eg-amp-rs> rdfs:seeAlso <amp.ttl> .
```

### amp/eg-amp-rs.lv2/amp.ttl

The full description of the plugin is in this file, which is linked to from
`manifest.ttl`.  This is done so the host only needs to scan the relatively
small `manifest.ttl` files to quickly discover all plugins.

```ttl

@prefix doap:  <http://usefulinc.com/ns/doap#> .
@prefix lv2:   <http://lv2plug.in/ns/lv2core#> .
@prefix rdf:   <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix rdfs:  <http://www.w3.org/2000/01/rdf-schema#> .
@prefix units: <http://lv2plug.in/ns/extensions/units#> .

```

First the type of the plugin is described.  All plugins must explicitly list
`lv2:Plugin` as a type.  A more specific type should also be given, where
applicable, so hosts can present a nicer UI for loading plugins.  Note that
this URI is the identifier of the plugin, so if it does not match the one in
`manifest.ttl`, the host will not discover the plugin data at all.

```ttl
<urn:rust-lv2-book:eg-amp-rs>
        a lv2:Plugin ,
                lv2:AmplifierPlugin ;
```

Plugins are associated with a project, where common information like
developers, home page, and so on are described.  This plugin is part of the
rust-lv2-book project, which has URI <https://github.com/Janonard/rust-lv2-book>, and is described
elsewhere.  Typical plugin collections will describe the project in
manifest.ttl

```ttl
        lv2:project <https://github.com/Janonard/rust-lv2-book> ;
```

Every plugin must have a name, described with the doap:name property.
Translations to various languages can be added by putting a language tag
after strings as shown later.

```ttl
        doap:name "Simple Amplifier (Rust Version)" ;
        doap:license <http://opensource.org/licenses/isc> ;
        lv2:optionalFeature lv2:hardRTCapable ;
        lv2:port [
```

Every port must have at least two types, one that specifies direction
(lv2:InputPort or lv2:OutputPort), and another to describe the data type.
This port is a lv2:ControlPort, which means it contains a single float.

```ttl
                a lv2:InputPort ,
                        lv2:ControlPort ;
                lv2:index 0 ;
                lv2:symbol "gain" ;
                lv2:name "Gain" ,
                        "收益"@ch ,
                        "Gewinn"@de ,
                        "Gain"@en-gb ,
                        "Aumento"@es ,
                        "Gain"@fr ,
                        "Guadagno"@it ,
                        "利益"@jp ,
                        "Увеличение"@ru ;
```

An lv2:ControlPort should always describe its default value, and usually a
minimum and maximum value.  Defining a range is not strictly required, but
should be done wherever possible to aid host support, particularly for UIs.

```ttl
                lv2:default 0.0 ;
                lv2:minimum -90.0 ;
                lv2:maximum 24.0 ;
```

Ports can describe units and control detents to allow better UI generation
and host automation.

```ttl
                units:unit units:db ;
                lv2:scalePoint [
                        rdfs:label "+5" ;
                        rdf:value 5.0
                ] , [
                        rdfs:label "0" ;
                        rdf:value 0.0
                ] , [
                        rdfs:label "-5" ;
                        rdf:value -5.0
                ] , [
                        rdfs:label "-10" ;
                        rdf:value -10.0
                ]
        ] , [
                a lv2:AudioPort ,
                        lv2:InputPort ;
                lv2:index 1 ;
                lv2:symbol "in" ;
                lv2:name "In"
        ] , [
                a lv2:AudioPort ,
                        lv2:OutputPort ;
                lv2:index 2 ;
                lv2:symbol "out" ;
                lv2:name "Out"
        ] .
```

### amp/Cargo.toml


```toml
[package]
name = "amp"
version = "0.2.0"
authors = ["Jan-Oliver 'Janonard' Opdenhövel <jan.opdenhoevel@protonmail.com>"]
license = "ISC"
edition = "2018"

```

Plugins are dynamic libraries. This setting tells cargo to export it this way.

```toml
[lib]
crate-type = ["dylib"]

```

This simple example only needs the core crate.

```toml
[dependencies]
lv2-core = { git = "https://github.com/RustAudio/rust-lv2.git" }
```

### amp/src/lib.rs

Include the prelude of the core crate. Every specification is implemented by exactly one crate and their preludes always contain most of the types needed to use them in a plugin.

```rs
use lv2_core::prelude::*;

```

Most useful plugins will have ports for input and output data. In code, these ports are represented by a struct implementing the `PortContainer` trait. Internally, ports are referred to by index. These indices are assigned in ascending order, starting with 0 for the first port. The indices in `amp.ttl` have to match them.

```rs
#[derive(PortContainer)]
struct Ports {
    gain: InputPort<Control>,
    input: InputPort<Audio>,
    output: OutputPort<Audio>,
}

```

Every plugin defines a struct for the plugin instance. All persistent data associated with a plugin instance is stored here, and is available to every instance method. In this simple plugin, there is no additional instance data and therefore, this struct is empty.

```rs
struct Amp;

```

The URI is the identifier for a plugin, and how the host associates this implementation in code with its description in data. If this URI does not match that used in the data files, the host will fail to load the plugin. Since many other things are also identified by URIs, there is a separate trait for them: The `UriBound`. It stores the URI as a constant null-terminated byte slice and provides a method to easily retrieve the URI. If the null-terminator is omitted, some other parts of the system may cause undefined behaviour. Since this can not checked by Rust's type system, this trait has to be unsafe.

```rs
unsafe impl UriBound for Amp {
    const URI: &'static [u8] = b"urn:rust-lv2-book:eg-amp-rs\0";
}

```

Every plugin struct implements the `Plugin` trait.

```rs
impl Plugin for Amp {
```

Set the ports type.

```rs
    type Ports = Ports;

```

This plugin does not use additional host features and therefore, we set the features collection type to `()`. Other plugins define a separate struct with their required and optional features and set it here.

```rs
    type Features = ();

```

The `new` method is called by the plugin backend when it creates a new plugin instance. The host passes the plugin URI, sample rate, and bundle path for plugins that need to load additional resources (e.g. waveforms). The features parameter contains host-provided features defined in LV2 extensions, but this simple plugin does not use any. This method is in the “instantiation” threading class, so no other methods on this instance will be called concurrently with it.

```rs
    fn new(_plugin_info: &PluginInfo, _features: ()) -> Option<Self> {
        Some(Self)
    }

```

The `run()` method is the main process function of the plugin. It processes a block of audio in the audio context. Since this plugin is `lv2:hardRTCapable`, `run()` must be real-time safe, so blocking (e.g. with a mutex) or memory allocation are not allowed.

```rs
    fn run(&mut self, ports: &mut Ports) {
        let coef = if *(ports.gain) > -90.0 {
            10.0_f32.powf(*(ports.gain) * 0.05)
        } else {
            0.0
        };

        for (in_frame, out_frame) in Iterator::zip(ports.input.iter(), ports.output.iter_mut()) {
            *out_frame = in_frame * coef;
        }
    }
}

```

The `lv2_descriptors` macro creates the entry point to the plugin library. It takes structs that implement `Plugin` and exposes them. The host will load the library and call a generated function to find all the plugins defined in the library.

```rs
lv2_descriptors!(Amp);
```

## MIDI Gate

This plugin demonstrates:
* Receiving MIDI input
* Processing audio based on MIDI events with sample accuracy
* Supporting MIDI programs which the host can control/automate, or present a user interface for with human readable labels

### midigate/eg-midigate-rs.lv2/manifest.ttl

The manifest.ttl file follows the same template as the previous example.

```ttl

@prefix lv2:  <http://lv2plug.in/ns/lv2core#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix ui:   <http://lv2plug.in/ns/extensions/ui#> .

<urn:rust-lv2-book:eg-midigate-rs>
	a lv2:Plugin ;
	lv2:binary <libmidigate.so> ;
	rdfs:seeAlso <midigate.ttl> .
```

### midigate/eg-midigate-rs.lv2/midigate.ttl

The same set of namespace prefixes with two additions for LV2 extensions this
plugin uses: atom and urid.

```ttl

@prefix atom: <http://lv2plug.in/ns/ext/atom#> .
@prefix doap: <http://usefulinc.com/ns/doap#> .
@prefix lv2:  <http://lv2plug.in/ns/lv2core#> .
@prefix midi: <http://lv2plug.in/ns/ext/midi#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix urid: <http://lv2plug.in/ns/ext/urid#> .

<urn:rust-lv2-book:eg-midigate-rs>
	a lv2:Plugin ;
	doap:name "Example MIDI Gate (Rust Version)" ;
	doap:license <http://opensource.org/licenses/isc> ;
    lv2:project <https://github.com/Janonard/rust-lv2-book> ;
	lv2:requiredFeature urid:map ;
	lv2:optionalFeature lv2:hardRTCapable ;
```

This plugin has three ports.  There is an audio input and output as before,
as well as a new AtomPort.  An AtomPort buffer contains an Atom, which is a
generic container for any type of data.  In this case, we want to receive
MIDI events, so the (mandatory) `atom:bufferType` is atom:Sequence, which is
a series of events with time stamps.

Events themselves are also generic and can contain any type of data, but in
this case we are only interested in MIDI events.  The (optional)
`atom:supports` property describes which event types are supported.  Though
not required, this information should always be given so the host knows what
types of event it can expect the plugin to understand.

The (optional) `lv2:designation` of this port is `lv2:control`, which
indicates that this is the "main" control port where the host should send
events it expects to configure the plugin, in this case changing the MIDI
program.  This is necessary since it is possible to have several MIDI input
ports, though typically it is best to have one.

```ttl
	lv2:port [
		a lv2:InputPort ,
			atom:AtomPort ;
		atom:bufferType atom:Sequence ;
		atom:supports midi:MidiEvent ;
		lv2:designation lv2:control ;
		lv2:index 0 ;
		lv2:symbol "control" ;
		lv2:name "Control"
	] , [
		a lv2:AudioPort ,
			lv2:InputPort ;
		lv2:index 1 ;
		lv2:symbol "in" ;
		lv2:name "In"
	] , [
		a lv2:AudioPort ,
			lv2:OutputPort ;
		lv2:index 2 ;
		lv2:symbol "out" ;
		lv2:name "Out"
	] .
```

### midigate/Cargo.toml


```toml
[package]
name = "midigate"
version = "0.1.0"
authors = ["Jan-Oliver 'Janonard' Opdenhövel <jan.opdenhoevel@protonmail.com>"]
edition = "2018"

[lib]
crate-type = ["dylib"]

[dependencies]
wmidi = "3.1.0"
lv2-core = { git = "https://github.com/RustAudio/rust-lv2.git" }
lv2-urid = { git = "https://github.com/RustAudio/rust-lv2.git" }
lv2-atom = { git = "https://github.com/RustAudio/rust-lv2.git" }
lv2-units = { git = "https://github.com/RustAudio/rust-lv2.git" }
lv2-midi = { git = "https://github.com/RustAudio/rust-lv2.git", features = ["wmidi"]}
```

### midigate/src/lib.rs


```rs
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
    map: Map<'a>,
}

pub struct Midigate {
    n_active_notes: u64,
    program: u8,
    atom_urids: AtomURIDCache,
    midi_urids: MidiURIDCache,
    unit_urids: UnitURIDCache,
}

unsafe impl UriBound for Midigate {
    const URI: &'static [u8] = b"urn:rust-lv2-book:eg-midigate-rs\0";
}

impl Midigate {
```

A function to write a chunk of output, to be called from `run()`. If the gate is high, then the input will be passed through for this chunk, otherwise silence is written.

```rs
    fn write_output(&mut self, ports: &mut Ports, offset: usize, mut len: usize) {
```

check the bounds of the offset and length and cap the length, if nescessary.

```rs
        if ports.input.len() < offset + len {
            len = ports.input.len() - offset;
        }

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

```

The core crate handles feature detection for the plugin.

```rs
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

```

This plugin works through the cycle in chunks starting at offset zero. The `offset` represents the current time within this this cycle, so the output from 0 to `offset` has already been written.

MIDI events are read in a loop. In each iteration, the number of active notes (on note on and note off) or the program (on program change) is updated, then the output is written up until the current event time. Then `offset` is updated and the next event is processed. After the loop the final chunk from the last event to the end of the cycle is emitted.

There is currently no standard way to describe MIDI programs in LV2, so the host has no way of knowing that these programs exist and should be presented to the user. A future version of LV2 will address this shortcoming.

This pattern of iterating over input events and writing output along the way is a common idiom for writing sample accurate output based on event input.

Note that this simple example simply writes input or zero for each sample based on the gate. A serious implementation would need to envelope the transition to avoid aliasing.

```rs
    fn run(&mut self, ports: &mut Ports) {
        let mut offset: usize = 0;

        let control_sequence = ports
            .control
            .read(self.atom_urids.sequence, self.unit_urids.beat)
            .unwrap();

        for (timestamp, message) in control_sequence {
            let timestamp: usize = if let Some(timestamp) = timestamp.as_frames() {
                timestamp as usize
            } else {
                continue;
            };

            let message = if let Some(message) = message.read(self.midi_urids.wmidi, ()) {
                message
            } else {
                continue;
            };

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

            self.write_output(ports, offset, timestamp + offset);
            offset += timestamp;
        }

        self.write_output(ports, offset, ports.input.len() - offset);
    }
}

lv2_descriptors!(Midigate);
```

## Fifths

This plugin demonstrates simple MIDI event reading and writing.
### fifths/eg-fifths-rs.lv2/manifest.ttl


```ttl
@prefix lv2:  <http://lv2plug.in/ns/lv2core#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix ui:   <http://lv2plug.in/ns/extensions/ui#> .

<urn:rust-lv2-book:eg-fifths-rs>
	a lv2:Plugin ;
	lv2:binary <libfifths.so> ;
	rdfs:seeAlso <fifths.ttl> .
```

### fifths/eg-fifths-rs.lv2/fifths.ttl


```ttl
@prefix atom:  <http://lv2plug.in/ns/ext/atom#> .
@prefix doap:  <http://usefulinc.com/ns/doap#> .
@prefix lv2:   <http://lv2plug.in/ns/lv2core#> .
@prefix urid:  <http://lv2plug.in/ns/ext/urid#> .
@prefix midi:  <http://lv2plug.in/ns/ext/midi#> .

<urn:rust-lv2-book:eg-fifths-rs>
	a lv2:Plugin ;
	doap:name "Example Fifths (Rust Edition)" ;
	doap:license <http://opensource.org/licenses/isc> ;
    lv2:project <https://github.com/Janonard/rust-lv2-book> ;
	lv2:requiredFeature urid:map ;
	lv2:optionalFeature lv2:hardRTCapable ;
	lv2:port [
		a lv2:InputPort ,
			atom:AtomPort ;
		atom:bufferType atom:Sequence ;
		atom:supports midi:MidiEvent ;
		lv2:index 0 ;
		lv2:symbol "in" ;
		lv2:name "In"
	] , [
		a lv2:OutputPort ,
			atom:AtomPort ;
		atom:bufferType atom:Sequence ;
		atom:supports midi:MidiEvent ;
		lv2:index 1 ;
		lv2:symbol "out" ;
		lv2:name "Out"
	] .
```

### fifths/Cargo.toml


```toml
[package]
name = "fifths"
version = "0.1.0"
authors = ["Jan-Oliver 'Janonard' Opdenhövel <jan.opdenhoevel@protonmail.com>"]
edition = "2018"

[lib]
crate-type = ["dylib"]

[dependencies]
wmidi = "3.1.0"
lv2-core = { git = "https://github.com/RustAudio/rust-lv2.git" }
lv2-urid = { git = "https://github.com/RustAudio/rust-lv2.git" }
lv2-atom = { git = "https://github.com/RustAudio/rust-lv2.git" }
lv2-units = { git = "https://github.com/RustAudio/rust-lv2.git" }
lv2-midi = { git = "https://github.com/RustAudio/rust-lv2.git", features = ["wmidi"]}
```

### fifths/src/lib.rs


```rs
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
```

Get the reading handle of the input sequence.

```rs
        let input_sequence = ports
            .input
            .read(self.atom_urids.sequence, self.unit_urids.beat)
            .unwrap();

```

Initialise the output sequence and get the writing handle.

```rs
        let mut output_sequence = ports
            .output
            .init(
                self.atom_urids.sequence,
                TimeStampURID::Frames(self.unit_urids.frame),
            )
            .unwrap();

        for (timestamp, atom) in input_sequence {
```

Forward message to output.

```rs
            output_sequence.forward(timestamp, atom);

```

Retrieve the message.

```rs
            let message = if let Some(message) = atom.read(self.midi_urids.wmidi, ()) {
                message
            } else {
                continue;
            };

            match message {
                MidiMessage::NoteOn(channel, note, velocity) => {
```

Make a note 5th (7 semitones) higher than input.

```rs
                    if let Ok(note) = note.step(7) {
```

Write the fifth. Writing is done after initialization.

```rs
                        output_sequence
                            .init(
                                timestamp,
                                self.midi_urids.wmidi,
                                MidiMessage::NoteOn(channel, note, velocity),
                            )
                            .unwrap();
                        println!("Wrote a note-on");
                    }
                }
                MidiMessage::NoteOff(channel, note, velocity) => {
```

Do the same thing for `NoteOff`.

```rs
                    if let Ok(note) = note.step(7) {
                        output_sequence
                            .init(
                                timestamp,
                                self.midi_urids.wmidi,
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
```
