use git_object::{bstr::ByteSlice, commit::ref_iter::Token, CommitRefIter};

use crate::{
    hex_to_id,
    immutable::{
        commit::{LONG_MESSAGE, MERGE_TAG, SIGNATURE},
        fixture_bytes, linus_signature, signature,
    },
};

#[test]
fn newline_right_after_signature_multiline_header() -> crate::Result {
    let data = fixture_bytes("commit", "signed-whitespace.txt");
    let tokens = CommitRefIter::from_bytes(&data).collect::<Result<Vec<_>, _>>()?;
    assert_eq!(tokens.len(), 7, "mainly a parsing exercise");
    match tokens.last().expect("there are tokens") {
        Token::Message(msg) => {
            assert!(msg.starts_with(b"Rollup"));
        }
        _ => unreachable!(),
    }
    Ok(())
}

#[test]
fn signed_with_encoding() -> crate::Result {
    assert_eq!(
        CommitRefIter::from_bytes(&fixture_bytes("commit", "signed-with-encoding.txt"))
            .collect::<Result<Vec<_>, _>>()?,
        vec![
            Token::Tree {
                id: hex_to_id("1973afa74d87b2bb73fa884aaaa8752aec43ea88")
            },
            Token::Parent {
                id: hex_to_id("79c51cc86923e2b8ca0ee5c4eb75e48027133f9a")
            },
            Token::Author {
                signature: signature(1592448995)
            },
            Token::Committer {
                signature: signature(1592449083)
            },
            Token::Encoding(b"ISO-8859-1".as_bstr()),
            Token::ExtraHeader((b"gpgsig".as_bstr(), SIGNATURE.as_bytes().as_bstr().into())),
            Token::Message(b"encoding & sig".as_bstr()),
        ]
    );
    Ok(())
}

#[test]
fn whitespace() -> crate::Result {
    assert_eq!(
        CommitRefIter::from_bytes(&fixture_bytes("commit", "whitespace.txt")).collect::<Result<Vec<_>, _>>()?,
        vec![
            Token::Tree {
                id: hex_to_id("9bed6275068a0575243ba8409253e61af81ab2ff")
            },
            Token::Parent {
                id: hex_to_id("26b4df046d1776c123ac69d918f5aec247b58cc6")
            },
            Token::Author {
                signature: signature(1592448450)
            },
            Token::Committer {
                signature: signature(1592448450)
            },
            Token::Message(b" nl".as_bstr())
        ]
    );
    Ok(())
}

#[test]
fn unsigned() -> crate::Result {
    assert_eq!(
        CommitRefIter::from_bytes(&fixture_bytes("commit", "unsigned.txt")).collect::<Result<Vec<_>, _>>()?,
        vec![
            Token::Tree {
                id: hex_to_id("1b2dfb4ac5e42080b682fc676e9738c94ce6d54d")
            },
            Token::Author {
                signature: signature(1592437401)
            },
            Token::Committer {
                signature: signature(1592437401)
            },
            Token::Message(b"without sig".as_bstr())
        ]
    );
    Ok(())
}

#[test]
fn signed_singleline() -> crate::Result {
    assert_eq!(
        CommitRefIter::from_bytes(&fixture_bytes("commit", "signed-singleline.txt")).collect::<Result<Vec<_>, _>>()?,
        vec![
            Token::Tree {
                id: hex_to_id("00fc39317701176e326974ce44f5bd545a32ec0b")
            },
            Token::Parent {
                id: hex_to_id("09d8d3a12e161a7f6afb522dbe8900a9c09bce06")
            },
            Token::Author {
                signature: signature(1592391367)
            },
            Token::Committer {
                signature: signature(1592391367)
            },
            Token::ExtraHeader((b"gpgsig".as_bstr(), b"magic:signature".as_bstr().into())),
            Token::Message(b"update tasks\n".as_bstr()),
        ]
    );
    Ok(())
}

#[test]
fn error_handling() -> crate::Result {
    let data = fixture_bytes("commit", "unsigned.txt");
    let iter = CommitRefIter::from_bytes(&data[..data.len() / 2]);
    let tokens = iter.collect::<Vec<_>>();
    assert!(
        tokens.last().expect("at least the errored token").is_err(),
        "errors are propagated and none is returned from that point on"
    );
    Ok(())
}

#[test]
fn mergetag() -> crate::Result {
    assert_eq!(
        CommitRefIter::from_bytes(&fixture_bytes("commit", "mergetag.txt")).collect::<Result<Vec<_>, _>>()?,
        vec![
            Token::Tree {
                id: hex_to_id("1c61918031bf2c7fab9e17dde3c52a6a9884fcb5")
            },
            Token::Parent {
                id: hex_to_id("44ebe016df3aad96e3be8f95ec52397728dd7701")
            },
            Token::Parent {
                id: hex_to_id("8d485da0ddee79d0e6713405694253d401e41b93")
            },
            Token::Author {
                signature: linus_signature(1591996221)
            },
            Token::Committer {
                signature: linus_signature(1591996221)
            },
            Token::ExtraHeader((b"mergetag".as_bstr(), MERGE_TAG.as_bytes().as_bstr().into())),
            Token::Message(LONG_MESSAGE.as_bytes().as_bstr()),
        ]
    );
    Ok(())
}

mod method {
    use git_object::CommitRefIter;

    use crate::{
        hex_to_id,
        immutable::{fixture_bytes, signature},
    };

    #[test]
    fn tree_id() -> crate::Result {
        assert_eq!(
            CommitRefIter::from_bytes(&fixture_bytes("commit", "unsigned.txt")).tree_id(),
            Some(hex_to_id("1b2dfb4ac5e42080b682fc676e9738c94ce6d54d"))
        );
        assert_eq!(
            CommitRefIter::from_bytes(&fixture_bytes("commit", "unsigned.txt"))
                .signatures()
                .collect::<Vec<_>>(),
            vec![signature(1592437401), signature(1592437401)]
        );
        Ok(())
    }

    #[test]
    fn signatures() -> crate::Result {
        assert_eq!(
            CommitRefIter::from_bytes(&fixture_bytes("commit", "unsigned.txt"))
                .signatures()
                .collect::<Vec<_>>(),
            vec![signature(1592437401), signature(1592437401)]
        );
        Ok(())
    }
}
