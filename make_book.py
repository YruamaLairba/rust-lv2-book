#!/usr/bin/env python3
from pathlib import Path
import re
import itertools


class Line(object):
    def __init__(self, content):
        self.content = content

    def __str__(self):
        return self.content


class CommentLine(Line):
    pass


class CodeLine(Line):
    pass


class Block(object):
    def __init__(self):
        self.lines = list()

    def add_line(self, line):
        self.lines.append(line)

    def __iter__(self):
        return (str(line) for line in self.lines)


class CodeBlock(Block):
    def __init__(self, language):
        self.language = language
        self.lines = list()

    def __iter__(self):
        return itertools.chain(
            ["```{}".format(self.language)], super().__iter__(), ["```"]
        )


class Document(object):
    def __init__(self, path):
        def make_lines(raw_lines, language):
            if language == "rs":
                comment_indicator_re = re.compile(r"\s*//\s*([^\n]*)")
            else:
                comment_indicator_re = re.compile(r"\s*#\s*([^\n]*)")
            clean_line_re = re.compile(r"([^\n]+)")

            for line in raw_lines:
                is_comment = comment_indicator_re.match(line)
                if is_comment:
                    yield CommentLine(is_comment.group(1))
                else:
                    cleaned_line = clean_line_re.match(line)
                    if cleaned_line is not None:
                        yield CodeLine(cleaned_line.group(1))

        def lines_to_blocks(lines, language):
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

        path = Path(path)
        language = re.match(r".([^\n]*)", path.suffix).group(1)
        self.name = path.name
        with open(path, "r") as input:
            raw_lines = input.readlines()
        lines = make_lines(raw_lines, language)
        self.blocks = list(lines_to_blocks(lines, language))

    def __iter__(self):
        yield "### `{}`\n".format(self.name)
        for block in self.blocks:
            yield "\n"
            for line in block:
                yield str(line) + "\n"


class Chapter(object):
    def __init__(self, name, documents):
        self.name = name
        self.documents = [Document(path) for path in documents]

    def __iter__(self):
        yield "## {}\n\n".format(self.name)
        for doc in self.documents:
            for line in iter(doc):
                yield line


if __name__ == "__main__":
    amp = Chapter(
        "Simple Amplifier",
        [
            "amp/eg-amp-rs.lv2/manifest.ttl",
            "amp/eg-amp-rs.lv2/amp.ttl",
            "amp/Cargo.toml",
            "amp/src/lib.rs",
        ],
    )
    with open("book.md", "w") as output:
        output.writelines(iter(amp))
