#
# Copyright (c) 2023 Markus Neifer
# Licensed under the MIT License.
# See file LICENSE in project root directory.
#
*** Settings ***
Documentation    Test some keywords coming from a test library implemented in Rust.
Library          robot_rust_test_library


*** Test Cases ***
Keyword Argument Conversion Test
    ${x} =    Sum As String    ${5}    ${20}
    Log To Console    We got ${x}

