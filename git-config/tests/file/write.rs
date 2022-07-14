use std::convert::TryFrom;

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
            
        [test "sub-section \"special\" C:\\root"]
            bool-explicit = false
            bool-implicit
            integer-no-prefix = 10
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
}
