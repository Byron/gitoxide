use crate::{file, spanned};
use dangerous::{Bytes, BytesReader, Error, Expected, Input};

fn config(bytes: &[u8]) -> Result<Vec<file::Token>, Expected<'_>> {
    fn config<'i, E>(input: Bytes<'i>, r: &mut BytesReader<'i, E>) -> Result<Vec<file::Token>, E>
    where
        E: Error<'i>,
    {
        let mut tokens = Vec::new();
        skip_whitespace_or_comment(r, ConsumeTo::NextToken).map(|section| {
            tokens.push(spanned::Comment(
                dangerous::input(section)
                    .span_of(&input)
                    .expect("range contained")
                    .into(),
            ))
        });
        unimplemented!("sections and values");
    }
    let input = dangerous::input(bytes);
    input.read_all(|r| config(dangerous::input(bytes), r))
}

enum ConsumeTo {
    NextToken,
    EndOfLine,
}

#[must_use]
fn skip_whitespace_or_comment<'a, E>(r: &mut BytesReader<'a, E>, to_where: ConsumeTo) -> Option<&'a [u8]> {
    fn skip_whitespace_or_comment<E>(r: &mut BytesReader<'_, E>, to_where: ConsumeTo) {
        fn skip_comment<E>(r: &mut BytesReader<'_, E>) -> usize {
            if r.peek_eq(b'#') {
                r.take_while(|c| c != b'\n').len()
            } else {
                0
            }
        }

        let (mut last, mut current) = (0, 0);
        loop {
            current += skip_comment(r);
            current += r
                .take_while(|c| {
                    let iwb = c.is_ascii_whitespace();
                    iwb && match to_where {
                        ConsumeTo::NextToken => true,
                        ConsumeTo::EndOfLine => c != b'\n',
                    }
                })
                .len();
            if last == current {
                break;
            }
            last = current;
        }
    }
    let parsed = r
        .take_consumed(|r| skip_whitespace_or_comment(r, to_where))
        .as_dangerous();
    if parsed.is_empty() {
        None
    } else {
        Some(parsed)
    }
}

#[cfg(test)]
mod tests {
    mod comments {
        use crate::parse::{skip_whitespace_or_comment, ConsumeTo};
        use dangerous::Input;

        macro_rules! decode_span {
            ($name:ident, $input:literal, $option:path, $range:expr, $explain:literal) => {
                #[test]
                fn $name() {
                    let bytes = $input;
                    let (res, _remaining) =
                        dangerous::input(bytes).read_infallible(|r| skip_whitespace_or_comment(r, $option));
                    assert_eq!(
                        res.map(dangerous::input)
                            .and_then(|s| s.span_of(&dangerous::input(bytes))),
                        Some($range),
                        $explain
                    );
                }
            };
        }

        decode_span!(
            no_comment_till_next_token,
            b"     \n     \t\n",
            ConsumeTo::NextToken,
            0..13,
            "it consumes newlines as well, taking everything"
        );

        decode_span!(
            no_comment_to_end_of_line,
            b"     \n     \t ",
            ConsumeTo::EndOfLine,
            0..5,
            "it consumes only a single line, EXCLUDING the EOF marker"
        );

        decode_span!(
            comment_to_next_token,
            b" #ho \n     \t ",
            ConsumeTo::NextToken,
            0..13,
            "comments are the same as whitespace"
        );

        decode_span!(
            comment_to_end_of_line,
            b"# hi \n     \t ",
            ConsumeTo::EndOfLine,
            0..5,
            "comments are the same as whitespace"
        );

        decode_span!(
            whitespace_to_token,
            b"   a=2   \n     \t ",
            ConsumeTo::NextToken,
            0..3,
            "it does not consume tokens"
        );

        decode_span!(
            whitespace_to_token_on_next_line,
            b"    \n  b=2\t ",
            ConsumeTo::NextToken,
            0..7,
            "it does not consume tokens while skipping lines"
        );
    }
}
