use gix_object::{bstr::ByteSlice, commit::ref_iter::Token, CommitRefIter};

use crate::{
    commit::{LONG_MESSAGE, MERGE_TAG, SIGNATURE},
    fixture_name, hex_to_id, linus_signature, signature,
};

#[test]
fn signed_singleline() -> crate::Result {
    Commit::from_bytes(&fixture_name("commit", "signed-singleline.txt"))?.into_raw_iter(),
    assert_eq!(
        
        CommitRef {
            tree: b"00fc39317701176e326974ce44f5bd545a32ec0b".as_bstr(),
            parents: SmallVec::from(vec![b"09d8d3a12e161a7f6afb522dbe8900a9c09bce06".as_bstr()]),
            author: signature(1592391367),
            committer: signature(1592391367),
            encoding: None,
            message: b"update tasks\n".as_bstr(),
            extra_headers: vec![(b"gpgsig".as_bstr(), b"magic:signature".as_bstr().into())]
        }
    );
    Ok(())
}

#[test]
fn signed_with_encoding() -> crate::Result {
    let input = fixture_name("commit", "signed-with-encoding.txt");
    let iter = CommitRefIter::from_bytes(&input);
    assert_eq!(
        iter.collect::<Result<Vec<_>, _>>()?,
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

    assert_eq!(iter.author().ok(), Some(signature(1592448995)));
    assert_eq!(iter.committer().ok(), Some(signature(1592449083)));
    Ok(())
}

#[test]
fn whitespace() -> crate::Result {
    assert_eq!(
        CommitRefIter::from_bytes(&fixture_name("commit", "whitespace.txt")).collect::<Result<Vec<_>, _>>()?,
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
        CommitRefIter::from_bytes(&fixture_name("commit", "unsigned.txt")).collect::<Result<Vec<_>, _>>()?,
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
        CommitRefIter::from_bytes(&fixture_name("commit", "signed-singleline.txt")).collect::<Result<Vec<_>, _>>()?,
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
    assert_eq!(
        CommitRefIter::from_bytes(&fixture_name("commit", "signed-singleline.txt"))
            .parent_ids()
            .collect::<Vec<_>>(),
        vec![hex_to_id("09d8d3a12e161a7f6afb522dbe8900a9c09bce06")]
    );
    Ok(())
}

#[test]
fn error_handling() -> crate::Result {
    let data = fixture_name("commit", "unsigned.txt");
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
    let input = fixture_name("commit", "mergetag.txt");
    let iter = CommitRefIter::from_bytes(&input);
    assert_eq!(
        iter.collect::<Result<Vec<_>, _>>()?,
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
            Token::Message(LONG_MESSAGE.into()),
        ]
    );
    assert_eq!(
        iter.parent_ids().collect::<Vec<_>>(),
        vec![
            hex_to_id("44ebe016df3aad96e3be8f95ec52397728dd7701"),
            hex_to_id("8d485da0ddee79d0e6713405694253d401e41b93")
        ]
    );
    assert_eq!(iter.message().ok(), Some(LONG_MESSAGE.into()));
    Ok(())
}

mod method {
    use gix_object::CommitRefIter;

    use crate::{fixture_name, hex_to_id, signature};

    #[test]
    fn tree_id() -> crate::Result {
        let input = fixture_name("commit", "unsigned.txt");
        let iter = CommitRefIter::from_bytes(&input);
        assert_eq!(
            iter.clone().tree_id().ok(),
            Some(hex_to_id("1b2dfb4ac5e42080b682fc676e9738c94ce6d54d"))
        );
        assert_eq!(
            iter.signatures().collect::<Vec<_>>(),
            vec![signature(1592437401), signature(1592437401)]
        );
        assert_eq!(iter.parent_ids().count(), 0);
        Ok(())
    }

    #[test]
    fn signatures() -> crate::Result {
        let input = fixture_name("commit", "unsigned.txt");
        let iter = CommitRefIter::from_bytes(&input);
        assert_eq!(
            iter.signatures().collect::<Vec<_>>(),
            vec![signature(1592437401), signature(1592437401)]
        );
        assert_eq!(iter.author().ok(), Some(signature(1592437401)));
        assert_eq!(iter.committer().ok(), Some(signature(1592437401)));
        assert_eq!(iter.author().ok(), Some(signature(1592437401)), "it's not consuming");
        Ok(())
    }
}
