// This file is part of Windmark Comments <https://github.com/gemrest/windmark-comments>.
// Copyright (C) 2022-2022 Fuwn <contact@fuwn.me>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.
//
// Copyright (C) 2022-2022 Fuwn <contact@fuwn.me>
// SPDX-License-Identifier: GPL-3.0-only

//! # Windmark Comments
//!
//! [![github.com](https://github.com/gemrest/windmark-comments/actions/workflows/check.yaml/badge.svg?branch=main)](https://github.com/gemrest/windmark-comments/actions/workflows/check.yaml)
//!
//! A comment engine module for Windmark.
//!
//! ## Usage
//!
//! ### Add Windmark Comments as a dependency
//!
//! ```toml
//! # Cargo.toml
//!
//! [dependencies]
//! windmark-comments = "0.1.0"
//! ```
//!
//! ### Attach Windmark Comments as a module
//!
//! ```rust
//! // src/main.rs
//!
//! use windmark::Response;
//!
//! #[windmark::main]
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!   windmark::Router::new()
//!     .set_private_key_file("windmark_comments_private.pem")
//!     .set_certificate_file("windmark_comments_public.pem")
//!     .mount("/", Box::new(|_| Response::Success("Hello, World!".into())))
//!     .set_error_handler(Box::new(|_| {
//!       Response::PermanentFailure("This route does not exist!".into())
//!     }))
//!     // Attach Windmark Comments
//!     .attach(windmark_comments::module)
//!     .run()
//!     .await
//! }
//! ```
//!
//! ## Examples
//!
//! Examples can be found within the
//! [`examples`](https://github.com/gemrest/windmark-comments/tree/main/examples)
//! directory.
//!
//! ## License
//!
//! This project is licensed with the
//! [GNU General Public License v3.0](https://github.com/gemrest/windmark-comments/blob/main/LICENSE).

#![feature(once_cell)]
#![deny(
  warnings,
  nonstandard_style,
  unused,
  future_incompatible,
  rust_2018_idioms,
  unsafe_code
)]
#![deny(clippy::all, clippy::nursery, clippy::pedantic)]
#![recursion_limit = "128"]

use std::{
  lazy::SyncLazy,
  sync::{
    atomic::{AtomicUsize, Ordering},
    Mutex,
  },
};

use chrono::{DateTime, Utc};
use windmark::Response;

static COMMENTS: SyncLazy<Mutex<Comments>> =
  SyncLazy::new(|| Mutex::new(vec![]));
static MAX_COMMENTS: AtomicUsize = AtomicUsize::new(500);

/// Keeps track of comments, storing date/ time and the comment.
pub type Comments = Vec<(DateTime<Utc>, String)>;

/// Get a `Vec` of posted `Comments`.
///
/// # Errors
///
/// May produce an error if the comments could not be loaded.
pub fn get_comments() -> Result<Comments, Box<dyn std::error::Error>> {
  Ok((*COMMENTS.lock()?).clone())
}

/// Set the max amount of comments.
pub fn set_max_comments(max_comments: usize) {
  MAX_COMMENTS.store(max_comments, Ordering::SeqCst);
}

/// The Windmark Comments module.
///
/// # Mounts
///
/// - /api/post-comment
/// - Overrides footer
///
/// # Examples
///
/// ```rust
/// windmark::Router::new().attach(windmark_comments::module); 
/// ```
pub fn module(router: &mut windmark::Router) {
  router.mount(
    "/api/post-comment",
    Box::new(|context| {
      context.url.query().map_or_else(
        || Response::Input("What comment would you like to post?".to_string()),
        |query| {
          if let Ok(comment) = urlencoding::decode(query) {
            if let Ok(mut comments) = COMMENTS.lock() {
              if comments.len() >= MAX_COMMENTS.load(Ordering::SeqCst) {
                Response::Success(format!(
                  "Your comment, \"{}\", could not be posted as the instance \
                   comment limit ({}) has been met...",
                  comment,
                  MAX_COMMENTS.load(Ordering::SeqCst)
                ))
              } else {
                (*comments).push((Utc::now(), comment.to_string()));

                Response::Success(format!(
                  "Your comment, \"{}\", has been posted!",
                  comment
                ))
              }
            } else {
              Response::Success(format!(
                "Your comment, \"{}\", could not been posted...",
                comment
              ))
            }
          } else {
            Response::Success(
              "Your comment was unable to be posted...".to_string(),
            )
          }
        },
      )
    }),
  );

  router.set_footer(Box::new(|_| {
    format!(
      "## COMMENTS ({}/{})\n=> /api/post-comment Make a comment!\n{}",
      if let Ok(comments) = COMMENTS.lock() {
        (*comments).len().to_string()
      } else {
        "?".to_string()
      },
      MAX_COMMENTS.load(Ordering::SeqCst),
      if let Ok(comments) = COMMENTS.lock() {
        let comments = (*comments)
          .iter()
          .map(|c| format!("{}: {}", c.0.format("%Y. %B. %e. %T"), c.1))
          .collect::<Vec<_>>()
          .join("\n");

        if comments.is_empty() {
          "There are currently no comments!".to_string()
        } else {
          comments
        }
      } else {
        "Comments could not be loaded...".to_string()
      }
    )
  }));
}
