use bstr::ByteVec;
use git_config::file::{init, Metadata};
use std::convert::TryFrom;

#[test]
fn empty_sections_roundtrip() {
    let input = r#"
        [a]
    [b]
        [c] 
        
            [d]
"#;

    let config = git_config::File::try_from(input).unwrap();
    assert_eq!(config.to_bstring(), input);
}

#[test]
fn empty_sections_with_comments_roundtrip() {
    let input = r#"; pre-a
        [a] # side a
        ; post a  
    [b] ; side b
        [c] ; side c
        ; post c
            [d] # side d
"#;

    let mut config = git_config::File::try_from(input).unwrap();
    let mut single_string = config.to_bstring();
    assert_eq!(single_string, input);
    assert_eq!(
        config.append(config.clone()).to_string(),
        {
            let clone = single_string.clone();
            single_string.push_str(&clone);
            single_string
        },
        "string-duplication is the same as data structure duplication"
    );
}

#[test]
fn complex_lossless_roundtrip() {
    let input = r#"
        [core]
            repositoryformatversion = 0
            filemode = true
            bare = false
            logallrefupdates = true
        
        [remote "origin"]
            url = git@github.com:Byron/gitoxide.git
            fetch = +refs/heads/*:refs/remotes/origin/*

        [test]  # other comment
            other-quoted = "hello" ; comment
            implicit
            implicit-equal =
            implicit-equal-trailing-ws=     
            
        ; more comments
        # another one
            
        [test "sub-section \"special\" C:\\root"] ; section comment
            bool-explicit = false
            bool-implicit
            integer-no-prefix = 10 ; a value comment
            integer-prefix = 10g
            color = brightgreen red \
            bold
            other = hello world
            other-quoted = "hello world"
            location = ~/tmp
            location-quoted = "~/quoted"
            escaped = \n\thi\b
            escaped-quoted = "\n\thi\b"
            
        [alias]
            save = "!f() { \
               git status; \
               git add "-A"; \
               git commit -m \"$1\"; \
               git push -f; \
               git log -1;  \
            }; \
            f;  \
            unset f" ; here we go
    "#;
    let config = git_config::File::try_from(input).unwrap();
    assert_eq!(config.to_bstring(), input);

    let lossy_config = git_config::File::from_bytes_owned(
        &mut input.as_bytes().into(),
        Metadata::api(),
        init::Options {
            lossy: true,
            ..Default::default()
        },
    )
    .unwrap();

    let lossy_config: git_config::File = lossy_config.to_string().parse().unwrap();
    assert_eq!(
        lossy_config, config,
        "Even lossy configuration serializes properly to be able to restore all values"
    );
}
