#!/usr/bin/env python3
from pathlib import Path
import re
import itertools
import argparse


class Line(object):
    """
    Base class for a document line.
    """

    def __init__(self, content):
        self.content = content

    def __str__(self):
        return self.content


class CommentLine(Line):
    """
    A line that was a comment in the source document.
    It will be outputed as normal text.
    """

    pass


class CodeLine(Line):
    """
    A line that was source code in the source document.
    It will be outputed as quoted code.
    """

    pass


class Block(object):
    """
    Base class for a block of lines. This class represents normal, unmodified text.
    """

    def __init__(self):
        self.lines = list()

    def add_line(self, line):
        """
        Add another line to the block.
        `line` is supposed to be a `Line` object and has to be of the same type as the other lines
        in this block.
        """
        self.lines.append(line)

    def __str__(self):
        return "\n".join(map(str, self.lines))


class CodeBlock(Block):
    """
    A block of code lines. It has additional features to export the lines as quoted code.
    """

    def __init__(self, language):
        self.language = language
        self.lines = list()

    def __str__(self):
        return "\n```{}\n".format(self.language) + "\n".join(map(str, self.lines)) + "\n```\n"

class File(object):

    def __init__(self, path, language=None):
        self.path = path

        # Determine the language of the source.
        if language is None:
            language = re.match(r".([^\n]*)", path.suffix).group(1)

        # Read the raw lines fromt the input file.
        with open(path, "r") as input:
            raw_lines = input.readlines()

        # Retrieve an iterator over all line object.
        lines = make_lines(raw_lines, language)
        # Retrieve an iterator over all blocks.
        self.blocks = list(lines_to_blocks(lines, language))

    def __str__(self):
        return "### {}\n\n".format(str(self.path)) + "\n".join(map(str, self.blocks))

class Chapter(object):

    def __init__(self, intro_path, paths):
        self.intro = "".join(open(intro_path, "r").readlines())
        self.files = list()
        for path in paths:
            self.files.append(File(Path(path)))

    def __str__(self):
        return self.intro + "\n" + "\n".join(map(str, self.files))

class Book(object):

    def __init__(self, intro_path, chapters):
        self.intro = "".join(open(intro_path, "r").readlines())
        self.chapters = chapters

    def __str__(self):
        return "{} \n".format(self.intro) + "\n".join(map(str, self.chapters))

def make_lines(raw_lines, language):
    """
    Iterate through the raw lines and yield either a `CommentLine` or a `CodeLine`.
    """
    # Depending on the language, other comment indicators are used.
    if language == "rs":
        comment_indicator_re = re.compile(r"\s*//\s*([^\n]*)")
    else:
        comment_indicator_re = re.compile(r"\s*#\s*([^\n]*)")
    # A RE to clean a code line of the new-line character and to remove empty code lines.
    clean_line_re = re.compile(r"([^\n]*)")

    for line in raw_lines:
        is_comment = comment_indicator_re.match(line)
        if is_comment:
            yield CommentLine(is_comment.group(1))
        else:
            cleaned_line = clean_line_re.match(line)
            if cleaned_line is not None:
                yield CodeLine(cleaned_line.group(1))


def lines_to_blocks(lines, language):
    """
    Iterate through the lines and group them in blocks.
    """
    last_block = None
    for line in lines:
        if last_block is None:
            new_block = True
        elif type(last_block.lines[-1]) != type(line):
            yield last_block
            new_block = True
        else:
            new_block = False

        if new_block:
            if type(line) == CodeLine:
                last_block = CodeBlock(language)
            else:
                last_block = Block()

        last_block.add_line(line)
    yield last_block


def build_document(path):
    """
    Create the markdown document from the source file in `path`.
    """
    # Determine the language of the source.
    language = re.match(r".([^\n]*)", path.suffix).group(1)

    # Read the raw lines fromt the input file.
    with open(path, "r") as input:
        raw_lines = input.readlines()

    # Retrieve an iterator over all line object.
    lines = make_lines(raw_lines, language)
    # Retrieve an iterator over all blocks.
    blocks = lines_to_blocks(lines, language)

    # Generator function to produce each single line in the document.
    def blocks_as_lines(blocks):
        for block in blocks:
            yield "\n"
            for line in block:
                yield str(line) + "\n"

    # Join the blocks into one string.
    return str().join(blocks_as_lines(blocks))

amp = Chapter("introductions/amp.md", [
    "amp/eg-amp-rs.lv2/amp.ttl",
    "amp/eg-amp-rs.lv2/manifest.ttl",
    "amp/Cargo.toml",
    "amp/src/lib.rs",
])

midigate = Chapter("introductions/midigate.md", [
    "midigate/eg-midigate-rs.lv2/midigate.ttl",
    "midigate/eg-midigate-rs.lv2/manifest.ttl",
    "midigate/Cargo.toml",
    "midigate/src/lib.rs"
])

book = Book("introductions/intro.md", [
    amp,
    midigate
])

open("export/book.md", "w").write(str(book))