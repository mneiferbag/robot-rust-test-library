#
# Copyright (c) 2023 Markus Neifer
# Licensed under the MIT License.
# See file LICENSE in project root directory.
#
*** Settings ***
Documentation    Test some keywords from a test library implemented in Rust.
Library          RustyLibrary


*** Test Cases ***
Integer Argument Conversion Test
    ${x} =    Sum As String    ${5}    ${20}
    Should Be Equal    ${x}    25

List Argument Conversion Test
    @{MY_LIST} =    Create List    foo    bar

    ${y} =    Join Strings    ${MY_LIST}
    Should Be Equal    ${y}    foo,bar
