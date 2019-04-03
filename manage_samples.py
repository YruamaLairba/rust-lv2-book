#!/usr/bin/python3
import os
import sys
import shutil
from os.path import expanduser

CARGO_MISSING_MESSAGE = """cargo could not be found!

You need a complete rust toolchain to build the examples. Please visit rustup.rs!"""
USAGE = """USAGE:
Build: {exe} build
Clean: {exe} clean
Install: {exe} install
Uninstall: {exe} uninstall
"""

EXAMPLES = ["amp", "midigate"]


def print_usage():
    print(USAGE.format(exe=sys.argv[0]))


def build_example(name):
    # Build the crate.
    if os.system("cargo build --release") != 0:
        quit()

    # Copy the shared object.
    shutil.copy(
        "target/release/libeg_{}_rs.so".format(name),
        "eg-{name}-rs.lv2/{name}.so".format(name=name),
    )


def clean_example(name):
    os.system("cargo clean")
    os.remove("eg-{name}-rs.lv2/{name}.so".format(name=name))


def install_example(name):
    build_example(name)

    uninstall_example(name)

    # Install the new plugin.
    shutil.copytree(
        "eg-{name}-rs.lv2/".format(name=name),
        expanduser("~/.lv2/eg-{name}-rs.lv2/".format(name=name)),
    )


def uninstall_example(name):
    shutil.rmtree(
        expanduser("~/.lv2/eg-{name}-rs.lv2/".format(name=name)), ignore_errors=True
    )


def main():
    if shutil.which("cargo") is None:
        print(CARGO_MISSING_MESSAGE)

    if len(sys.argv) != 2:
        print("No command given!\n")
        print_usage()
        return

    command = sys.argv[1]

    for name in EXAMPLES:
        os.chdir("samples/{}".format(name))

        if command == "build":
            build_example(name)
        elif command == "clean":
            clean_example(name)
        elif command == "install":
            install_example(name)
        elif command == "uninstall":
            uninstall_example(name)
        else:
            print("Invalid command {}\n".format(command))
            print_usage()
            return

        os.chdir("../..")


if __name__ == "__main__":
    main()
