const SIGNATURE: & [u8; 487] = b"-----BEGIN PGP SIGNATURE-----\n\niQEzBAABCAAdFiEEdjYp/sh4j8NRKLX27gKdHl60AwAFAl7q2DsACgkQ7gKdHl60\nAwDvewgAkL5UjEztzeVXlzceom0uCrAkCw9wSGLTmYcMKW3JwEaTRgQ4FX+sDuFT\nLZ8DoPu3UHUP0QnKrUwHulTTlKcOAvsczHbVPIKtXCxo6QpUfhsJQwz/J29kiE4L\nsOd+lqKGnn4oati/de2xwqNGi081fO5KILX75z6KfsAe7Qz7R3jxRF4uzHI033O+\nJc2Y827XeaELxW40SmzoLanWgEcdreXf3PstXEWW77CAu0ozXmvYj56vTviVybxx\nG7bc8lwc+SSKVe2VVB+CCfVbs0i541gmghUpZfMhUgaqttcCH8ysrUJDhne1BLG8\nCrOJIWTwAeEDtomV1p76qrMeqr1GFg==\n=qlSN\n-----END PGP SIGNATURE-----";

const LONG_MESSAGE: &str = "Merge tag 'thermal-v5.8-rc1' of git://git.kernel.org/pub/scm/linux/kernel/git/thermal/linux

Pull thermal updates from Daniel Lezcano:

 - Add the hwmon support on the i.MX SC (Anson Huang)

 - Thermal framework cleanups (self-encapsulation, pointless stubs,
   private structures) (Daniel Lezcano)

 - Use the PM QoS frequency changes for the devfreq cooling device
   (Matthias Kaehlcke)

 - Remove duplicate error messages from platform_get_irq() error
   handling (Markus Elfring)

 - Add support for the bandgap sensors (Keerthy)

 - Statically initialize .get_mode/.set_mode ops (Andrzej Pietrasiewicz)

 - Add Renesas R-Car maintainer entry (Niklas Söderlund)

 - Fix error checking after calling ti_bandgap_get_sensor_data() for the
   TI SoC thermal (Sudip Mukherjee)

 - Add latency constraint for the idle injection, the DT binding and the
   change the registering function (Daniel Lezcano)

 - Convert the thermal framework binding to the Yaml schema (Amit
   Kucheria)

 - Replace zero-length array with flexible-array on i.MX 8MM (Gustavo A.
   R. Silva)

 - Thermal framework cleanups (alphabetic order for heads, replace
   module.h by export.h, make file naming consistent) (Amit Kucheria)

 - Merge tsens-common into the tsens driver (Amit Kucheria)

 - Fix platform dependency for the Qoriq driver (Geert Uytterhoeven)

 - Clean up the rcar_thermal_update_temp() function in the rcar thermal
   driver (Niklas Söderlund)

 - Fix the TMSAR register for the TMUv2 on the Qoriq platform (Yuantian
   Tang)

 - Export GDDV, OEM vendor variables, and don't require IDSP for the
   int340x thermal driver - trivial conflicts fixed (Matthew Garrett)

* tag 'thermal-v5.8-rc1' of git://git.kernel.org/pub/scm/linux/kernel/git/thermal/linux: (48 commits)
  thermal/int340x_thermal: Don't require IDSP to exist
  thermal/int340x_thermal: Export OEM vendor variables
  thermal/int340x_thermal: Export GDDV
  thermal: qoriq: Update the settings for TMUv2
  thermal: rcar_thermal: Clean up rcar_thermal_update_temp()
  thermal: qoriq: Add platform dependencies
  drivers: thermal: tsens: Merge tsens-common.c into tsens.c
  thermal/of: Rename of-thermal.c
  thermal/governors: Prefix all source files with gov_
  thermal/drivers/user_space: Sort headers alphabetically
  thermal/drivers/of-thermal: Sort headers alphabetically
  thermal/drivers/cpufreq_cooling: Replace module.h with export.h
  thermal/drivers/cpufreq_cooling: Sort headers alphabetically
  thermal/drivers/clock_cooling: Include export.h
  thermal/drivers/clock_cooling: Sort headers alphabetically
  thermal/drivers/thermal_hwmon: Include export.h
  thermal/drivers/thermal_hwmon: Sort headers alphabetically
  thermal/drivers/thermal_helpers: Include export.h
  thermal/drivers/thermal_helpers: Sort headers alphabetically
  thermal/core: Replace module.h with export.h
  ...
";

const MERGE_TAG: &str = "object 8d485da0ddee79d0e6713405694253d401e41b93
type commit
tag thermal-v5.8-rc1
tagger Daniel Lezcano <daniel.lezcano@linaro.org> 1591979433 +0200

- Add the hwmon support on the i.MX SC (Anson Huang)

- Thermal framework cleanups (self-encapsulation, pointless stubs,
  private structures) (Daniel Lezcano)

- Use the PM QoS frequency changes for the devfreq cooling device (Matthias
  Kaehlcke)

- Remove duplicate error messages from platform_get_irq() error handling
  (Markus Elfring)

- Add support for the bandgap sensors (Keerthy)

- Statically initialize .get_mode/.set_mode ops (Andrzej Pietrasiewicz)

- Add Renesas R-Car maintainer entry (Niklas Söderlund)

- Fix error checking after calling ti_bandgap_get_sensor_data() for the TI SoC
  thermal (Sudip Mukherjee)

- Add latency constraint for the idle injection, the DT binding and the change
  the registering function (Daniel Lezcano)

- Convert the thermal framework binding to the Yaml schema (Amit Kucheria)

- Replace zero-length array with flexible-array on i.MX 8MM (Gustavo A. R. Silva)

- Thermal framework cleanups (alphabetic order for heads, replace module.h by
  export.h, make file naming consistent) (Amit Kucheria)

- Merge tsens-common into the tsens driver (Amit Kucheria)

- Fix platform dependency for the Qoriq driver (Geert Uytterhoeven)

- Clean up the rcar_thermal_update_temp() function in the rcar thermal driver
  (Niklas Söderlund)

- Fix the TMSAR register for the TMUv2 on the Qoriq platform (Yuantian Tang)

- Export GDDV, OEM vendor variables, and don't require IDSP for the int340x
  thermal driver - trivial conflicts fixed (Matthew Garrett)
-----BEGIN PGP SIGNATURE-----

iQEzBAABCAAdFiEEGn3N4YVz0WNVyHskqDIjiipP6E8FAl7jra8ACgkQqDIjiipP
6E+ugAgApBF6FsHoonWIvoSrzBrrbU2oqhEJA42Mx+iY/UnXi01I79vZ/8WpZt7M
D1J01Kf0PUhRbywoKaoCX3Oh9ZO9PKq4N9ZC8yqdoD6GLl+rC9Wmr7Ui+c80klcv
M9rYhpPYfNXTFj0saSbbFWNNhP4TvhzGsNj8foYVQDKyhjbSmNE5ipZlbmP23jlr
O53SmJAwS5zxLOd8QA5nfSWP9FYYMuCR2AHj8BUCmxiAjXZLPNB/Hz2RRBr7q0MF
zRo/4HJ04mSQYp0kluP/EBhz9g2wM/htIPyWRveB/ByKEYt3UNKjB++PJmPbu5UG
dS3aXZhRfaPqpdsWrMB9fY7ll+oyfw==
=T+RI
-----END PGP SIGNATURE-----";
mod method {
    use git_object::CommitRef;
    use pretty_assertions::assert_eq;

    use crate::{hex_to_id, immutable::fixture_bytes};

    #[test]
    fn tree() -> crate::Result {
        let fixture = fixture_bytes("commit", "unsigned.txt");
        let commit = CommitRef::from_bytes(&fixture)?;
        assert_eq!(commit.tree(), hex_to_id("1b2dfb4ac5e42080b682fc676e9738c94ce6d54d"));
        assert_eq!(commit.tree, "1b2dfb4ac5e42080b682fc676e9738c94ce6d54d");
        Ok(())
    }
}

mod iter {
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
            CommitRefIter::from_bytes(&fixture_bytes("commit", "signed-singleline.txt"))
                .collect::<Result<Vec<_>, _>>()?,
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
}

mod from_bytes {
    use git_object::{bstr::ByteSlice, CommitRef};
    use smallvec::SmallVec;

    use crate::immutable::{
        commit::{LONG_MESSAGE, MERGE_TAG, SIGNATURE},
        fixture_bytes, linus_signature, signature,
    };

    #[test]
    fn unsigned() -> crate::Result {
        assert_eq!(
            CommitRef::from_bytes(&fixture_bytes("commit", "unsigned.txt"))?,
            CommitRef {
                tree: b"1b2dfb4ac5e42080b682fc676e9738c94ce6d54d".as_bstr(),
                parents: SmallVec::default(),
                author: signature(1592437401),
                committer: signature(1592437401),
                encoding: None,
                message: b"without sig".as_bstr(),
                extra_headers: vec![]
            }
        );
        Ok(())
    }

    #[test]
    fn whitespace() -> crate::Result {
        assert_eq!(
            CommitRef::from_bytes(&fixture_bytes("commit", "whitespace.txt"))?,
            CommitRef {
                tree: b"9bed6275068a0575243ba8409253e61af81ab2ff".as_bstr(),
                parents: SmallVec::from(vec![b"26b4df046d1776c123ac69d918f5aec247b58cc6".as_bstr()]),
                author: signature(1592448450),
                committer: signature(1592448450),
                encoding: None,
                message: b" nl".as_bstr(), // this one had a \n trailing it, but git seems to trim that
                extra_headers: vec![]
            }
        );
        Ok(())
    }

    #[test]
    fn signed_singleline() -> crate::Result {
        assert_eq!(
            CommitRef::from_bytes(&fixture_bytes("commit", "signed-singleline.txt"))?,
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
    fn mergetag() -> crate::Result {
        let fixture = fixture_bytes("commit", "mergetag.txt");
        let commit = CommitRef {
            tree: b"1c61918031bf2c7fab9e17dde3c52a6a9884fcb5".as_bstr(),
            parents: SmallVec::from(vec![
                b"44ebe016df3aad96e3be8f95ec52397728dd7701".as_bstr(),
                b"8d485da0ddee79d0e6713405694253d401e41b93".as_bstr(),
            ]),
            author: linus_signature(1591996221),
            committer: linus_signature(1591996221),
            encoding: None,
            message: LONG_MESSAGE.as_bytes().as_bstr(),
            extra_headers: vec![(
                b"mergetag".as_bstr(),
                std::borrow::Cow::Owned(MERGE_TAG.as_bytes().into()),
            )],
        };
        assert_eq!(CommitRef::from_bytes(&fixture)?, commit);
        assert_eq!(commit.extra_headers().find_all("mergetag").count(), 1);
        assert_eq!(commit.extra_headers().mergetags().count(), 1);
        Ok(())
    }

    #[test]
    fn signed() -> crate::Result {
        assert_eq!(
            CommitRef::from_bytes(&fixture_bytes("commit", "signed.txt"))?,
            CommitRef {
                tree: b"00fc39317701176e326974ce44f5bd545a32ec0b".as_bstr(),
                parents: SmallVec::from(vec![b"09d8d3a12e161a7f6afb522dbe8900a9c09bce06".as_bstr()]),
                author: signature(1592391367),
                committer: signature(1592391367),
                encoding: None,
                message: b"update tasks\n".as_bstr(),
                extra_headers: vec![(b"gpgsig".as_bstr(), b"-----BEGIN PGP SIGNATURE-----\n\niQEzBAABCAAdFiEEdjYp/sh4j8NRKLX27gKdHl60AwAFAl7p9tgACgkQ7gKdHl60\nAwBpegf+KQciv9AOIN7+yPmowecGxBnSfpKWTDzFxnyGR8dq63SpWT8WEKG5mf3a\nG6iUqpsDWaMHlzihaMKRvgRpZxFRbjnNPFBj6F4RRqfE+5R7k6DRSLUV5PqnsdSH\nuccfIDWi1imhsm7AaP5trwl1t+83U2JhHqPcPVFLMODYwWeO6NLR/JCzGSTQRa8t\nRgaVMKI19O/fge5OT5Ua8D47VKEhsJX0LfmkP5RfZQ8JJvNd40TupqKRdlv0sAzP\nya7NXkSHXCavHNR6kA+KpWxn900UoGK8/IDlwU6MeOkpPVawb3NFMqnc7KJDaC2p\nSMzpuEG8LTrCx2YSpHNLqHyzvQ1CZA==\n=5ITV\n-----END PGP SIGNATURE-----".as_bstr().into())]
            }
        );
        Ok(())
    }

    #[test]
    fn signed_with_encoding() -> crate::Result {
        assert_eq!(
            CommitRef::from_bytes(&fixture_bytes("commit", "signed-with-encoding.txt"))?,
            CommitRef {
                tree: b"1973afa74d87b2bb73fa884aaaa8752aec43ea88".as_bstr(),
                parents: SmallVec::from(vec![b"79c51cc86923e2b8ca0ee5c4eb75e48027133f9a".as_bstr()]),
                author: signature(1592448995),
                committer: signature(1592449083),
                encoding: Some(b"ISO-8859-1".as_bstr()),
                message: b"encoding & sig".as_bstr(),
                extra_headers: vec![(b"gpgsig".as_bstr(), SIGNATURE.as_bstr().into())]
            }
        );
        Ok(())
    }

    #[test]
    fn with_encoding() -> crate::Result {
        assert_eq!(
            CommitRef::from_bytes(&fixture_bytes("commit", "with-encoding.txt"))?,
            CommitRef {
                tree: b"4a1c03029e7407c0afe9fc0320b3258e188b115e".as_bstr(),
                parents: SmallVec::from(vec![b"7ca98aad461a5c302cb4c9e3acaaa6053cc67a62".as_bstr()]),
                author: signature(1592438199),
                committer: signature(1592438199),
                encoding: Some("ISO-8859-1".into()),
                message: b"commit with encoding".as_bstr(),
                extra_headers: vec![]
            }
        );
        Ok(())
    }

    #[test]
    fn merge() -> crate::Result {
        assert_eq!(
            CommitRef::from_bytes(&fixture_bytes("commit", "merge.txt"))?,
            CommitRef {
                tree: b"0cf16ce8e229b59a761198975f0c0263229faf82".as_bstr(),
                parents: SmallVec::from(vec![
                    b"6a6054db4ce3c1e4e6a37f8c4d7acb63a4d6ad71".as_bstr(),
                    b"c91d592913d47ac4e4a76daf16fd649b276e211e".as_bstr()
                ]),
                author: signature(1592454703),
                committer: signature(1592454738),
                encoding: Some("ISO-8859-1".into()),
                message: b"Merge branch 'branch'".as_bstr(),
                extra_headers: vec![]
            }
        );
        Ok(())
    }

    const OTHER_SIGNATURE: &[u8; 455] = b"-----BEGIN PGP SIGNATURE-----

wsBcBAABCAAQBQJeqxW4CRBK7hj4Ov3rIwAAdHIIAFD98qgN/k8ybukCLf6kpzvi
5V8gf6BflONXc/oIDySurW7kfS9/r6jOgu08UN8KlQx4Q4g8yY7PROABhwGI70B3
+mHPFcParQf5FBDDZ3GNNpJdlaI9eqzEnFk8AmHmyKHfuGLoclXUObXQ3oe3fmT7
QdTC7JTyk/bPnZ9HQKw7depa3+7Kw4wv4DG8QcW3BG6B9bcE15qaWmOiq0ryRXsv
k7D0LqGSXjU5wrQrKnemC7nWhmQsqaXDe89XXmliClCAx4/bepPiXK0eT/DNIKUr
iyBBl69jASy41Ug/BlFJbw4+ItkShpXwkJKuBBV/JExChmvbxYWaS7QnyYC9UO0=
=HLmy
-----END PGP SIGNATURE-----
";

    #[test]
    fn newline_right_after_signature_multiline_header() -> crate::Result {
        let fixture = fixture_bytes("commit", "signed-whitespace.txt");
        let commit = CommitRef::from_bytes(&fixture)?;
        let pgp_sig = OTHER_SIGNATURE.as_bstr();
        assert_eq!(commit.extra_headers[0].1.as_ref(), pgp_sig);
        assert_eq!(commit.extra_headers().pgp_signature(), Some(pgp_sig));
        assert_eq!(commit.extra_headers().find("gpgsig"), Some(pgp_sig));
        assert!(commit.message.starts_with(b"Rollup"));
        Ok(())
    }
}
