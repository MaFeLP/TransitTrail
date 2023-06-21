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
//! This module holds and exports all endpoints of the API.
//!

pub use destinations::*;
pub use locations::*;
pub use routes::*;
pub use service_advisories::*;
pub use stops::*;
pub use street::*;
pub use trip_planner::*;
pub use variants::*;

pub mod destinations;
pub mod locations;
pub mod routes;
pub mod service_advisories;
pub mod stops;
pub mod street;
pub mod trip_planner;
pub mod variants;
