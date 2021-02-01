use crate::file;
use dangerous::{BytesReader, Error};

fn read_config<'i, E>(r: &mut BytesReader<'i, E>) -> Result<Vec<file::Token>, E>
where
    E: Error<'i>,
{
    skip_whitespace_or_comment(r, ConsumeTo::NextToken);
    unimplemented!("sections and values");
}

enum ConsumeTo {
    NextToken,
    EndOfLine,
}

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

        #[test]
        fn whitespace_only() {
            let bytes = b"     \n     \t ";
            let (res, remaining) =
                dangerous::input(bytes).read_infallible(|r| skip_whitespace_or_comment(r, ConsumeTo::NextToken));
            assert!(remaining.is_empty());
            assert_eq!(
                res.map(dangerous::input)
                    .and_then(|s| s.span_of(&dangerous::input(bytes))),
                Some(0..bytes.len())
            );
        }
    }
}
