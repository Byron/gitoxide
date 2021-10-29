# Must be sourced into the main journey test
set -eu

if test "$kind" = "max"; then
title "Porcelain ${kind}"
(
  (when "running a debug-only panic test"
    snapshot="$snapshot/panic-behaviour"
    (with "the --quiet option set"
      it "fails as expected" && {
        WITH_SNAPSHOT="$snapshot/expected-failure" \
        expect_run_sh 101 "$exe -q panic"
      }
    )

    (with "NO --quiet option set"
      it "fails as expected" && {
        WITH_SNAPSHOT="$snapshot/expected-failure-in-thread" \
        expect_run_sh 101 "$exe panic"
      }
    )
    (not_on_ci # due to different TTY settings, the output differs, it's OK for now
      (with "progress option set"
        it "fails as expected" && {
          WITH_SNAPSHOT="$snapshot/expected-failure-in-thread-with-progress" \
          expect_run_sh $WITH_FAILURE "$exe --progress panic"
        }
      )
    )
  )
  snapshot="$snapshot/porcelain"
  (with_program tree
    (when "using the 'tools' subcommand"
      title "gix tools…"
      (with "a repo with a tiny commit history"
        (small-repo-in-sandbox
          title "gix tools estimate-hours"
          (when "running 'estimate-hours'"
            snapshot="$snapshot/estimate-hours"
            (with "no arguments"
              it "succeeds and prints only a summary" && {
                WITH_SNAPSHOT="$snapshot/no-args-success" \
                expect_run_sh $SUCCESSFULLY "$exe tools estimate-hours 2>/dev/null"
              }
            )
            (with "the show-pii argument"
              it "succeeds and shows information identifying people before the summary" && {
                WITH_SNAPSHOT="$snapshot/show-pii-success" \
                expect_run_sh $SUCCESSFULLY "$exe tools estimate-hours --show-pii 2>/dev/null"
              }
            )
            (with "the omit-unify-identities argument"
              it "succeeds and doesn't show unified identities (in this case there is only one author anyway)" && {
                WITH_SNAPSHOT="$snapshot/no-unify-identities-success" \
                expect_run_sh $SUCCESSFULLY "$exe tools estimate-hours --omit-unify-identities 2>/dev/null"
              }
            )
            (with "a branch name that doesn't exist"
              it "fails and shows a decent enough error message" && {
                WITH_SNAPSHOT="$snapshot/invalid-branch-name-failure" \
                expect_run_sh $WITH_FAILURE "$exe -q tools estimate-hours . foobar"
              }
            )
          )
        )
      )
      (with "a mix of repositories"
        (sandbox
          repo-with-remotes dir/one-origin origin https://example.com/one-origin
          repo-with-remotes origin-and-fork origin https://example.com/origin-and-fork fork https://example.com/other/origin-and-fork
          repo-with-remotes special-origin special-name https://example.com/special-origin
          repo-with-remotes no-origin
          repo-with-remotes a-non-bare-repo-with-extension.git origin https://example.com/a-repo-with-extension.git
          snapshot="$snapshot/tools"

          title "gix tools find"
          (when "running 'find'"
            snapshot="$snapshot/find"
            (with "no arguments"
              it "succeeds and prints a list of repository work directories" && {
                WITH_SNAPSHOT="$snapshot/no-args-success" \
                expect_run_sh $SUCCESSFULLY "$exe tools find 2>/dev/null"
              }
            )
          )
          title "gix tools organize"
          (when "running 'organize'"
            snapshot="$snapshot/organize"
            (with "no arguments"
              it "succeeds and informs about the operations that it WOULD do" && {
                WITH_SNAPSHOT="$snapshot/no-args-success" \
                expect_run_sh $SUCCESSFULLY "$exe tools organize 2>/dev/null"
              }

              it "does not change the directory structure at all" && {
                WITH_SNAPSHOT="$snapshot/initial-directory-structure" \
                expect_run $SUCCESSFULLY tree -L 2
              }
            )

            (with "--execute"
              it "succeeds" && {
                WITH_SNAPSHOT="$snapshot/execute-success" \
                expect_run_sh $SUCCESSFULLY "$exe tools organize --execute 2>/dev/null"
              }

              it "changes the directory structure" && {
                WITH_SNAPSHOT="$snapshot/directory-structure-after-organize" \
                expect_run $SUCCESSFULLY tree -L 2
              }
            )

            (with "--execute again"
              it "succeeds" && {
                WITH_SNAPSHOT="$snapshot/execute-success" \
                expect_run_sh $SUCCESSFULLY "$exe tools organize --execute 2>/dev/null"
              }

              it "does not alter the directory structure as these are already in place" && {
                WITH_SNAPSHOT="$snapshot/directory-structure-after-organize" \
                expect_run $SUCCESSFULLY tree -L 2
              }
            )
          )
          (with "running with no further arguments"
            it "succeeds and informs about possible operations" && {
              WITH_SNAPSHOT="$snapshot/no-args-failure" \
              expect_run_sh $WITH_CLAP_FAILURE "$exe t"
            }
          )
        )
      )
    )
  )

  title "gix init"
  (when "running 'init'"
    snapshot="$snapshot/init"
    (with "no argument"
      (with "an empty directory"
        (sandbox
          it "succeeds" && {
            WITH_SNAPSHOT="$snapshot/success" \
            expect_run $SUCCESSFULLY "$exe" init
          }

          it "matches the output of baseline git init" && {
            rm .git/config # this one is altered, ignore
            expect_snapshot "$fixtures/baseline-init" .git
          }

          (when "trying to initialize the same directory again"
            it "fails" && {
              WITH_SNAPSHOT="$snapshot/fail" \
              expect_run $WITH_FAILURE "$exe" init
            }
          )
        )
      )
    )
    (with "a single argument denoting the directory to initialize"
      DIR=foo/bar
      (with "a multi-element directory: $DIR"
        (sandbox
          it "succeeds" && {
            WITH_SNAPSHOT="$snapshot/success-with-multi-element-directory" \
            expect_run $SUCCESSFULLY "$exe" init $DIR
          }

          it "matches the output of baseline git init" && {
            rm $DIR/.git/config # this one is altered, ignore
            expect_snapshot "$fixtures/baseline-init" $DIR/.git
          }

          (when "trying to initialize the same directory again"
            it "fails" && {
              WITH_SNAPSHOT="$snapshot/fail-with-multi-element-directory" \
              expect_run $WITH_FAILURE "$exe" init $DIR
            }
          )
        )
      )
    )
  )
)
fi
