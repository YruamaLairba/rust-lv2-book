#!/usr/bin/env python3
from pathlib import Path
import re
import itertools
import argparse


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


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Compile a Markdown file from a source file."
    )
    parser.add_argument(
        "-i",
        "--input",
        type=str,
        required=True,
        nargs=1,
        help="Path to input source file.",
    )
    parser.add_argument(
        "-o", "--output", type=str, required=True, nargs=1, help="Path to output file."
    )
    args = parser.parse_args()

    document = Document(args.input[0])
    with open(args.output[0], "w") as output:
        output.writelines(str().join(iter(document)))

