//!
//! Default formatter for datetimes in this project.
//!
//! It has been generated by the following macro, that is not available in version 0.3.15 of crate
//! time, but tauri requires 0.3.15 of time...
//! Use this, as long as tauri is not updated...
//!
//! Formates the dates and times used in this projects easily. This module is
//! automatically generated from the following code, which only is available in
//! time >= 0.3.16 ; tauri however requires time = 0.3.15, so this is why this
//! file exists.
//!
//! ```no_compile
//! time::serde::format_description!(
//!     datetime_formatter,
//!     PrimitiveDateTime,
//!     "[year]-[month]-[day]T[hour]:[minute]:[second]"
//! );
//! ```

use ::time::PrimitiveDateTime as __TimeSerdeType;

const DESCRIPTION: &[::time::format_description::FormatItem<'_>] = &[
    ::time::format_description::FormatItem::Component {
        0: ::time::format_description::Component::Year({
            let mut value = ::time::format_description::modifier::Year::default();
            value.padding = ::time::format_description::modifier::Padding::Zero;
            value.repr = ::time::format_description::modifier::YearRepr::Full;
            value.iso_week_based = false;
            value.sign_is_mandatory = false;
            value
        }),
    },
    ::time::format_description::FormatItem::Literal { 0: b"-" },
    ::time::format_description::FormatItem::Component {
        0: ::time::format_description::Component::Month({
            let mut value = ::time::format_description::modifier::Month::default();
            value.padding = ::time::format_description::modifier::Padding::Zero;
            value.repr = ::time::format_description::modifier::MonthRepr::Numerical;
            value.case_sensitive = true;
            value
        }),
    },
    ::time::format_description::FormatItem::Literal { 0: b"-" },
    ::time::format_description::FormatItem::Component {
        0: ::time::format_description::Component::Day({
            let mut value = ::time::format_description::modifier::Day::default();
            value.padding = ::time::format_description::modifier::Padding::Zero;
            value
        }),
    },
    ::time::format_description::FormatItem::Literal { 0: b"T" },
    ::time::format_description::FormatItem::Component {
        0: ::time::format_description::Component::Hour({
            let mut value = ::time::format_description::modifier::Hour::default();
            value.padding = ::time::format_description::modifier::Padding::Zero;
            value.is_12_hour_clock = false;
            value
        }),
    },
    ::time::format_description::FormatItem::Literal { 0: b":" },
    ::time::format_description::FormatItem::Component {
        0: ::time::format_description::Component::Minute({
            let mut value = ::time::format_description::modifier::Minute::default();
            value.padding = ::time::format_description::modifier::Padding::Zero;
            value
        }),
    },
    ::time::format_description::FormatItem::Literal { 0: b":" },
    ::time::format_description::FormatItem::Component {
        0: ::time::format_description::Component::Second({
            let mut value = ::time::format_description::modifier::Second::default();
            value.padding = ::time::format_description::modifier::Padding::Zero;
            value
        }),
    },
];

struct Visitor;

struct OptionVisitor;

impl<'a> ::serde::de::Visitor<'a> for Visitor {
    type Value = __TimeSerdeType;
    fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            concat!("a(n) `", "PrimitiveDateTime", "` in the format \"{}\"",),
            "[year]-[month]-[day]T[hour]:[minute]:[second]"
        )
    }
    fn visit_str<E: ::serde::de::Error>(self, value: &str) -> Result<__TimeSerdeType, E> {
        __TimeSerdeType::parse(value, &DESCRIPTION).map_err(E::custom)
    }
}

impl<'a> ::serde::de::Visitor<'a> for OptionVisitor {
    type Value = Option<__TimeSerdeType>;
    fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            concat!(
                "an `Option<",
                "PrimitiveDateTime",
                ">` in the format \"{}\"",
            ),
            "[year]-[month]-[day]T[hour]:[minute]:[second]"
        )
    }
    fn visit_some<D: ::serde::de::Deserializer<'a>>(
        self,
        deserializer: D,
    ) -> Result<Option<__TimeSerdeType>, D::Error> {
        deserializer.deserialize_any(Visitor).map(Some)
    }
    fn visit_none<E: ::serde::de::Error>(self) -> Result<Option<__TimeSerdeType>, E> {
        Ok(None)
    }
}

pub fn serialize<S: ::serde::Serializer>(
    datetime: &__TimeSerdeType,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    use ::serde::Serialize;
    datetime
        .format(&DESCRIPTION)
        .map_err(::time::error::Format::into_invalid_serde_value::<S>)?
        .serialize(serializer)
}

pub fn deserialize<'a, D: ::serde::Deserializer<'a>>(
    deserializer: D,
) -> Result<__TimeSerdeType, D::Error> {
    use ::serde::Deserialize;
    deserializer.deserialize_any(Visitor)
}

pub(super) mod option {
    use super::{OptionVisitor, Visitor};
    use super::{__TimeSerdeType, DESCRIPTION};

    pub fn serialize<S: ::serde::Serializer>(
        option: &Option<__TimeSerdeType>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        use ::serde::Serialize;
        option
            .map(|datetime| datetime.format(&DESCRIPTION))
            .transpose()
            .map_err(::time::error::Format::into_invalid_serde_value::<S>)?
            .serialize(serializer)
    }

    pub fn deserialize<'a, D: ::serde::Deserializer<'a>>(
        deserializer: D,
    ) -> Result<Option<__TimeSerdeType>, D::Error> {
        use ::serde::Deserialize;
        deserializer.deserialize_option(OptionVisitor)
    }
}
