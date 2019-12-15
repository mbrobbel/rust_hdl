// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) 2019, Olof Kraigher olof.kraigher@gmail.com

use super::*;

#[test]
fn resolves_names_in_object_decl_init_expressions() {
    let mut builder = LibraryBuilder::new();
    let code = builder.code(
        "libname",
        "
package pkg is
  constant c0 : natural := 0;
  constant c1 : natural := c0;
  constant c2 : natural := missing;
  constant c3 : natural := 1 + missing;
  constant c4 : natural := - missing;
end package;",
    );

    let (root, diagnostics) = builder.get_analyzed_root();
    check_diagnostics(
        diagnostics,
        (1..=3).map(|idx| missing(&code, "missing", idx)).collect(),
    );

    // Goto declaration from declaration
    assert_eq!(
        root.search_reference(code.source(), code.s("c0", 1).end()),
        Some(code.s("c0", 1).pos())
    );

    // Goto declaration from reference
    assert_eq!(
        root.search_reference(code.source(), code.s("c0", 2).end()),
        Some(code.s("c0", 1).pos())
    );
}

#[test]
fn subprogram_parameters_are_visible_in_body() {
    let mut builder = LibraryBuilder::new();
    let code = builder.code(
        "libname",
        "
package pkg is
end package;

package body pkg is
  function foo(c0 : natural) return natural is
     constant c1 : natural := c0;
     constant c2 : natural := missing;
  begin
     return c1;
  end;
end package body;
",
    );
    check_missing(&builder, &code, 1);
}

/// Check that at least the prefix is resolved also for names which are not purely selected names
#[test]
fn resolves_names_for_prefix_of_non_selected() {
    let mut builder = LibraryBuilder::new();
    let code = builder.code(
        "libname",
        "
package pkg is
  type arr_t is array (natural range 0 to 1) of integer_vector(0 to 1);
  constant c0 : integer_vector(0 to 1) := (0,1);
  constant c1 : natural := c0(0);
  constant c2 : natural := missing(0);
  constant c3 : integer_vector(0 to 1) := c0(0 to 1);
  constant c4 : integer_vector(0 to 1) := missing(0 to 1);
  constant c5 : arr_t := (0 => (0,1), 1 => (2, 3));

  -- This was a bug at one point
  constant c6 : natural := c5(0)'length;
  constant c7 : natural := missing(0)'length;

  -- This was also a but at one point
  type rec_t is record
      field : natural;
  end record;
  type rec_arr_t is array (natural range <>) of rec_t;
  constant c8 : rec_arr_t(0 to 0) := (0 => (field => 0));
  constant c9 : natural := c8(0).field;
  constant ca : natural := missing(0).field;
  constant cb : rec_t := (field => 0);
  constant cc : natural := cb.field;
  constant cd : natural := missing.field;

end package;",
    );

    let (root, diagnostics) = builder.get_analyzed_root();
    check_diagnostics(
        diagnostics,
        (1..=5).map(|idx| missing(&code, "missing", idx)).collect(),
    );

    // Goto declaration from reference that is prefix of indexed/function call
    assert_eq!(
        root.search_reference(code.source(), code.s("c0", 2).end()),
        Some(code.s("c0", 1).pos())
    );

    // Goto declaration from reference that is prefix of slice
    assert_eq!(
        root.search_reference(code.source(), code.s("c0", 3).end()),
        Some(code.s("c0", 1).pos())
    );
}

#[test]
fn labels_are_visible() {
    let mut builder = LibraryBuilder::new();
    let code = builder.code(
        "libname",
        "
entity ent is
end entity;

architecture a of ent is
begin
  lab1 : process
     constant name1 : string := lab1'instance_name;
     constant dummy : string := missing'instance_name;
  begin
  end process;

  lab2 : block is
     constant name1 : string := lab1'instance_name;
     constant name2 : string := lab2'instance_name;
     constant dummy : string := missing'instance_name;
  begin
  end block;
end architecture;
",
    );

    let (root, diagnostics) = builder.get_analyzed_root();
    check_diagnostics(
        diagnostics,
        (1..=2).map(|idx| missing(&code, "missing", idx)).collect(),
    );

    for i in 1..=3 {
        assert_eq!(
            root.search_reference(code.source(), code.s("lab1", i).end()),
            Some(code.s("lab1", 1).pos())
        );
    }

    for i in 1..=2 {
        assert_eq!(
            root.search_reference(code.source(), code.s("lab2", i).end()),
            Some(code.s("lab2", 1).pos())
        );
    }
}

#[test]
fn resolves_names_in_discrete_ranges() {
    let mut builder = LibraryBuilder::new();
    let code = builder.code(
        "libname",
        "
package pkg is
  type arr_t is array (natural range missing to missing) of natural;
  type arr2_t is array (missing to missing) of natural;
  type arr3_t is array (missing'range) of natural;
end package;
",
    );

    check_missing(&builder, &code, 5);
}

#[test]
fn search_names_in_discrete_ranges() {
    let mut builder = LibraryBuilder::new();
    let code = builder.code(
        "libname",
        "
package pkg is
  constant decl : natural := 0;
  type arr_t is array (natural range decl to decl) of natural;
  type arr2_t is array (decl to decl) of natural;
  type arr3_t is array (decl'range) of natural;
end package;
",
    );
    check_search_reference(&builder, &code, "decl", 6);
}

#[test]
fn resolves_names_in_subtype_constraints() {
    let mut builder = LibraryBuilder::new();
    let code = builder.code(
        "libname",
        "
package pkg is
  subtype sub1_t is integer_vector(missing to missing);
  subtype sub2_t is integer range missing to missing;

  type rec_t is record
    field : integer_vector;
  end record;

  subtype sub3_t is rec_t(field(missing to missing));

  type uarr_t is array (natural range <>) of integer_vector;
  subtype sub4_t is uarr_t(open)(missing to missing);

end package;
",
    );
    // @TODO check for missing fields in record element constraints
    check_missing(&builder, &code, 8);
}

#[test]
fn search_names_in_subtype_constraints() {
    let mut builder = LibraryBuilder::new();
    let code = builder.code(
        "libname",
        "
package pkg is
  constant decl : natural := 0;
  subtype sub1_t is integer_vector(decl to decl);
  subtype sub2_t is integer range decl to decl;

  type rec_t is record
    field : integer_vector;
  end record;

  subtype sub3_t is rec_t(field(decl to decl));

  type uarr_t is array (natural range <>) of integer_vector;
  subtype sub4_t is uarr_t(open)(decl to decl);

end package;
",
    );
    check_search_reference(&builder, &code, "decl", 9);
}

fn check_missing(builder: &LibraryBuilder, code: &Code, occurences: usize) {
    let diagnostics = builder.analyze();
    check_diagnostics(
        diagnostics,
        (1..=occurences)
            .map(|idx| missing(&code, "missing", idx))
            .collect(),
    );
}

fn check_search_reference(builder: &LibraryBuilder, code: &Code, name: &str, occurences: usize) {
    let (root, diagnostics) = builder.get_analyzed_root();
    check_no_diagnostics(&diagnostics);

    for idx in 1..=occurences {
        assert_eq!(
            root.search_reference(code.source(), code.s(name, idx).end()),
            Some(code.s(name, 1).pos()),
            "{}",
            idx
        );
    }
}