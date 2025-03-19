#!/usr/local/bin/python3 -tt
# -*- coding: utf-8 -*-

from typing import Literal
import argparse
from fontTools import subset
from pathlib import Path
import pyperclip as pc
import sys
from subsetter import font_face, hash


def eprint(*args, **kwargs):
    print(*args, file=sys.stderr, **kwargs)


def get_subset_font_path(input_path: Path, hash: str, format: str) -> Path:
    input_filename = Path(input_path.name).stem
    output_filename_str = "{}__subset_{}.{}".format(input_filename, hash, format)

    return Path(".").joinpath(output_filename_str)


def write_subset_font(
    input_path: Path,
    text: str,
    hash: str,
    format: Literal["ttf", "woff", "woff2"],
):
    try:
        output_path = get_subset_font_path(input_path, hash, format)
    except TypeError as e:
        print("\n[ Error ] {}".format(e))
        sys.exit(1)

    if format == "woff2" or format == "woff":
        options = subset.Options(flavor=format)
    else:
        options = subset.Options()

    try:
        font = subset.load_font(input_path, options)
    except FileNotFoundError:
        message = "\n[ Error ] Missing input font file `{}`".format(input_path)
        eprint(message)
        raise FileNotFoundError(message)

    fonttools_subsetter = subset.Subsetter(options)
    fonttools_subsetter.populate(text=text)
    fonttools_subsetter.subset(font)
    try:
        subset.save_font(font, output_path, options)
    except PermissionError:
        message = f"\n[ Error ] Unable to write font file `{output_path}`.  Check the "
        "path exists with write permissions."
        eprint(message)
        raise PermissionError(message)
    print("Wrote {} subset to {}".format(format, str(output_path)))

    return str(output_path)


def write_subset_font_file_for_format(font_file_path_str: str, text: str, hash: str):
    extension = Path(font_file_path_str).suffix
    format = extension[1:]
    if format == "woff2" or format == "woff" or format == "ttf":
        try:
            subset_file_path = write_subset_font(
                Path(font_file_path_str), text, hash, format
            )
        except FileNotFoundError:
            sys.exit(1)
        except PermissionError:
            sys.exit(1)

        return subset_file_path

    message = "Unrecognised file format for font file {}".format(font_file_path_str)
    eprint(message)
    raise ValueError(message)


def main():
    # parse CLI arguments
    parser = argparse.ArgumentParser(
        prog="subsetter", description="Generate font subset CSS and font files."
    )
    parser.add_argument("-f", "--family", help="Font family name.")
    parser.add_argument("-t", "--text", help="Text fragment to create subset for.")
    parser.add_argument(
        "-w", "--weight", type=int, help="Font weight to generate @fontface CSS for."
    )
    parser.add_argument(
        "font_files", nargs="*", help="Input woff2, woff and ttf font files"
    )
    family = parser.parse_args().family
    text = parser.parse_args().text
    weight_int = parser.parse_args().weight

    # generate subsets for requested formats
    hash_value = hash(text)
    font_file_path_list = parser.parse_args().font_files
    subset_file_path_list = []
    for font_file_path_str in font_file_path_list:
        subset_file_path = write_subset_font_file_for_format(
            font_file_path_str, text, hash_value
        )
        if len(subset_file_path) > 0:
            subset_file_path_list.append(subset_file_path)

    # generate CSS
    css = font_face(family, weight_int, text, subset_file_path_list)
    print(css)
    pc.copy(css)
    print("Copied @fontface CSS to clipboard")


if __name__ == "__main__":
    main()
