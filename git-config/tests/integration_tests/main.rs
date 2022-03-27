// See https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html
// for an explanation of why the integration tests are laid out like this.
//
// TL;DR single mod makes integration tests faster to compile, test, and with
// less build artifacts.

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

mod file_integeration_test;
mod parser_integration_tests;
