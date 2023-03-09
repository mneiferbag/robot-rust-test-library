#
# Copyright (c) 2023 Markus Neifer
# Licensed under the MIT License.
# See file LICENSE in project root directory.
#
*** Settings ***
Documentation    Test some keywords coming from a test library implemented in Rust.
Library          RustyLibrary


*** Test Cases ***
Keyword Argument Conversion Test
    [Documentation]    Testing integer and string datatypes.

    ${x} =    Sum As String    ${5}    ${20}
    Should Be Equal    ${x}    25
