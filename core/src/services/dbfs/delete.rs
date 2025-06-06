// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use std::sync::Arc;

use http::StatusCode;

use super::core::*;
use super::error::parse_error;
use crate::raw::*;
use crate::*;

pub struct DbfsDeleter {
    core: Arc<DbfsCore>,
}

impl DbfsDeleter {
    pub fn new(core: Arc<DbfsCore>) -> Self {
        Self { core }
    }
}

impl oio::OneShotDelete for DbfsDeleter {
    async fn delete_once(&self, path: String, _: OpDelete) -> Result<()> {
        let resp = self.core.dbfs_delete(&path).await?;

        let status = resp.status();

        match status {
            // NOTE: Server will return 200 even if the path doesn't exist.
            StatusCode::OK => Ok(()),
            _ => Err(parse_error(resp)),
        }
    }
}
