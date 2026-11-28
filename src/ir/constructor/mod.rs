// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
use crate::{parser::parameters::Parameter, Hasher};
use indexmap::IndexMap;
use voca_rs::case::camel_case;

#[derive(Debug, Default)]
pub struct Constructor {
    pub inputs: Vec<ConstructorParameter>,
}

impl Constructor {
    pub(super) fn from(parse_tree: IndexMap<String, Parameter, Hasher>) -> Self {
        Self {
            inputs: parse_tree
                .into_iter()
                .map(|(name, param)| ConstructorParameter {
                    name: camel_case(&name),
                    logical_id: name,
                    description: param.description,
                    constructor_type: match param.parameter_type {
                        crate::parser::parameters::ParameterType::String => {
                            if param.allowed_values.is_some() {
                                let values = param.allowed_values.clone().unwrap();
                                if values.len() == 2 {
                                    if (values[0].to_lowercase() == "true"
                                        && values[1].to_lowercase() == "false")
                                        || (values[0].to_lowercase() == "false"
                                            && values[1].to_lowercase() == "true")
                                    {
                                        crate::parser::parameters::ParameterType::Bool.to_string()
                                    } else {
                                        param.parameter_type.to_string()
                                    }
                                } else {
                                    param.parameter_type.to_string()
                                }
                            } else {
                                param.parameter_type.to_string()
                            }
                        }
                        _ => param.parameter_type.to_string(),
                    },
                    default_value: param.default,
                    allowed_values: param.allowed_values,
                    allowed_pattern: param.allowed_pattern,
                    constraint_description: param.constraint_description,
                    min_length: param.min_length,
                    max_length: param.max_length,
                    min_value: param.min_value,
                    max_value: param.max_value,
                    no_echo: param.no_echo,
                })
                .collect(),
        }
    }
}

#[derive(Debug, Default)]
pub struct ConstructorParameter {
    pub name: String,
    pub logical_id: String,
    pub description: Option<String>,
    pub constructor_type: String,
    pub default_value: Option<String>,
    pub allowed_values: Option<Vec<String>>,
    pub allowed_pattern: Option<String>,
    pub constraint_description: Option<String>,
    pub min_length: Option<String>,
    pub max_length: Option<String>,
    pub min_value: Option<String>,
    pub max_value: Option<String>,
    pub no_echo: Option<String>,
}

#[cfg(test)]
mod tests;
