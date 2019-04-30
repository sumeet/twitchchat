use super::*;

/// Simple channel wrapper.
///
/// This ensures the twitch channels align with IRC naming scheme.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Channel(String);

impl Channel {
    pub(crate) fn validate<C>(channel: C) -> Result<Channel, Error>
    where
        C: Into<Channel>,
    {
        let channel = channel.into();
        if channel.0.is_empty() {
            return Err(Error::EmptyChannelName);
        }
        Ok(channel)
    }
}

// TODO these are atrocious

impl From<&Channel> for Channel {
    fn from(ch: &Channel) -> Self {
        ch.clone()
    }
}

impl<T> From<T> for Channel
where
    T: Into<String>,
{
    fn from(name: T) -> Self {
        let name = name.into();
        if name.is_empty() {
            return Self("".into());
        }

        let name = name.to_lowercase();
        let name = if !name.starts_with('#') {
            ["#", name.as_str()].concat()
        } else {
            name.to_string()
        };

        Self(name)
    }
}

impl std::fmt::Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<str> for Channel {
    fn eq(&self, other: &str) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<&str> for Channel {
    fn eq(&self, other: &&str) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<String> for Channel {
    fn eq(&self, other: &String) -> bool {
        self.0.eq(other)
    }
}

impl std::ops::Deref for Channel {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn good_channel() {
        assert_eq!(Channel::validate("museun").unwrap().0, "#museun");
    }

    #[test]
    fn equals() {
        let s = "foobar";
        let ch: Channel = s.into();
        assert!(ch == "#foobar");
        assert!(ch == "#foobar".to_string());
    }

    #[test]
    fn bad_channel() {
        let err = Channel::validate("").unwrap_err();
        if let Error::EmptyChannelName = err {
        } else {
            panic!("wrong error: {}", err)
        }
    }

    #[test]
    fn into_channel() {
        let s = String::from("museun");

        let channels: Vec<Channel> = vec![
            s.as_str().into(),
            s.clone().into(),
            s.into(),
            "museun".into(),
            String::from("museun").into(),
        ];

        for name in channels {
            assert_eq!(*name, "#museun");
        }
    }
}
