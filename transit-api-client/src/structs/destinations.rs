// TransitTrail - Navigate Winnipeg Transit with a different style
// Copyright (C) - 2023 Foxx Azalea Pinkerton, Max Fehlinger
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with
// this program. If not, see <https://www.gnu.org/licenses/>.

//!
//! Structures for the [destinations endpoint](crate::TransitClient::destinations)
//!

use serde::Deserialize;

/// An important landmark the buses on the variant will pass
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct Destination {
    /// The key of the important landmark
    pub key: u32,

    /// The name of the important landmark
    pub name: String,
}
