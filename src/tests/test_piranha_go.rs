/*
Copyright (c) 2022 Uber Technologies, Inc.

 <p>Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file
 except in compliance with the License. You may obtain a copy of the License at
 <p>http://www.apache.org/licenses/LICENSE-2.0

 <p>Unless required by applicable law or agreed to in writing, software distributed under the
 License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
 express or implied. See the License for the specific language governing permissions and
 limitations under the License.
*/

use super::{
  check_result, copy_folder, create_match_tests, create_rewrite_tests, initialize, substitutions,
};
use crate::execute_piranha;
use crate::models::{
  default_configs::GO,
  piranha_arguments::{piranha_arguments, PiranhaArgumentsBuilder},
};
use std::path::{Path, PathBuf};
use tempdir::TempDir;

create_match_tests! {
  GO,
  test_match_only_for_loop: "structural_find/go_stmt_for_loop", 1;
  test_match_only_go_stmt_for_loop:"structural_find/for_loop", 4;
}

create_rewrite_tests! {
  GO,
  test_builtin_boolean_expression_simplify:  "feature_flag/builtin_rules/boolean_expression_simplify", 1,
    substitutions= substitutions! {
      "true_flag_name" => "true",
      "false_flag_name" => "false",
      "nil_flag_name" => "nil"
    };
  test_builtin_statement_cleanup: "feature_flag/builtin_rules/statement_cleanup", 1,
    substitutions= substitutions! {
      "treated" => "true",
      "treated_complement" => "false"
    };
  test_const_same_file: "feature_flag/system_1/const_same_file", 1,
    substitutions= substitutions! {
      "stale_flag_name" => "staleFlag",
      "treated" => "false"
    };
}
