use std::str::FromStr;

/// Hidden Answer
pub struct HiddenAnswer;

impl HiddenAnswer {
    /// Verify a hidden answer
    pub fn verify(input: HiddenAnswerInput) -> bool {
        let message = input.message;
        let username = input.username;
        let password = input.password;

        let input_str = format!("{message} {username} {password}");
        let input_hash = blake3::Hash::from_str(input.hash);
        let hash = blake3::hash(input_str.as_bytes());

        match input_hash {
            Ok(input_hash) => input_hash == hash,
            Err(_) => {
                panic!("could not hash input");
            }
        }
    }
}

/// Hidden Answer Input
pub struct HiddenAnswerInput<'a> {
    /// Hidden message
    message: &'a str,
    /// Username/handle
    username: &'a str,
    /// Secret onetime password
    password: &'a str,
    /// HiddenAnswer hash
    hash: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify() {
        let input = HiddenAnswerInput {
            message: "my message",
            username: "cy6erlion",
            password: "firekey",
            hash: "2c0eeeda2328e4a1b7209337038a0f3bc388606f991da6c0ea09baaebb9e6bd9",
        };

        let verification = HiddenAnswer::verify(input);
        assert!(verification);

        let with_wrong_input = HiddenAnswerInput {
            message: "my message",
            username: "wrong_username",
            password: "firekey",
            hash: "2c0eeeda2328e4a1b7209337038a0f3bc388606f991da6c0ea09baaebb9e6bd9",
        };
        let verification = HiddenAnswer::verify(with_wrong_input);
        assert!(!verification);
    }
}
