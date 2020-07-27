#!/usr/bin/env bash
set -eu

exe=${1:?First argument must be the executable to test}
exe_plumbing=${2:?Second argument must be the plumbing executable to test}
jtt=${3:?Third argument the journey test tool}
kind=${4:?Fourth argument must an indicator of the kind of binary under test}

root="$(cd "${0%/*}" && pwd)"
exe="${root}/../$exe"
exe_plumbing="${root}/../$exe_plumbing"
jtt="${root}/../$jtt"

# shellcheck disable=1090
source "$root/utilities.sh"
snapshot="$root/snapshots"
fixtures="$root/fixtures"

SUCCESSFULLY=0
WITH_FAILURE=1

title "CLI ${kind}"
(when "initializing a repository"
  (with "an empty directory"
    (sandbox
      it "succeeds" && {
        WITH_SNAPSHOT="$snapshot/init-success" \
        expect_run $SUCCESSFULLY "$exe" init
      }

      it "matches the output of baseline git init" && {
        expect_snapshot "$fixtures/baseline-init" .git
      }
      
      (when "trying to initialize the same directory again"
        it "fails" && {
          WITH_SNAPSHOT="$snapshot/init-fail" \
          expect_run $WITH_FAILURE "$exe" init
        }
      )
    )
  )
)

(when "running 'plumbing pack-explode"
  PACK_FILE="$fixtures/packs/pack-11fdfa9e156ab73caae3b6da867192221f2089c2"
  (with "no objects directory specified"
    it "explodes the pack successfully and with desired output" && {
      WITH_SNAPSHOT="$snapshot/plumbing-pack-explode-to-sink-success" \
      expect_run $SUCCESSFULLY "$exe_plumbing" pack-explode "${PACK_FILE}.idx"
    }

    (when "using the --delete-pack flag"
      (sandbox
        (with "a valid pack"
          cp ${PACK_FILE}.idx ${PACK_FILE}.pack .
          PACK_FILE="${PACK_FILE##*/}"
          it "explodes the pack successfully and deletes the original pack and index" && {
            WITH_SNAPSHOT="$snapshot/plumbing-pack-explode-to-sink-delete-pack-success" \
            expect_run $SUCCESSFULLY "$exe_plumbing" pack-explode --check skip-file-checksum --delete-pack "${PACK_FILE}.pack"
          }
          it "removes the original files" && {
            expect_run $WITH_FAILURE test -e ${PACK_FILE}.pack
            expect_run $WITH_FAILURE test -e ${PACK_FILE}.idx
          }
        )
        (with "a pack file that is invalid somewhere"
          cp ${PACK_FILE}.idx ${PACK_FILE}.pack .
          PACK_FILE="${PACK_FILE##*/}"
          "$jtt" mess-in-the-middle ${PACK_FILE}.pack

          (with "and all safety checks"
            it "does not explode the file at all" && {
              WITH_SNAPSHOT="$snapshot/plumbing-broken-pack-explode-delete-pack-to-objects-dir-failure" \
              expect_run $WITH_FAILURE "$exe_plumbing" pack-explode --check all --delete-pack "${PACK_FILE}.pack"
            }

            it "did not touch index or pack file" && {
              expect_exists ${PACK_FILE}.pack
              expect_exists ${PACK_FILE}.idx
            }
          )
        )
      )
    )
  )
  (with "a non-existing directory specified"
    it "fails with a helpful error message" && {
      WITH_SNAPSHOT="$snapshot/plumbing-pack-explode-missing-objects-dir-fail" \
      expect_run $WITH_FAILURE "$exe_plumbing" pack-explode -c skip-file-and-object-checksum "${PACK_FILE}.idx" does-not-exist
    }
  )
  (with "an existing directory specified"
    (sandbox
      it "succeeds" && {
        WITH_SNAPSHOT="$snapshot/plumbing-pack-explode-with-objects-dir-success" \
        expect_run $SUCCESSFULLY "$exe_plumbing" pack-explode -c skip-file-and-object-checksum-and-no-abort-on-decode \
                                                 "${PACK_FILE}.pack" .
      }

      (with_program tree
        it "creates all pack objects" && {
          WITH_SNAPSHOT="$snapshot/plumbing-pack-explode-with-objects-dir-success-tree" \
          expect_run $SUCCESSFULLY tree
        }
      )
    )
  )
)

(when "running 'plumbing pack-verify"
  (with "a valid pack file"
    PACK_FILE="$fixtures/packs/pack-11fdfa9e156ab73caae3b6da867192221f2089c2.pack"
    it "verifies the pack successfully and with desired output" && {
      WITH_SNAPSHOT="$snapshot/plumbing-pack-verify-success" \
      expect_run $SUCCESSFULLY "$exe_plumbing" pack-verify "$PACK_FILE"
    }
  )
  (with "a valid pack INDEX file"
    PACK_INDEX_FILE="$fixtures/packs/pack-11fdfa9e156ab73caae3b6da867192221f2089c2.idx"
    (with "no statistics"
      it "verifies the pack index successfully and with desired output" && {
        WITH_SNAPSHOT="$snapshot/plumbing-pack-verify-index-success" \
        expect_run $SUCCESSFULLY "$exe_plumbing" pack-verify "$PACK_INDEX_FILE"
      }
    )
    (with "statistics"
      it "verifies the pack index successfully and with desired output" && {
        WITH_SNAPSHOT="$snapshot/plumbing-pack-verify-index-with-statistics-success" \
        expect_run $SUCCESSFULLY "$exe_plumbing" pack-verify --statistics "$PACK_INDEX_FILE"
      }
    )
    (with "decode"
      it "verifies the pack index successfully and with desired output, and decodes all objects" && {
        WITH_SNAPSHOT="$snapshot/plumbing-pack-verify-index-success" \
        expect_run $SUCCESSFULLY "$exe_plumbing" pack-verify --algorithm less-memory --decode "$PACK_INDEX_FILE"
      }
    )
    (with "re-encode"
      it "verifies the pack index successfully and with desired output, and re-encodes all objects" && {
        WITH_SNAPSHOT="$snapshot/plumbing-pack-verify-index-success" \
        expect_run $SUCCESSFULLY "$exe_plumbing" pack-verify --algorithm less-time --re-encode "$PACK_INDEX_FILE"
      }
    )
    if test "$kind" = "max"; then
    (with "statistics (JSON)"
      it "verifies the pack index successfully and with desired output" && {
        WITH_SNAPSHOT="$snapshot/plumbing-pack-verify-index-with-statistics-json-success" \
        expect_run $SUCCESSFULLY "$exe_plumbing" --threads 1 pack-verify --statistics --format json "$PACK_INDEX_FILE"
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
        WITH_SNAPSHOT="$snapshot/plumbing-pack-verify-index-failure" \
        expect_run $WITH_FAILURE "$exe_plumbing" pack-verify index.idx
      }
    )
  )
)
