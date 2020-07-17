mod method {
    use crate::{borrowed::fixture_bytes, hex_to_id};
    use git_object::borrowed::Commit;
    use pretty_assertions::assert_eq;

    #[test]
    fn tree() {
        let fixture = fixture_bytes("commit", "unsigned.txt");
        let commit = Commit::from_bytes(&fixture).unwrap();
        assert_eq!(commit.tree(), hex_to_id("1b2dfb4ac5e42080b682fc676e9738c94ce6d54d"));
        assert_eq!(commit.tree, "1b2dfb4ac5e42080b682fc676e9738c94ce6d54d");
    }
}

mod from_bytes {
    use crate::borrowed::linus_signature;
    use crate::{borrowed::fixture_bytes, borrowed::signature};
    use git_object::{borrowed::Commit, bstr::ByteSlice};
    use smallvec::SmallVec;

    #[test]
    fn unsigned() {
        assert_eq!(
            Commit::from_bytes(&fixture_bytes("commit", "unsigned.txt")).unwrap(),
            Commit {
                tree: b"1b2dfb4ac5e42080b682fc676e9738c94ce6d54d".as_bstr(),
                parents: SmallVec::default(),
                author: signature(1592437401),
                committer: signature(1592437401),
                encoding: None,
                message: b"without sig".as_bstr(),
                pgp_signature: None,
                extra_headers: vec![]
            }
        );
    }

    #[test]
    fn whitespace() {
        assert_eq!(
            Commit::from_bytes(&fixture_bytes("commit", "whitespace.txt")).unwrap(),
            Commit {
                tree: b"9bed6275068a0575243ba8409253e61af81ab2ff".as_bstr(),
                parents: SmallVec::from(vec![b"26b4df046d1776c123ac69d918f5aec247b58cc6".as_bstr()]),
                author: signature(1592448450),
                committer: signature(1592448450),
                encoding: None,
                message: b" nl".as_bstr(), // this one had a \n trailing it, but git seems to trim that
                pgp_signature: None,
                extra_headers: vec![]
            }
        );
    }

    #[test]
    fn signed_singleline() {
        assert_eq!(
            Commit::from_bytes(&fixture_bytes("commit", "signed-singleline.txt")).unwrap(),
            Commit {
                tree: b"00fc39317701176e326974ce44f5bd545a32ec0b".as_bstr(),
                parents: SmallVec::from(vec![b"09d8d3a12e161a7f6afb522dbe8900a9c09bce06".as_bstr()]),
                author: signature(1592391367),
                committer: signature(1592391367),
                encoding: None,
                message: b"update tasks\n".as_bstr(),
                pgp_signature: None,
                extra_headers: vec![(b"gpgsig".as_bstr(), b"magic:signature".as_bstr().into())]
            }
        );
    }

    #[test]
    fn mergetag() {
        assert_eq!(
            Commit::from_bytes(&fixture_bytes("commit", "mergetag.txt")).unwrap(),
            Commit {
                tree: b"1c61918031bf2c7fab9e17dde3c52a6a9884fcb5".as_bstr(),
                parents: SmallVec::from(vec![
                    b"44ebe016df3aad96e3be8f95ec52397728dd7701".as_bstr(),
                    b"8d485da0ddee79d0e6713405694253d401e41b93".as_bstr()
                ]),
                author: linus_signature(1591996221),
                committer: linus_signature(1591996221),
                encoding: None,
                message: "Merge tag 'thermal-v5.8-rc1' of git://git.kernel.org/pub/scm/linux/kernel/git/thermal/linux

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
"
                .as_bytes()
                .as_bstr(),
                pgp_signature: None,
                extra_headers: vec![(
                    b"mergetag".as_bstr(),
                    std::borrow::Cow::Owned(
                        "object 8d485da0ddee79d0e6713405694253d401e41b93
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
-----END PGP SIGNATURE-----"
                            .as_bytes()
                            .into()
                    )
                )]
            }
        );
    }

    #[test]
    fn signed() {
        assert_eq!(
            Commit::from_bytes(&fixture_bytes("commit", "signed.txt")).unwrap(),
            Commit {
                tree: b"00fc39317701176e326974ce44f5bd545a32ec0b".as_bstr(),
                parents: SmallVec::from(vec![b"09d8d3a12e161a7f6afb522dbe8900a9c09bce06".as_bstr()]),
                author: signature(1592391367),
                committer: signature(1592391367),
                encoding: None,
                message: b"update tasks\n".as_bstr(),
                pgp_signature: None,
                extra_headers: vec![(b"gpgsig".as_bstr(), b"-----BEGIN PGP SIGNATURE-----\n\niQEzBAABCAAdFiEEdjYp/sh4j8NRKLX27gKdHl60AwAFAl7p9tgACgkQ7gKdHl60\nAwBpegf+KQciv9AOIN7+yPmowecGxBnSfpKWTDzFxnyGR8dq63SpWT8WEKG5mf3a\nG6iUqpsDWaMHlzihaMKRvgRpZxFRbjnNPFBj6F4RRqfE+5R7k6DRSLUV5PqnsdSH\nuccfIDWi1imhsm7AaP5trwl1t+83U2JhHqPcPVFLMODYwWeO6NLR/JCzGSTQRa8t\nRgaVMKI19O/fge5OT5Ua8D47VKEhsJX0LfmkP5RfZQ8JJvNd40TupqKRdlv0sAzP\nya7NXkSHXCavHNR6kA+KpWxn900UoGK8/IDlwU6MeOkpPVawb3NFMqnc7KJDaC2p\nSMzpuEG8LTrCx2YSpHNLqHyzvQ1CZA==\n=5ITV\n-----END PGP SIGNATURE-----".as_bstr().into())]
            }
        );
    }

    #[test]
    fn signed_with_encoding() {
        assert_eq!(
            Commit::from_bytes(&fixture_bytes("commit", "signed-with-encoding.txt")).unwrap(),
            Commit {
                tree: b"1973afa74d87b2bb73fa884aaaa8752aec43ea88".as_bstr(),
                parents: SmallVec::from(vec![b"79c51cc86923e2b8ca0ee5c4eb75e48027133f9a".as_bstr()]),
                author: signature(1592448995),
                committer: signature(1592449083),
                encoding: Some(b"ISO-8859-1".as_bstr()),
                message: b"encoding & sig".as_bstr(),
                pgp_signature: None,
                extra_headers: vec![(b"gpgsig".as_bstr(), b"-----BEGIN PGP SIGNATURE-----\n\niQEzBAABCAAdFiEEdjYp/sh4j8NRKLX27gKdHl60AwAFAl7q2DsACgkQ7gKdHl60\nAwDvewgAkL5UjEztzeVXlzceom0uCrAkCw9wSGLTmYcMKW3JwEaTRgQ4FX+sDuFT\nLZ8DoPu3UHUP0QnKrUwHulTTlKcOAvsczHbVPIKtXCxo6QpUfhsJQwz/J29kiE4L\nsOd+lqKGnn4oati/de2xwqNGi081fO5KILX75z6KfsAe7Qz7R3jxRF4uzHI033O+\nJc2Y827XeaELxW40SmzoLanWgEcdreXf3PstXEWW77CAu0ozXmvYj56vTviVybxx\nG7bc8lwc+SSKVe2VVB+CCfVbs0i541gmghUpZfMhUgaqttcCH8ysrUJDhne1BLG8\nCrOJIWTwAeEDtomV1p76qrMeqr1GFg==\n=qlSN\n-----END PGP SIGNATURE-----".as_bstr().into())]
            }
        );
    }

    #[test]
    fn with_encoding() {
        assert_eq!(
            Commit::from_bytes(&fixture_bytes("commit", "with-encoding.txt")).unwrap(),
            Commit {
                tree: b"4a1c03029e7407c0afe9fc0320b3258e188b115e".as_bstr(),
                parents: SmallVec::from(vec![b"7ca98aad461a5c302cb4c9e3acaaa6053cc67a62".as_bstr()]),
                author: signature(1592438199),
                committer: signature(1592438199),
                encoding: Some("ISO-8859-1".into()),
                message: b"commit with encoding".as_bstr(),
                pgp_signature: None,
                extra_headers: vec![]
            }
        );
    }

    #[test]
    fn merge() {
        assert_eq!(
            Commit::from_bytes(&fixture_bytes("commit", "merge.txt")).unwrap(),
            Commit {
                tree: b"0cf16ce8e229b59a761198975f0c0263229faf82".as_bstr(),
                parents: SmallVec::from(vec![
                    b"6a6054db4ce3c1e4e6a37f8c4d7acb63a4d6ad71".as_bstr(),
                    b"c91d592913d47ac4e4a76daf16fd649b276e211e".as_bstr()
                ]),
                author: signature(1592454703),
                committer: signature(1592454738),
                encoding: Some("ISO-8859-1".into()),
                message: b"Merge branch 'branch'".as_bstr(),
                pgp_signature: None,
                extra_headers: vec![]
            }
        );
    }
}
