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

use crate::utilities::{eq_without_whitespace, read_file};
use log::error;
use std::fs;
use std::path::Path;

mod test_piranha_java;
mod test_piranha_kt;

mod test_piranha_swift;

mod test_piranha_python;

mod test_piranha_go;
mod test_piranha_ts;
mod test_piranha_tsx;

use std::sync::Once;

static INIT: Once = Once::new();

fn initialize() {
  INIT.call_once(|| {
    env_logger::init();
  });
}

/// Copies the files under `src` to `dst`.
/// The copy is NOT recursive.
/// The files under `src` are copied under `dst`.
///
/// # Arguments
///
/// * src: Path to the directory to be copied
/// * dest: Path to destination
///
/// This method causes side effects - writes new files to a directory
fn copy_folder(src: &Path, dst: &Path) {
  for entry in fs::read_dir(src).unwrap() {
    let entry = entry.unwrap();
    let path = entry.path();
    if path.is_file() {
      _ = fs::copy(
        path.to_str().unwrap(),
        dst.join(path.file_name().unwrap()).to_str().unwrap(),
      )
      .unwrap();
    }
  }
}

/// Checks if the file updates returned by piranha are as expected.
fn check_result(path_to_codebase: &Path, path_to_expected: &Path) {
  let mut all_files_match = true;

  let count_files = |path: &Path| {
    let mut count = 0;
    for dir_entry in fs::read_dir(path).unwrap() {
      if dir_entry.unwrap().path().is_file() {
        count += 1;
      }
    }
    count
  };

  assert_eq!(count_files(path_to_codebase), count_files(path_to_expected));

  for p in fs::read_dir(path_to_codebase).unwrap() {
    let dir_entry = p.unwrap();
    if dir_entry.path().is_file() {
      let path = dir_entry.path();
      let file_name = path.file_name().unwrap();
      let cb_content = read_file(&dir_entry.path().to_path_buf()).unwrap();
      let expected_file_path = path_to_expected.join(file_name);
      let expected_content = read_file(&expected_file_path).unwrap();

      if !eq_without_whitespace(&cb_content, &expected_content) {
        all_files_match = false;
        error!("{}", &cb_content);
      }
    }
  }

  assert!(all_files_match);
}

/// This macro creates a new match test case.
///
/// # Arguments:
/// * test_name: Name of the test (identifier)
/// * relative_path: relative path such that `test-resources/<language>/<relative_path>` leads to a directory containing the folders `input` and `configurations`
/// * expected_number_of_matches: expression returning the expected number of matches
///
/// Usage:
/// ```
/// create_match_tests! {
///  "java",
///  test_a1:  "relative/path_1", 2;
///  test_a2:  "relative/path_2", 3;
/// }
/// ```
macro_rules! create_match_tests {
  ($language: expr,
    $($test_name:ident: $path_to_test: expr,
                        $expected_number_of_matches: expr
                        $(,$kw: ident = $value: expr)* ; )*) => {
    $(
    #[test]
    fn $test_name() {
      initialize();
      let _path= PathBuf::from("test-resources").join($language).join($path_to_test);
      let path_to_codebase = _path.join("input").to_str().unwrap().to_string();
      let path_to_configurations = _path.join("configurations").to_str().unwrap().to_string();
      let piranha_arguments =  piranha_arguments!{
        path_to_codebase = path_to_codebase,
        path_to_configurations = path_to_configurations,
        language= $language.to_string(),
        $(
          $kw = $value,
        )*
      };
      let output_summaries = execute_piranha(&piranha_arguments);
      assert_eq!(
        output_summaries.iter().flat_map(|os| os.matches().iter()).count(),
        $expected_number_of_matches
      );
    }
  )*
  };
}

/// This macro creates a new rewrite test case.
///
/// # Arguments:
/// * language: target language
/// * test_name: Name of the test (identifier)
/// * relative_path: relative path such that `test-resources/<language>/<relative_path>` leads to a directory containing the folders `input`, `expected` and `configurations`
/// * files_changed: expression returning the expected number of files changed after the rewriting
///
/// Usage:
/// ```
/// create_rewrite_tests! {
/// "java".to_string(),
///  test_a1:  "relative/path_1", 2;
///  test_a2:  "relative/path_2", 3;
/// }
/// ```
macro_rules! create_rewrite_tests {
  ($language: expr,
    $($test_name:ident: $path_to_test: expr,
                        $files_changed: expr
                        $(,$kw: ident = $value: expr)* ; )*) => {
    $(
    #[test]
    fn $test_name() {
      initialize();
      let _path= PathBuf::from("test-resources").join($language).join($path_to_test);
      let path_to_codebase = _path.join("input").to_str().unwrap().to_string();
      let path_to_configurations = _path.join("configurations").to_str().unwrap().to_string();
      let path_to_expected = _path.join("expected");

      // Copy the test scenario to temporary directory
      let temp_dir = TempDir::new_in(".", "tmp_test").unwrap();
      let temp_dir_path = &temp_dir.path();
      copy_folder(
        Path::new(&path_to_codebase),
        temp_dir_path,
      );

      let piranha_arguments =  piranha_arguments!{
        path_to_codebase = temp_dir_path.to_str().unwrap().to_string(),
        path_to_configurations = path_to_configurations,
        language= $language.to_string(),
        $(
         $kw = $value,
        )*
      };

      let output_summaries = execute_piranha(&piranha_arguments);
      // Checks if there are any rewrites performed for the file
      assert!(output_summaries.iter().any(|x|!x.rewrites().is_empty()));

      assert_eq!(output_summaries.len(), $files_changed);
      check_result(temp_dir_path, &path_to_expected);

      // Delete temp_dir
      _ = temp_dir.close().unwrap();
    }
  )*
  };
}

/// This macro accepts substitutions as `key` => `value` pairs and transforms it to a `Vec<Vec<String>>`.
///
/// Usage:
/// ```
/// substitutions! {
/// "project" => "Piranha",
/// "language" => "Rust"
/// }
/// ```
///
/// expands to
///
/// ```
/// vec!\[
///      vec!\["project".to_string(), "Piranha".to_string()\]
///      vec!\["language".to_string(), "Rust".to_string()\]
/// \]
/// ```
///
macro_rules! substitutions(
  () =>  { vec![] };
  { $($key:literal => $value:literal),+ } => {
      {
          vec![$(vec![$key.to_string(), $value.to_string()],)+]

      }
   };
);

pub(crate) use create_match_tests;
pub(crate) use create_rewrite_tests;
pub(crate) use substitutions;
