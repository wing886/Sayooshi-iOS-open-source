use anyhow::{Result, bail};
use siglus_compiler_common::{derive_exe_key, lzss_pack, utf16le, xor_cycle};

#[derive(Debug, Clone)]
pub struct CompileOptions {
    pub fixed_key: Vec<u8>,
    pub exe_key: Option<[u8; 16]>,
}

/// Mirrors `C_ini_file_analizer::inia_comment_cut`.
///
/// ASCII a-z is uppercased outside quoted strings. `;` and `//` line
/// comments and `/* ... */` block comments are removed while newlines inside
/// comments are retained. Only `\\` and `\"` escapes are accepted in strings.
pub fn preprocess(source: &str) -> Result<String> {
    #[derive(Clone, Copy, PartialEq)]
    enum State {
        Normal,
        Quote,
        QuoteEscape,
        LineComment,
        BlockComment,
    }

    let chars: Vec<char> = source.chars().collect();
    let mut out = String::with_capacity(source.len());
    let mut state = State::Normal;
    let mut line = 1usize;
    let mut block_start = 1usize;
    let mut i = 0usize;
    while i < chars.len() {
        let ch = chars[i];
        let next = chars.get(i + 1).copied();
        if ch == '\n' {
            if matches!(state, State::Quote | State::QuoteEscape) {
                bail!("line {line}: newline inside double-quoted string");
            }
            if state == State::LineComment {
                state = State::Normal;
            }
            out.push(ch);
            line += 1;
            i += 1;
            continue;
        }

        match state {
            State::Quote => {
                out.push(ch);
                if ch == '\\' {
                    state = State::QuoteEscape;
                } else if ch == '"' {
                    state = State::Normal;
                }
                i += 1;
            }
            State::QuoteEscape => {
                if ch != '\\' && ch != '"' {
                    bail!(
                        "line {line}: invalid string escape \\{ch}; only \\\\ and \\\" are accepted"
                    );
                }
                out.push(ch);
                state = State::Quote;
                i += 1;
            }
            State::LineComment => i += 1,
            State::BlockComment => {
                if ch == '*' && next == Some('/') {
                    state = State::Normal;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            State::Normal => {
                if ch == '"' {
                    state = State::Quote;
                    out.push(ch);
                    i += 1;
                } else if ch == ';' {
                    state = State::LineComment;
                    i += 1;
                } else if ch == '/' && next == Some('/') {
                    state = State::LineComment;
                    i += 2;
                } else if ch == '/' && next == Some('*') {
                    state = State::BlockComment;
                    block_start = line;
                    i += 2;
                } else {
                    out.push(if ch.is_ascii_lowercase() {
                        ch.to_ascii_uppercase()
                    } else {
                        ch
                    });
                    i += 1;
                }
            }
        }
    }
    match state {
        State::Quote | State::QuoteEscape => bail!("line {line}: unclosed double quote"),
        State::BlockComment => bail!("line {block_start}: unclosed block comment"),
        _ => Ok(out),
    }
}

/// Produces the exact C++ pipeline:
/// preprocess -> UTF-16LE -> LZSS -> fixed XOR -> optional executable XOR.
pub fn compile(source: &str, options: &CompileOptions) -> Result<Vec<u8>> {
    if options.fixed_key.is_empty() {
        bail!("fixed Gameexe XOR key cannot be empty");
    }
    let processed = preprocess(source)?;
    let mut out = Vec::new();
    out.extend_from_slice(&0i32.to_le_bytes());
    let exe_key = options
        .exe_key
        .filter(|key| key.iter().any(|&byte| byte != 0));
    out.extend_from_slice(&(exe_key.is_some() as i32).to_le_bytes());
    // The original compiler emits only the header for an empty input file.
    if !source.is_empty() {
        let payload = utf16le(&processed);
        let mut packed = lzss_pack(&payload);
        xor_cycle(&mut packed, &options.fixed_key);
        if let Some(exe_key) = exe_key {
            xor_cycle(&mut packed, &exe_key);
        }
        out.extend_from_slice(&packed);
    }
    Ok(out)
}

pub fn derive_exe_key_from_cp932(text: &str) -> Result<[u8; 16]> {
    let (encoded, _, had_errors) = encoding_rs::SHIFT_JIS.encode(text);
    if had_errors {
        bail!("executable key string contains characters not representable in Windows CP932");
    }
    if encoded.len() < 8 {
        bail!(
            "executable key material must encode to at least 8 bytes (got {})",
            encoded.len()
        );
    }
    Ok(derive_exe_key(&encoded))
}

#[cfg(test)]
mod tests {
    use super::*;
    use siglus_compiler_common::{RECOVERED_GAMEEXE_KEY, lzss_unpack};

    #[test]
    fn preprocessing_matches_cpp_states() {
        let got = preprocess("abc=\"MiX;/*x*/\";gone\nq/*x\ny*/=z").unwrap();
        assert_eq!(got, "ABC=\"MiX;/*x*/\"\nQ\n=Z");
    }

    #[test]
    fn output_decodes_to_preprocessed_utf16() {
        let out = compile(
            "foo=\"Bar\"; comment\n",
            &CompileOptions {
                fixed_key: RECOVERED_GAMEEXE_KEY.to_vec(),
                exe_key: None,
            },
        )
        .unwrap();
        let mut packed = out[8..].to_vec();
        xor_cycle(&mut packed, &RECOVERED_GAMEEXE_KEY);
        let raw = lzss_unpack(&packed).unwrap();
        let units: Vec<u16> = raw
            .as_chunks::<2>()
            .0
            .iter()
            .map(|c| u16::from_le_bytes([c[0], c[1]]))
            .collect();
        assert_eq!(String::from_utf16(&units).unwrap(), "FOO=\"Bar\"\n");
    }

    #[test]
    fn zero_exe_key_selects_unencrypted_header() {
        let out = compile(
            "foo=1\n",
            &CompileOptions {
                fixed_key: RECOVERED_GAMEEXE_KEY.to_vec(),
                exe_key: Some([0; 16]),
            },
        )
        .unwrap();
        assert_eq!(&out[4..8], &0i32.to_le_bytes());
    }
}
