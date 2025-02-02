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
use super::{create_match_tests, initialize};
use crate::execute_piranha;
use crate::models::{
  default_configs::TYPESCRIPT, piranha_arguments::piranha_arguments,
  piranha_arguments::PiranhaArgumentsBuilder,
};
use std::path::PathBuf;

create_match_tests! {
  TYPESCRIPT,
  test_find_fors_within_functions_not_within_whiles:  "structural_find/find_fors_within_functions_not_within_whiles", 1;
  test_find_fors_within_functions:"structural_find/find_fors_within_functions", 2;
  test_find_fors: "structural_find/find_fors", 3;
}
