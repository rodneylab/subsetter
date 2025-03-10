use std::{
    ops::Range,
    path::{Path, PathBuf},
};

use anyhow::bail;
use base64::{Engine, prelude::BASE64_STANDARD};
use xxhash_rust::xxh3::xxh3_64;

const BITS_TO_POP: u32 = 32;

fn load_font_base64<P: AsRef<Path>>(file: P) -> String {
    let data = std::fs::read(file).unwrap();
    BASE64_STANDARD.encode(&data)
}

pub struct FontFaceParameters {
    font_family: String,
    font_weight: u32,
    text: String,
    woff2_input_path: Option<PathBuf>,
    woff_input_path: Option<PathBuf>,
    ttf_input_path: Option<PathBuf>,
    output_base_url: Option<String>,
}

impl FontFaceParameters {
    pub fn new<P: AsRef<Path>>(
        font_family: &str,
        font_weight: u32,
        text: &str,
        font_file_paths: &[P],
        output_base_url: Option<&str>,
    ) -> anyhow::Result<Self> {
        if font_file_paths.is_empty() {
            bail!("Missing font files");
        }
        let mut ttf_input_path: Option<PathBuf> = None;
        let mut woff_input_path: Option<PathBuf> = None;
        let mut woff2_input_path: Option<PathBuf> = None;
        for file in font_file_paths {
            if let Some(value) = Path::new(file.as_ref()).extension() {
                if value == "ttf" {
                    ttf_input_path = Some(file.as_ref().to_path_buf());
                } else if value == "woff" {
                    woff_input_path = Some(file.as_ref().to_path_buf());
                } else if value == "woff2" {
                    woff2_input_path = Some(file.as_ref().to_path_buf());
                }
            }
        }
        Ok(Self {
            font_family: font_family.to_string(),
            font_weight,
            text: text.to_string(),
            woff2_input_path,
            woff_input_path,
            ttf_input_path,
            output_base_url: output_base_url.map(std::string::ToString::to_string),
        })
    }
}

fn add_font_src<P: AsRef<Path>>(
    format: &str,
    input_path: Option<P>,
    output_base_url: Option<&str>,
    src_elements: &mut Vec<String>,
) {
    if let Some(value) = input_path {
        let font_src_url_value = if let Some(output_base_url_value) = output_base_url {
            if src_elements.is_empty() {
                None
            } else {
                let basename = value.as_ref().file_stem().unwrap().to_str().unwrap();
                Some(format!(
                    "url('{output_base_url_value}{basename}.{format}') format('{format}')",
                ))
            }
        } else {
            None
        };
        let font_src = font_src_url_value.unwrap_or_else(|| {
            let base64 = load_font_base64(value);
            format!("url('data:font/{format};base64, {base64}') format('{format}')")
        });
        src_elements.push(font_src);
    }
}

pub fn font_face(
    FontFaceParameters {
        font_family,
        font_weight,
        text,
        woff2_input_path,
        woff_input_path,
        ttf_input_path,
        output_base_url,
    }: &FontFaceParameters,
) -> String {
    debug_assert!(
        woff2_input_path.is_some() || woff_input_path.is_some() || ttf_input_path.is_some()
    );
    let mut src_elements: Vec<String> = Vec::new();
    add_font_src(
        "woff2",
        woff2_input_path.as_deref(),
        // todo(rodneylab): add size check to make sure inlining is sensible
        None, // always use inline base64: woff2 will be first in graceful degradation sequence.
        &mut src_elements,
    );
    add_font_src(
        "woff",
        woff_input_path.as_deref(),
        output_base_url.as_deref(),
        &mut src_elements,
    );
    add_font_src(
        "ttf",
        ttf_input_path.as_deref(),
        output_base_url.as_deref(),
        &mut src_elements,
    );
    let src = src_elements.join(",\n        ");

    let unicode_range = unicodes(text);
    let font_family_value = if font_family.contains(' ') {
        format!(r#""{font_family}__subset""#)
    } else {
        format!("{font_family}__subset")
    };
    let result = format!(
        "@font-face {{
    font-display: swap;
    font-family: {font_family_value};
    font-stretch: normal;
    font-style: normal;
    font-weight: {font_weight};
    src:
        {src};
    unicode-range: {unicode_range};
  }}"
    );

    result
}
fn format_range(range: &Range<u32>) -> String {
    let Range { start, end } = range;
    let range_span = end - start;
    match range_span {
        0 => String::new(), // empty range
        1 => format!("U+{:X}", { *start }),
        2 => format!("U+{:X}, U+{:X}", { *start }, { *end - 1 }),
        3.. => format!("U+{:X}-{:X}", { *start }, { *end - 1 }),
    }
}

/// Extract characters from text, sort them and remove duplicate elements
fn sort_characters(text: &str) -> Vec<char> {
    let mut characters: Vec<char> = text.chars().collect();
    characters.sort_unstable();
    characters.dedup();

    characters
}

pub fn hash(text: &str) -> String {
    let characters: String = sort_characters(text).into_iter().collect();
    let hash = xxh3_64(characters.as_bytes()) >> BITS_TO_POP;

    format!("{hash:x}")
}

pub fn unicodes(text: &str) -> String {
    let characters = sort_characters(text);
    let mut collapsed: String = String::with_capacity(characters.len());
    let mut collapsed_vec: Vec<u32> = Vec::with_capacity(characters.len());
    for character in &characters {
        collapsed.push(*character);
        collapsed_vec.push(*character as u32);
    }

    let mut ranges: Vec<Range<u32>> = Vec::new();
    let mut start = collapsed_vec[0];
    let mut end = start + 1;
    for character in &collapsed_vec[1..] {
        if *character == end {
            end = *character + 1;
        } else {
            ranges.push(Range { start, end });
            start = *character;
            end = start + 1;
        }
    }
    end = collapsed_vec[collapsed_vec.len() - 1] + 1;
    ranges.push(Range { start, end });

    let mut ranges_string = String::new();
    for range in &ranges[..ranges.len() - 1] {
        ranges_string.push_str(&format_range(range));
        ranges_string.push_str(", ");
    }
    ranges_string.push_str(&format_range(&ranges[ranges.len() - 1]));

    ranges_string
}

#[cfg(test)]
mod tests {
    use super::hash;

    #[test]
    fn hash_generates_expected_result_for_valid_input() {
        // arrange
        let text = "Halloa!";

        // act
        let result = hash(text);

        // assert
        assert_eq!(result, String::from("69178b77"));
    }
}
