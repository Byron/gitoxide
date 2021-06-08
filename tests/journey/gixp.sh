# Must be sourced into the main journey test
set -eu

title plumbing "${kind}"
snapshot="$snapshot/plumbing"
title "gixp pack-receive"
(when "running 'pack-receive'"
  snapshot="$snapshot/pack-receive"
  (small-repo-in-sandbox
    (with "file:// protocol"
      (with "version 1"
        (with "NO output directory"
          it "generates the correct output" && {
            WITH_SNAPSHOT="$snapshot/file-v-any-no-output" \
            expect_run $SUCCESSFULLY "$exe_plumbing" pack-receive -p 1 .git
          }
        )
        (with "output directory"
          mkdir out
          it "generates the correct output" && {
            WITH_SNAPSHOT="$snapshot/file-v-any-with-output" \
            expect_run $SUCCESSFULLY "$exe_plumbing" pack-receive -p 1 .git out/
          }
          it "creates an index and a pack in the output directory" && {
            WITH_SNAPSHOT="$snapshot/ls-in-output-dir" \
            expect_run $SUCCESSFULLY ls out/
          }
          (with "--write-refs set"
            it "generates the correct output" && {
              WITH_SNAPSHOT="$snapshot/file-v-any-with-output" \
              expect_run $SUCCESSFULLY "$exe_plumbing" pack-receive -p 1 --refs-directory out/all-refs .git out/
            }
            it "writes references into the refs folder of the output directory" && {
              expect_snapshot "$snapshot/repo-refs" out/all-refs
            }
          )
          rm -Rf out
        )
        if test "$kind" = "max"; then
        (with "--format json"
          it "generates the correct output in JSON format" && {
            WITH_SNAPSHOT="$snapshot/file-v-any-no-output-json" \
            expect_run $SUCCESSFULLY "$exe_plumbing" --format json pack-receive --protocol 1 .git
          }
        )
        fi
      )
      (with "version 2"
        (with "NO output directory"
          it "generates the correct output" && {
            WITH_SNAPSHOT="$snapshot/file-v-any-no-output" \
            expect_run $SUCCESSFULLY "$exe_plumbing" pack-receive -p 2 .git
          }
        )
        (with "output directory"
          mkdir out/
          it "generates the correct output" && {
            WITH_SNAPSHOT="$snapshot/file-v-any-with-output" \
            expect_run $SUCCESSFULLY "$exe_plumbing" pack-receive .git out/
          }
          it "creates an index and a pack in the output directory" && {
            WITH_SNAPSHOT="$snapshot/ls-in-output-dir" \
            expect_run $SUCCESSFULLY ls out/
          }
          rm -Rf out
        )
        if test "$kind" = "max"; then
        (with "--format json"
          it "generates the correct output in JSON format" && {
            WITH_SNAPSHOT="$snapshot/file-v-any-no-output-json" \
            expect_run $SUCCESSFULLY "$exe_plumbing" --format json pack-receive --protocol 2 .git
          }
        )
        fi
      )
    )
    (with "git:// protocol"
      launch-git-daemon
      (with "version 1"
        (with "NO output directory"
          it "generates the correct output" && {
            WITH_SNAPSHOT="$snapshot/file-v-any-no-output" \
            expect_run $SUCCESSFULLY "$exe_plumbing" pack-receive -p 1 git://localhost/
          }
        )
        (with "output directory"
          mkdir out
          it "generates the correct output" && {
            WITH_SNAPSHOT="$snapshot/file-v-any-with-output" \
            expect_run $SUCCESSFULLY "$exe_plumbing" pack-receive -p 1 git://localhost/ out/
          }
        )
      )
      (with "version 2"
        (with "NO output directory"
          it "generates the correct output" && {
            WITH_SNAPSHOT="$snapshot/file-v-any-no-output" \
            expect_run $SUCCESSFULLY "$exe_plumbing" pack-receive -p 2 git://localhost/
          }
        )
        (with "output directory"
          it "generates the correct output" && {
            WITH_SNAPSHOT="$snapshot/file-v-any-with-output" \
            expect_run $SUCCESSFULLY "$exe_plumbing" pack-receive git://localhost/ out/
          }
        )
      )
    )
    (on_ci
      if test "$kind" = "max"; then
      (with "https:// protocol"
        (with "version 1"
          it "works" && {
            expect_run $SUCCESSFULLY "$exe_plumbing" pack-receive -p 1 https://github.com/byron/gitoxide
          }
        )
        (with "version 2"
          it "works" && {
            expect_run $SUCCESSFULLY "$exe_plumbing" pack-receive -p 2 https://github.com/byron/gitoxide
          }
        )
      )
      fi
    )
  )
)
title "gixp remote-ref-list"
(when "running 'remote-ref-list'"
  snapshot="$snapshot/remote-ref-list"
  (small-repo-in-sandbox
    (with "file:// protocol"
      (with "version 1"
        it "generates the correct output" && {
          WITH_SNAPSHOT="$snapshot/file-v-any" \
          expect_run $SUCCESSFULLY "$exe_plumbing" remote-ref-list -p 1 .git
        }
      )
      (with "version 2"
        it "generates the correct output" && {
          WITH_SNAPSHOT="$snapshot/file-v-any" \
          expect_run $SUCCESSFULLY "$exe_plumbing" remote-ref-list --protocol 2 "$PWD/.git"
        }
      )
      if test "$kind" = "max"; then
      (with "--format json"
        it "generates the correct output in JSON format" && {
          WITH_SNAPSHOT="$snapshot/file-v-any-json" \
          expect_run $SUCCESSFULLY "$exe_plumbing" --format json remote-ref-list .git
        }
      )
      fi
    )
    (with "git:// protocol"
      launch-git-daemon
      (with "version 1"
        it "generates the correct output" && {
          WITH_SNAPSHOT="$snapshot/file-v-any" \
          expect_run $SUCCESSFULLY "$exe_plumbing" remote-ref-list -p 1 git://localhost/
        }
      )
      (with "version 2"
        it "generates the correct output" && {
          WITH_SNAPSHOT="$snapshot/file-v-any" \
          expect_run $SUCCESSFULLY "$exe_plumbing" remote-ref-list -p 2 git://localhost/
        }
      )
    )
    if test "$kind" != "max"; then
    (with "https:// protocol (in small builds)"
      it "fails as http is not compiled in" && {
        WITH_SNAPSHOT="$snapshot/fail-http-in-small" \
        expect_run $WITH_FAILURE "$exe_plumbing" remote-ref-list -p 1 https://github.com/byron/gitoxide
      }
    )
    fi
    (on_ci
      if test "$kind" = "max"; then
      (with "https:// protocol"
        (with "version 1"
          it "generates the correct output" && {
            expect_run $SUCCESSFULLY "$exe_plumbing" remote-ref-list -p 1 https://github.com/byron/gitoxide
          }
        )
        (with "version 2"
          it "generates the correct output" && {
            expect_run $SUCCESSFULLY "$exe_plumbing" remote-ref-list -p 2 https://github.com/byron/gitoxide
          }
        )
      )
      fi
    )
  )
)
title "gixp pack-index-from-data"
(when "running 'pack-index-from-data"
  snapshot="$snapshot/pack-index-from-data"
  PACK_FILE="$fixtures/packs/pack-11fdfa9e156ab73caae3b6da867192221f2089c2.pack"
  (with "a valid and complete pack file"
    (with "NO output directory specified"
      (with "pack file passed as file"
        it "generates an index into a sink and outputs pack and index information" && {
          WITH_SNAPSHOT="$snapshot/no-output-dir-success" \
          expect_run $SUCCESSFULLY "$exe_plumbing" pack-index-from-data -p "$PACK_FILE"
        }
      )
      (with "pack file passed from stdin"
        it "generates an index into a sink and outputs pack and index information" && {
          WITH_SNAPSHOT="$snapshot/no-output-dir-success" \
          expect_run $SUCCESSFULLY "$exe_plumbing" pack-index-from-data < "$PACK_FILE"
        }
        if test "$kind" = "max"; then
        (with "--format json"
          it "generates the index into a sink and outputs information as JSON" && {
            WITH_SNAPSHOT="$snapshot/no-output-dir-as-json-success" \
            expect_run $SUCCESSFULLY "$exe_plumbing" --format json pack-index-from-data < "$PACK_FILE"
          }
        )
        fi
      )
    )
    (sandbox
      (with "with an output directory specified"
        it "generates an index and outputs information" && {
          WITH_SNAPSHOT="$snapshot/output-dir-success" \
          expect_run $SUCCESSFULLY "$exe_plumbing" pack-index-from-data -p "$PACK_FILE" "$PWD"
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
        expect_run $SUCCESSFULLY "$exe_plumbing" pack-index-from-data -i restore -p "$PACK_FILE" "$PWD"
      }

      if test "$kind" = "max"; then
      (with "--format json and the very same output directory"
        it "generates the index, overwriting existing files, and outputs information as JSON" && {
          WITH_SNAPSHOT="$snapshot/output-dir-restore-as-json-success" \
          SNAPSHOT_FILTER=remove-paths \
          expect_run $SUCCESSFULLY "$exe_plumbing" --format json pack-index-from-data -i restore $PWD < "$PACK_FILE"
        }
      )
      fi
    )
  )
)
title "gixp pack-explode"
(when "running 'pack-explode"
  snapshot="$snapshot/pack-explode"
  PACK_FILE="$fixtures/packs/pack-11fdfa9e156ab73caae3b6da867192221f2089c2"
  (with "no objects directory specified"
    it "explodes the pack successfully and with desired output" && {
      WITH_SNAPSHOT="$snapshot/to-sink-success" \
      expect_run $SUCCESSFULLY "$exe_plumbing" pack-explode "${PACK_FILE}.idx"
    }

    (when "using the --delete-pack flag"
      (sandbox
        (with "a valid pack"
          cp "${PACK_FILE}".idx "${PACK_FILE}".pack .
          PACK_FILE="${PACK_FILE##*/}"
          it "explodes the pack successfully and deletes the original pack and index" && {
            WITH_SNAPSHOT="$snapshot/to-sink-delete-pack-success" \
            expect_run $SUCCESSFULLY "$exe_plumbing" pack-explode --check skip-file-checksum --delete-pack "${PACK_FILE}.pack"
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
              expect_run $WITH_FAILURE "$exe_plumbing" pack-explode --sink-compress --check all --delete-pack "${PACK_FILE}.pack"
            }

            it "did not touch index or pack file" && {
              expect_exists "${PACK_FILE}".pack
              expect_exists "${PACK_FILE}".idx
            }
          )

          (with "and no safety checks at all (and an output directory)"
            it "does explode the file" && {
              WITH_SNAPSHOT="$snapshot/broken-delete-pack-to-sink-skip-checks-success" \
              expect_run $SUCCESSFULLY "$exe_plumbing" pack-explode --verify --check skip-file-and-object-checksum-and-no-abort-on-decode \
                                        --delete-pack "${PACK_FILE}.pack" .
            }

            it "removes the original files" && {
              expect_run $WITH_FAILURE test -e "${PACK_FILE}".pack
              expect_run $WITH_FAILURE test -e "${PACK_FILE}".idx
            }

            (with_program tree

              if test "$kind" == "max"; then
                suffix=zlib-ng
              else
                suffix=miniz-oxide
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
      expect_run $WITH_FAILURE "$exe_plumbing" pack-explode -c skip-file-and-object-checksum "${PACK_FILE}.idx" does-not-exist
    }
  )
  (with "an existing directory specified"
    (sandbox
      it "succeeds" && {
        WITH_SNAPSHOT="$snapshot/with-objects-dir-success" \
        expect_run $SUCCESSFULLY "$exe_plumbing" pack-explode -c skip-file-and-object-checksum-and-no-abort-on-decode \
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

title "gixp pack-verify"
(when "running 'pack-verify"
  snapshot="$snapshot/pack-verify"
  (with "a valid pack file"
    PACK_FILE="$fixtures/packs/pack-11fdfa9e156ab73caae3b6da867192221f2089c2.pack"
    it "verifies the pack successfully and with desired output" && {
      WITH_SNAPSHOT="$snapshot/success" \
      expect_run $SUCCESSFULLY "$exe_plumbing" pack-verify "$PACK_FILE"
    }
  )
  (with "a valid pack INDEX file"
    PACK_INDEX_FILE="$fixtures/packs/pack-11fdfa9e156ab73caae3b6da867192221f2089c2.idx"
    (with "no statistics"
      it "verifies the pack index successfully and with desired output" && {
        WITH_SNAPSHOT="$snapshot/index-success" \
        expect_run $SUCCESSFULLY "$exe_plumbing" pack-verify "$PACK_INDEX_FILE"
      }
    )
    (with "statistics"
      it "verifies the pack index successfully and with desired output" && {
        WITH_SNAPSHOT="$snapshot/index-with-statistics-success" \
        expect_run $SUCCESSFULLY "$exe_plumbing" pack-verify --statistics "$PACK_INDEX_FILE"
      }

      (with "and the less-memory algorithm"
        it "verifies the pack index successfully and with desired output" && {
          WITH_SNAPSHOT="$snapshot/index-with-statistics-success" \
          expect_run $SUCCESSFULLY "$exe_plumbing" pack-verify --algorithm less-memory --statistics "$PACK_INDEX_FILE"
        }
      )
    )
    (with "decode"
      it "verifies the pack index successfully and with desired output, and decodes all objects" && {
        WITH_SNAPSHOT="$snapshot/index-success" \
        expect_run $SUCCESSFULLY "$exe_plumbing" pack-verify --algorithm less-memory --decode "$PACK_INDEX_FILE"
      }
    )
    (with "re-encode"
      it "verifies the pack index successfully and with desired output, and re-encodes all objects" && {
        WITH_SNAPSHOT="$snapshot/index-success" \
        expect_run $SUCCESSFULLY "$exe_plumbing" pack-verify --algorithm less-time --re-encode "$PACK_INDEX_FILE"
      }
    )
    if test "$kind" = "max"; then
    (with "statistics (JSON)"
      it "verifies the pack index successfully and with desired output" && {
        WITH_SNAPSHOT="$snapshot/index-with-statistics-json-success" \
        expect_run $SUCCESSFULLY "$exe_plumbing" --format json --threads 1 pack-verify --statistics "$PACK_INDEX_FILE"
      }
    )
    fi
  )
  (sandbox
    (with "an INvalid pack INDEX file"
      PACK_INDEX_FILE="$fixtures/packs/pack-11fdfa9e156ab73caae3b6da867192221f2089c2.idx"
      cp $PACK_INDEX_FILE index.idx
      echo $'\0' >> index.idx
      it "fails to verify the pack index and with desired output" && {
        WITH_SNAPSHOT="$snapshot/index-failure" \
        expect_run $WITH_FAILURE "$exe_plumbing" pack-verify index.idx
      }
    )
  )
)
title "gixp commit-graph-verify"
(when "running 'commit-graph-verify'"
  snapshot="$snapshot/commit-graph-verify"
  (small-repo-in-sandbox
    (with "a valid and complete commit-graph file"
      git commit-graph write --reachable
      (with "statistics"
        it "generates the correct output" && {
          WITH_SNAPSHOT="$snapshot/statistics-success" \
          expect_run $SUCCESSFULLY "$exe_plumbing" commit-graph-verify -s .git/objects/info
        }
      )
      if test "$kind" = "max"; then
      (with "statistics --format json"
        it "generates the correct output" && {
          WITH_SNAPSHOT="$snapshot/statistics-json-success" \
          expect_run $SUCCESSFULLY "$exe_plumbing" --format json commit-graph-verify -s .git/objects/info
        }
      )
      fi
    )
  )
)
