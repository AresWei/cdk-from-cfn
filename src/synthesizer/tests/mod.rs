// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg(test)]
use crate::ir_synthesizer_test;
use cdk_from_cfn_macros::generate_ir_tests;
#[cfg(test)]
use cdk_from_cfn_testing::{ClassTestCase, Stack};

mod class;
mod test;
use class::IrClass;

generate_ir_tests!();

#[test]
fn test_constraint_number() {
    assert_eq!(super::constraint_number("010"), Some(10.0));
    assert_eq!(super::constraint_number("65535.5"), Some(65535.5));
    assert_eq!(super::constraint_number("abc"), None);
    assert_eq!(super::constraint_number("inf"), None);
}
