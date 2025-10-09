use anyhow::anyhow;
use enum_fun::{Name, Variants};
use kstring::KString;
use sqlx::{Database, Decode, Encode, Sqlite, Type, prelude::FromRow};

#[derive(FromRow)]
pub struct ArticleVerdict {
    #[sqlx(try_from = "String")]
    pub name: KString,
    pub verdict: Verdict,
}

#[derive(Name, Variants, Copy, Clone)]
#[name(base = "title case")]
pub enum Verdict {
    Yes,
    No,
    Ignore,
}

impl Type<Sqlite> for Verdict {
    fn type_info() -> <Sqlite as Database>::TypeInfo {
        <&str as Type<Sqlite>>::type_info()
    }
}

impl<'r> Decode<'r, Sqlite> for Verdict {
    fn decode(value: <Sqlite as Database>::ValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        match <&str as Decode<Sqlite>>::decode(value)? {
            "y" => Ok(Self::Yes),
            "n" => Ok(Self::No),
            "i" => Ok(Self::Ignore),
            v => Err(anyhow!("invalid verdict: {v}").into()),
        }
    }
}

impl<'q> Encode<'q, Sqlite> for Verdict {
    fn encode_by_ref(
        &self,
        buf: &mut <Sqlite as Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        <&str as Encode<Sqlite>>::encode_by_ref(
            &match self {
                Self::Yes => "y",
                Self::No => "n",
                Self::Ignore => "i",
            },
            buf,
        )
    }
}
