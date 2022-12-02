# Must be sourced into the main journey test
set -eu

title plumbing "${kind}"
snapshot="$snapshot/plumbing"
title "git-tempfile crate"
(when "testing 'git-tempfile'"
  snapshot="$snapshot/git-tempfile"
  cd git-tempfile
  ABORTED=143

  (when "running the example program to raise a signal with a tempfile present"
    it "fails as the process aborts" && {
      expect_run $ABORTED cargo run --example delete-tempfiles-on-sigterm
    }
    TEMPFILE="$(cargo run --example delete-tempfiles-on-sigterm 2>/dev/null || true)"
    it "outputs a tempfile with an expected name" && {
      expect_run $SUCCESSFULLY test "$TEMPFILE" = "tempfile.ext"
    }
    it "cleans up the tempfile '$TEMPFILE' it created" && {
      expect_run $WITH_FAILURE test -e "$TEMPFILE"
    }
  )

  (when "running the example program to help assure there cannot be deadlocks"
    ABORTED=134
    it "succeeds as it won't deadlock" && {
      expect_run $ABORTED cargo run --release --example try-deadlock-on-cleanup -- 5
    }
  )
)

title "git-tempfile crate"
(when "testing 'git-repository'"
  snapshot="$snapshot/git-repository"
  cd git-repository
  ABORTED=143

  (when "running the example program to check order of signal handlers"
    it "fails as the process aborts" && {
      expect_run $ABORTED cargo run --no-default-features --example interrupt-handler-allows-graceful-shutdown
    }
    it "cleans up the tempfile it created" && {
      expect_run $WITH_FAILURE test -e "example-file.tmp"
    }
  )
)

title "gix (with repository)"
(with "a git repository"
  snapshot="$snapshot/repository"
  (small-repo-in-sandbox
    (with "the 'verify' sub-command"
      snapshot="$snapshot/verify"
      (with 'human output format'
        it "generates correct output" && {
          WITH_SNAPSHOT="$snapshot/success-format-human" \
          expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose --format human verify -s
        }
      )
      if test "$kind" = "max" || test "$kind" = "max-pure"; then
      (with "--format json"
        it "generates the correct output in JSON format" && {
          WITH_SNAPSHOT="$snapshot/success-format-json" \
          expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose --format json verify --statistics
        }
      )
      fi
    )
  )

  title "gix remote"
  (when "running 'remote'"
    snapshot="$snapshot/remote"
    title "gix remote refs"
    (with "the 'refs' subcommand"
      snapshot="$snapshot/refs"
      (small-repo-in-sandbox
        if [[ "$kind" != "small" ]]; then

        if [[ "$kind" != "async" ]]; then
        (with "file:// protocol"
          (with "version 1"
            it "generates the correct output" && {
              WITH_SNAPSHOT="$snapshot/file-v-any" \
              expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose -c protocol.version=1 remote -n .git refs
            }
          )
          (with "version 2"
            it "generates the correct output" && {
              WITH_SNAPSHOT="$snapshot/file-v-any" \
              expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose -c protocol.version=2  remote -n "$PWD" refs
            }
          )
          if test "$kind" = "max" || test "$kind" = "max-pure"; then
          (with "--format json"
            it "generates the correct output in JSON format" && {
              WITH_SNAPSHOT="$snapshot/file-v-any-json" \
              expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose --format json remote -n . refs
            }
          )
          fi
        )
        fi

        (with "git:// protocol"
          launch-git-daemon
          (with "version 1"
            it "generates the correct output" && {
              WITH_SNAPSHOT="$snapshot/file-v-any" \
              expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose --config protocol.version=1 remote --name git://localhost/ refs
            }
          )
          (with "version 2"
            it "generates the correct output" && {
              WITH_SNAPSHOT="$snapshot/file-v-any" \
              expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose -c protocol.version=2 remote -n git://localhost/ refs
            }
          )
        )
        if [[ "$kind" == "small" ]]; then
        (with "https:// protocol (in small builds)"
          it "fails as http is not compiled in" && {
            WITH_SNAPSHOT="$snapshot/fail-http-in-small" \
            expect_run $WITH_FAILURE "$exe_plumbing" --no-verbose -c protocol.version=1 remote -n https://github.com/byron/gitoxide refs
          }
        )
        fi
        (on_ci
          if test "$kind" = "max" || test "$kind" = "max-pure"; then
          (with "https:// protocol"
            (with "version 1"
              it "generates the correct output" && {
                expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose -c protocol.version=1 remote -n https://github.com/byron/gitoxide refs
              }
            )
            (with "version 2"
              it "generates the correct output" && {
                expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose -c protocol.version=2 remote -n https://github.com/byron/gitoxide refs
              }
            )
          )
          fi
        )
        else
          it "fails as the CLI doesn't include networking in 'small' mode" && {
            WITH_SNAPSHOT="$snapshot/remote ref-list-no-networking-in-small-failure" \
            expect_run 2 "$exe_plumbing" --no-verbose -c protocol.version=1 remote -n .git refs
          }
        fi
      )
    )
  )
)

(with "gix free"
  snapshot="$snapshot/no-repo"
  title "gix free pack"
  (when "running 'pack'"
    snapshot="$snapshot/pack"

    title "gix free pack receive"
    (with "the 'receive' sub-command"
      snapshot="$snapshot/receive"
      (small-repo-in-sandbox
        if [[ "$kind" != 'small' ]]; then

        if [[ "$kind" != 'async' ]]; then
        (with "file:// protocol"
          (with "version 1"
            (with "NO output directory"
              it "generates the correct output" && {
                WITH_SNAPSHOT="$snapshot/file-v-any-no-output" \
                expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose free pack receive -p 1 .git
              }
            )
            (with "output directory"
              mkdir out
              it "generates the correct output" && {
                WITH_SNAPSHOT="$snapshot/file-v-any-with-output" \
                expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose free pack receive -p 1 .git out/
              }
              it "creates an index and a pack in the output directory" && {
                WITH_SNAPSHOT="$snapshot/ls-in-output-dir" \
                expect_run $SUCCESSFULLY ls out/
              }
              (with "--write-refs set"
                it "generates the correct output" && {
                  WITH_SNAPSHOT="$snapshot/file-v-any-with-output" \
                  expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose free pack receive -p 1 --refs-directory out/all-refs .git out/
                }
                it "writes references into the refs folder of the output directory" && {
                  expect_snapshot "$snapshot/repo-refs" out/all-refs
                }
              )
              rm -Rf out
            )
            if test "$kind" = "max" || test "$kind" = "max-pure"; then
            (with "--format json"
              it "generates the correct output in JSON format" && {
                WITH_SNAPSHOT="$snapshot/file-v-any-no-output-json" \
                expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose --format json free pack receive --protocol 1 .git
              }
            )
            fi
          )
          (with "version 2"
            (with "NO output directory"
              it "generates the correct output" && {
                WITH_SNAPSHOT="$snapshot/file-v-any-no-output" \
                expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose free pack receive -p 2 .git
              }
            )
            (with "output directory"
              mkdir out/
              it "generates the correct output" && {
                WITH_SNAPSHOT="$snapshot/file-v-any-with-output" \
                expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose free pack receive .git out/
              }
              it "creates an index and a pack in the output directory" && {
                WITH_SNAPSHOT="$snapshot/ls-in-output-dir" \
                expect_run $SUCCESSFULLY ls out/
              }
              rm -Rf out
            )
            if test "$kind" = "max" || test "$kind" = "max-pure"; then
            (with "--format json"
              it "generates the correct output in JSON format" && {
                WITH_SNAPSHOT="$snapshot/file-v-any-no-output-json" \
                expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose --format json free pack receive --protocol 2 .git
              }
            )
            fi
          )
        )
        fi
        (with "git:// protocol"
          launch-git-daemon
          (with "version 1"
            (with "NO output directory"
              (with "no wanted refs"
                it "generates the correct output" && {
                  WITH_SNAPSHOT="$snapshot/file-v-any-no-output" \
                  expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose free pack receive -p 1 git://localhost/
                }
              )
              (with "wanted refs"
                it "generates the correct output" && {
                  WITH_SNAPSHOT="$snapshot/file-v-any-no-output-wanted-ref-p1" \
                  expect_run $WITH_FAILURE "$exe_plumbing" --no-verbose free pack receive -p 1 git://localhost/ -r =refs/heads/main
                }
              )
            )
            (with "output directory"
              mkdir out
              it "generates the correct output" && {
                WITH_SNAPSHOT="$snapshot/file-v-any-with-output" \
                expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose free pack receive -p 1 git://localhost/ out/
              }
            )
          )
          (with "version 2"
            (with "NO output directory"
              (with "NO wanted refs"
                it "generates the correct output" && {
                  WITH_SNAPSHOT="$snapshot/file-v-any-no-output" \
                  expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose free pack receive -p 2 git://localhost/
                }
              )
              (with "wanted refs"
                it "generates the correct output" && {
                  WITH_SNAPSHOT="$snapshot/file-v-any-no-output-single-ref" \
                  expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose free pack receive -p 2 git://localhost/ -r refs/heads/main
                }
                (when "ref does not exist"
                  it "fails with a detailed error message including what the server said" && {
                    WITH_SNAPSHOT="$snapshot/file-v-any-no-output-non-existing-single-ref" \
                    expect_run $WITH_FAILURE "$exe_plumbing" --no-verbose free pack receive -p 2 git://localhost/ -r refs/heads/does-not-exist
                  }
                )
              )
            )
            (with "output directory"
              it "generates the correct output" && {
                WITH_SNAPSHOT="$snapshot/file-v-any-with-output" \
                expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose free pack receive git://localhost/ out/
              }
            )
          )
        )
        (on_ci
          if test "$kind" = "max" || test "$kind" = "max-pure"; then
          (with "https:// protocol"
            (with "version 1"
              it "works" && {
                expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose free pack receive -p 1 https://github.com/byron/gitoxide
              }
            )
            (with "version 2"
              it "works" && {
                expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose free pack receive -p 2 https://github.com/byron/gitoxide
              }
            )
          )
          fi
        )
        elif [[ "$kind" = "small" ]]; then
          it "fails as the CLI doesn't have networking in 'small' mode" && {
            WITH_SNAPSHOT="$snapshot/pack receive-no-networking-in-small-failure" \
            expect_run 2 "$exe_plumbing" --no-verbose free pack receive -p 1 .git
          }
        fi
      )
    )
    (with "the 'index' sub-command"
      snapshot="$snapshot/index"
      title "gix free pack index create"
      (with "the 'create' sub-command"
        snapshot="$snapshot/create"
        PACK_FILE="$fixtures/packs/pack-11fdfa9e156ab73caae3b6da867192221f2089c2.pack"
        (with "a valid and complete pack file"
          (with "NO output directory specified"
            (with "pack file passed as file"
              it "generates an index into a sink and outputs pack and index information" && {
                WITH_SNAPSHOT="$snapshot/no-output-dir-success" \
                expect_run $SUCCESSFULLY "$exe_plumbing" free pack index create -p "$PACK_FILE"
              }
            )
            (with "pack file passed from stdin"
              it "generates an index into a sink and outputs pack and index information" && {
                WITH_SNAPSHOT="$snapshot/no-output-dir-success" \
                expect_run $SUCCESSFULLY "$exe_plumbing" free pack index create < "$PACK_FILE"
              }
              if test "$kind" = "max" || test "$kind" = "max-pure"; then
              (with "--format json"
                it "generates the index into a sink and outputs information as JSON" && {
                  WITH_SNAPSHOT="$snapshot/no-output-dir-as-json-success" \
                  expect_run $SUCCESSFULLY "$exe_plumbing" --format json free pack index create < "$PACK_FILE"
                }
              )
              fi
            )
          )
          (sandbox
            (with "with an output directory specified"
              it "generates an index and outputs information" && {
                WITH_SNAPSHOT="$snapshot/output-dir-success" \
                expect_run $SUCCESSFULLY "$exe_plumbing" free pack index create -p "$PACK_FILE" "$PWD"
              }
              it "writes the index and pack into the directory (they have the same names, different suffixes)" && {
                WITH_SNAPSHOT="$snapshot/output-dir-content" \
                expect_run $SUCCESSFULLY ls
              }
            )
          )
        )
        (with "'restore' iteration mode"
          (sandbox
            cp "${PACK_FILE}" .
            PACK_FILE="${PACK_FILE##*/}"
            "$jtt" mess-in-the-middle "${PACK_FILE}"

            it "generates an index and outputs information (instead of failing)" && {
              WITH_SNAPSHOT="$snapshot/output-dir-restore-success" \
              expect_run $SUCCESSFULLY "$exe_plumbing" free pack index create -i restore -p "$PACK_FILE" "$PWD"
            }

            if test "$kind" = "max" || test "$kind" = "max-pure"; then
            (with "--format json and the very same output directory"
              it "generates the index, overwriting existing files, and outputs information as JSON" && {
                WITH_SNAPSHOT="$snapshot/output-dir-restore-as-json-success" \
                SNAPSHOT_FILTER=remove-paths \
                expect_run $SUCCESSFULLY "$exe_plumbing" --format json free pack index create -i restore $PWD < "$PACK_FILE"
              }
            )
            fi
          )
        )
      )
    )

    title "gix free pack multi-index"
    (with "the 'multi-index' sub-command"
        snapshot="$snapshot/multi-index"
        title "gix free pack multi-index create"
        (with "the 'create' sub-command"
            snapshot="$snapshot/create"
            (with 'multiple pack indices'
              (sandbox
                it "creates a multi-index successfully" && {
                  expect_run $SUCCESSFULLY "$exe_plumbing" free pack multi-index -i multi-pack-index create $fixtures/packs/pack-*.idx
                }
              )
            )
        )
    )

    title "gix free pack explode"
    (with "the 'explode' sub-command"
      snapshot="$snapshot/explode"
      PACK_FILE="$fixtures/packs/pack-11fdfa9e156ab73caae3b6da867192221f2089c2"
      (with "no objects directory specified"
        it "explodes the pack successfully and with desired output" && {
          WITH_SNAPSHOT="$snapshot/to-sink-success" \
          expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose no-repo pack explode "${PACK_FILE}.idx"
        }

        (when "using the --delete-pack flag"
          (sandbox
            (with "a valid pack"
              cp "${PACK_FILE}".idx "${PACK_FILE}".pack .
              PACK_FILE="${PACK_FILE##*/}"
              it "explodes the pack successfully and deletes the original pack and index" && {
                WITH_SNAPSHOT="$snapshot/to-sink-delete-pack-success" \
                expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose free pack explode --check skip-file-checksum --delete-pack "${PACK_FILE}.pack"
              }
              it "removes the original files" && {
                expect_run $WITH_FAILURE test -e "${PACK_FILE}".pack
                expect_run $WITH_FAILURE test -e "${PACK_FILE}".idx
              }
            )
            (with "a pack file that is invalid somewhere"
              cp ${PACK_FILE}.idx ${PACK_FILE}.pack .
              PACK_FILE="${PACK_FILE##*/}"
              "$jtt" mess-in-the-middle "${PACK_FILE}".pack

              (with "and all safety checks"
                it "does not explode the file at all" && {
                  WITH_SNAPSHOT="$snapshot/broken-delete-pack-to-sink-failure" \
                  expect_run $WITH_FAILURE "$exe_plumbing" --no-verbose free pack explode --sink-compress --check all --delete-pack "${PACK_FILE}.pack"
                }

                it "did not touch index or pack file" && {
                  expect_exists "${PACK_FILE}".pack
                  expect_exists "${PACK_FILE}".idx
                }
              )

              (with "and no safety checks at all (and an output directory)"
                it "does explode the file" && {
                  WITH_SNAPSHOT="$snapshot/broken-delete-pack-to-sink-skip-checks-success" \
                  expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose free pack explode --verify --check skip-file-and-object-checksum-and-no-abort-on-decode \
                                            --delete-pack "${PACK_FILE}.pack" .
                }

                it "removes the original files" && {
                  expect_run $WITH_FAILURE test -e "${PACK_FILE}".pack
                  expect_run $WITH_FAILURE test -e "${PACK_FILE}".idx
                }

                (with_program tree

                  if test "$kind" = "small" ; then
                    suffix=miniz-oxide
                  elif test "$kind" = "max-pure"; then
                    suffix=miniz-oxide-max
                  else
                    suffix=zlib-ng
                  fi
                  it "creates all pack objects, but the broken ones" && {
                    WITH_SNAPSHOT="$snapshot/broken-with-objects-dir-skip-checks-success-tree-$suffix" \
                    expect_run $SUCCESSFULLY tree
                  }
                )
              )
            )
          )
        )
      )
      (with "a non-existing directory specified"
        it "fails with a helpful error message" && {
          WITH_SNAPSHOT="$snapshot/missing-objects-dir-fail" \
          expect_run $WITH_FAILURE "$exe_plumbing" --no-verbose free pack explode -c skip-file-and-object-checksum "${PACK_FILE}.idx" does-not-exist
        }
      )
      (with "an existing directory specified"
        (sandbox
          it "succeeds" && {
            WITH_SNAPSHOT="$snapshot/with-objects-dir-success" \
            expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose free pack explode -c skip-file-and-object-checksum-and-no-abort-on-decode \
                                                     "${PACK_FILE}.pack" .
          }

          (with_program tree
            it "creates all pack objects" && {
              WITH_SNAPSHOT="$snapshot/with-objects-dir-success-tree" \
              expect_run $SUCCESSFULLY tree
            }
          )
        )
      )
    )

    title "gix free pack verify"
    (with "the 'verify' sub-command"
      snapshot="$snapshot/verify"
      (with "a valid pack file"
        PACK_FILE="$fixtures/packs/pack-11fdfa9e156ab73caae3b6da867192221f2089c2.pack"
        it "verifies the pack successfully and with desired output" && {
          WITH_SNAPSHOT="$snapshot/success" \
          expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose free pack verify "$PACK_FILE"
        }
      )
      (with "a valid pack INDEX file"
        MULTI_PACK_INDEX="$fixtures/packs/pack-11fdfa9e156ab73caae3b6da867192221f2089c2.idx"
        (with "no statistics"
          it "verifies the pack index successfully and with desired output" && {
            WITH_SNAPSHOT="$snapshot/index-success" \
            expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose free pack verify "$MULTI_PACK_INDEX"
          }
        )
        (with "statistics"
          it "verifies the pack index successfully and with desired output" && {
            WITH_SNAPSHOT="$snapshot/index-with-statistics-success" \
            expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose free pack verify --statistics "$MULTI_PACK_INDEX"
          }

          (with "and the less-memory algorithm"
            it "verifies the pack index successfully and with desired output" && {
              WITH_SNAPSHOT="$snapshot/index-with-statistics-success" \
              expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose free pack verify --algorithm less-memory --statistics "$MULTI_PACK_INDEX"
            }
          )
        )
        (with "decode"
          it "verifies the pack index successfully and with desired output, and decodes all objects" && {
            WITH_SNAPSHOT="$snapshot/index-success" \
            expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose  free pack verify --algorithm less-memory --decode "$MULTI_PACK_INDEX"
          }
        )
        (with "re-encode"
          it "verifies the pack index successfully and with desired output, and re-encodes all objects" && {
            WITH_SNAPSHOT="$snapshot/index-success" \
            expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose free pack verify --algorithm less-time --re-encode "$MULTI_PACK_INDEX"
          }
        )
        if test "$kind" = "max" || test "$kind" = "max-pure"; then
        (with "statistics (JSON)"
          it "verifies the pack index successfully and with desired output" && {
            WITH_SNAPSHOT="$snapshot/index-with-statistics-json-success" \
            expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose --format json --threads 1 free pack verify --statistics "$MULTI_PACK_INDEX"
          }
        )
        fi
      )
      (with "a valid multi-pack index"
        snapshot="$snapshot/multi-index"
        (sandbox
          MULTI_PACK_INDEX=multi-pack-index
          cp $fixtures/packs/pack-* .
          $exe_plumbing free pack multi-index -i $MULTI_PACK_INDEX create *.idx

          (when "using fast validation via 'pack multi-index verify'"
            it "verifies the pack index successfully and with desired output" && {
              WITH_SNAPSHOT="$snapshot/fast-index-success" \
              expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose free pack multi-index -i "$MULTI_PACK_INDEX" verify
            }
          )

          (with "no statistics"
            it "verifies the pack index successfully and with desired output" && {
              WITH_SNAPSHOT="$snapshot/index-success" \
              expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose free pack verify "$MULTI_PACK_INDEX"
            }
          )
          (with "statistics"
            it "verifies the pack index successfully and with desired output" && {
              WITH_SNAPSHOT="$snapshot/index-with-statistics-success" \
              expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose free pack verify --statistics "$MULTI_PACK_INDEX"
            }

            (with "and the less-memory algorithm"
              it "verifies the pack index successfully and with desired output" && {
                WITH_SNAPSHOT="$snapshot/index-with-statistics-success" \
                expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose free pack verify --algorithm less-memory --statistics "$MULTI_PACK_INDEX"
              }
            )
          )
          (with "decode"
            it "verifies the pack index successfully and with desired output, and decodes all objects" && {
              WITH_SNAPSHOT="$snapshot/index-success" \
              expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose free pack verify --algorithm less-memory --decode "$MULTI_PACK_INDEX"
            }
          )
          (with "re-encode"
            it "verifies the pack index successfully and with desired output, and re-encodes all objects" && {
              WITH_SNAPSHOT="$snapshot/index-success" \
              expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose free pack verify --algorithm less-time --re-encode "$MULTI_PACK_INDEX"
            }
          )
          if test "$kind" = "max" || test "$kind" = "max-pure"; then
          (with "statistics (JSON)"
            it "verifies the pack index successfully and with desired output" && {
              WITH_SNAPSHOT="$snapshot/index-with-statistics-json-success" \
              expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose --format json --threads 1 free pack verify --statistics "$MULTI_PACK_INDEX"
            }
          )
          fi
        )
      )
      (sandbox
        (with "an INvalid pack INDEX file"
          MULTI_PACK_INDEX="$fixtures/packs/pack-11fdfa9e156ab73caae3b6da867192221f2089c2.idx"
          cp $MULTI_PACK_INDEX index.idx
          echo $'\0' >> index.idx
          it "fails to verify the pack index and with desired output" && {
            WITH_SNAPSHOT="$snapshot/index-failure" \
            expect_run $WITH_FAILURE "$exe_plumbing" --no-verbose free pack verify index.idx
          }
        )
      )
    )
  )

  title "gix free commit-graph"
  (when "running 'commit-graph'"
    snapshot="$snapshot/commit-graph"
    title "gix free commit-graph verify"
    (with "the 'verify' sub-command"
      snapshot="$snapshot/verify"

      (small-repo-in-sandbox
        (with "a valid and complete commit-graph file"
          git commit-graph write --reachable
          (with "statistics"
            it "generates the correct output" && {
              WITH_SNAPSHOT="$snapshot/statistics-success" \
              expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose free commit-graph verify -s .git/objects/info
            }
          )
          if test "$kind" = "max" || test "$kind" = "max-pure"; then
          (with "statistics --format json"
            it "generates the correct output" && {
              WITH_SNAPSHOT="$snapshot/statistics-json-success" \
              expect_run $SUCCESSFULLY "$exe_plumbing" --no-verbose --format json free commit-graph verify -s .git/objects/info
            }
          )
          fi
        )
      )
    )
  )
)
