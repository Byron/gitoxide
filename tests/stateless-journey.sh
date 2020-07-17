#!/usr/bin/env bash
set -eu

exe=${1:?First argument must be the executable to test}
exe_plumbing=${2:?Second argument must be the plumbing executable to test}
kind=${3:?third argument must an indicator of the kind of binary under test}

root="$(cd "${0%/*}" && pwd)"
exe="${root}/../$exe"
exe_plumbing="${root}/../$exe_plumbing"

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

  (when "running 'plumbing verify pack"
    (with "a valid pack file"
      PACK_FILE="$fixtures/packs/pack-11fdfa9e156ab73caae3b6da867192221f2089c2.pack"
      it "verifies the pack successfully and with desired output" && {
        WITH_SNAPSHOT="$snapshot/plumbing-verify-pack-success" \
        expect_run $SUCCESSFULLY "$exe_plumbing" verify-pack "$PACK_FILE"
      }
    )
    (with "a valid pack INDEX file"
      PACK_INDEX_FILE="$fixtures/packs/pack-11fdfa9e156ab73caae3b6da867192221f2089c2.idx"
      (with "no statistics"
        it "verifies the pack index successfully and with desired output" && {
          WITH_SNAPSHOT="$snapshot/plumbing-verify-pack-index-success" \
          expect_run $SUCCESSFULLY "$exe_plumbing" verify-pack "$PACK_INDEX_FILE"
        }
      )
      (with "statistics"
        it "verifies the pack index successfully and with desired output" && {
          WITH_SNAPSHOT="$snapshot/plumbing-verify-pack-index-with-statistics-success" \
          expect_run $SUCCESSFULLY "$exe_plumbing" verify-pack --statistics "$PACK_INDEX_FILE"
        }
      )
      (with "decode"
        it "verifies the pack index successfully and with desired output, and decodes all objects" && {
          WITH_SNAPSHOT="$snapshot/plumbing-verify-pack-index-success" \
          expect_run $SUCCESSFULLY "$exe_plumbing" verify-pack --decode "$PACK_INDEX_FILE"
        }
      )
      if test "$kind" = "max"; then
      (with "statistics (JSON)"
        it "verifies the pack index successfully and with desired output" && {
          WITH_SNAPSHOT="$snapshot/plumbing-verify-pack-index-with-statistics-json-success" \
          expect_run $SUCCESSFULLY "$exe_plumbing" --threads 1 verify-pack --statistics --format json "$PACK_INDEX_FILE"
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
          WITH_SNAPSHOT="$snapshot/plumbing-verify-pack-index-failure" \
          expect_run $WITH_FAILURE "$exe_plumbing" verify-pack index.idx
        }
      )
    )
  )
)

