// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
use super::*;

#[test]
fn test_parameter_type_display() {
    assert_eq!(ParameterType::String.to_string(), "String");
    assert_eq!(ParameterType::Number.to_string(), "Number");
    assert_eq!(ParameterType::ListOfNumbers.to_string(), "List<Number>");
    assert_eq!(
        ParameterType::CommaDelimitedList.to_string(),
        "CommaDelimitedList"
    );
    assert_eq!(
        ParameterType::Other("CustomType".to_string()).to_string(),
        "CustomType"
    );
}

#[test]
fn test_parameter_constraints_accept_numbers() {
    let param: Parameter = serde_yaml::from_str(
        r#"{ "Type": "Number", "MinValue": 1024, "MaxValue": 65535.5, "MinLength": 1, "MaxLength": "255" }"#,
    )
    .unwrap();

    assert_eq!(param.min_value.as_deref(), Some("1024"));
    assert_eq!(param.max_value.as_deref(), Some("65535.5"));
    assert_eq!(param.min_length.as_deref(), Some("1"));
    assert_eq!(param.max_length.as_deref(), Some("255"));
}
