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

#[cfg(test)]
mod tests {}
