# ExAmp

This plugin is a simple example of a basic LV2 plugin with no additional features. It has audio ports which contain an array of float, and a control port which contains a single float.

LV2 plugins are defined in two parts: code and data. The code is written in any C compatible language, in our case Rust. Static data is described separately in the human and machine friendly Turtle syntax.

Generally, the goal is to keep code minimal, and describe as much as possible in the static data. There are several advantages to this approach:

* Hosts can discover and inspect plugins without loading or executing any plugin code.
* Plugin data can be used from a wide range of generic tools like scripting languages and command line utilities.
* The standard data model allows the use of existing vocabularies to describe plugins and related information.
* The language is extensible, so authors may describe any data without requiring changes to the LV2 specification.
* Labels and documentation are translatable, and available to hosts for display in user interfaces.


## Usage

This repository contains a handy script that compiles the plugin, installs it to the users home directory for plugins and runs it using `jalv`. Simply install `jalv`, clone the repo, open a terminal and type `./run_plugin.sh`.