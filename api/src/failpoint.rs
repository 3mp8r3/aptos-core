// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

#![allow(unused_imports)]

use anyhow::{format_err, Result};
use aptos_api_types::{AptosError, Error};

use crate::poem_backend::InternalError;
use poem_openapi::payload::Json;

#[allow(unused_variables)]
#[inline]
pub fn fail_point(name: &str) -> Result<(), Error> {
    Ok(fail::fail_point!(format!("api::{}", name).as_str(), |_| {
        Err(format_err!("unexpected internal error for {}", name).into())
    }))
}

#[allow(unused_variables)]
#[inline]
pub fn fail_point_poem<E: InternalError>(name: &str) -> Result<(), E> {
    Ok(fail::fail_point!(format!("api::{}", name).as_str(), |_| {
        Err(E::internal_str(&format!(
            "unexpected internal error for {}",
            name
        )))
    }))
}
