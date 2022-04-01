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

use windmark::Response;

#[windmark::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  windmark::Router::new()
    .set_private_key_file("windmark_comments_private.pem")
    .set_certificate_file("windmark_comments_public.pem")
    .attach(windmark_comments::module)
    .mount(
      "/",
      Box::new(|_| {
        Response::Success(
          "# WINDMARK COMMENTS\nHello, World!\n=> /other To another route!"
            .to_string(),
        )
      }),
    )
    .mount(
      "/other",
      Box::new(|_| {
        Response::Success(
          "# OTHER\nComments also show up on this route!\n=> / Back to the \
           main route!"
            .to_string(),
        )
      }),
    )
    .run()
    .await
}
