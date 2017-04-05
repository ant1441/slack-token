//! A `Token` tracks the current "owner" of something, and is unique per channel.
#![allow(dead_code)]

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, RwLock};
use std::fmt;

use slack::{TeamId, ChannelId};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct User {
    user_id: String,
    user_name: String,
}

impl User {
    pub fn new(user_id: String, user_name: String) -> User {
        User {
            user_id: user_id,
            user_name: user_name,
        }
    }

    pub fn as_slack_str(&self) -> String {
        format!("<@{}|{}>", self.user_id, self.user_name)
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.user_name)
    }
}


#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    users: VecDeque<User>,
}
pub type TokenRef = Arc<RwLock<Token>>;
pub type TokensType = Mutex<HashMap<(TeamId, ChannelId), TokenRef>>;

pub struct Tokens(pub TokensType);

impl Tokens {
    pub fn new() -> Tokens {
        Tokens(Mutex::new(HashMap::new()))
    }
}

impl Token {
    /// Constructs a new, empty `Vec<T>`.
    ///
    /// The vector will not allocate until elements are pushed onto it.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(unused_mut)]
    /// let mut vec: Vec<i32> = Vec::new();
    /// ```
    pub fn new() -> Token {
        let users = VecDeque::new();
        Token { users: users }
    }

    pub fn len(&self) -> usize {
        self.users.len()
    }

    pub fn get(&mut self, user: User) -> Result<(), &'static str> {
        // We want the queue to be unique
        if self.users.iter().position(|u| *u == user).is_none() {
            Ok(self.users.push_back(user))
        } else {
            Err("You are already in the queue!")
        }
    }

    pub fn drop(&mut self, user: &User) -> Result<(), &'static str> {
        if let Some(_) = self.users.iter().position(|u| u == user) {
            Ok((&mut self.users).retain(|u| u != user))
        } else {
            Err("You are not in the queue!")
        }
    }

    pub fn step_back(&mut self, user: &User) -> Result<(), &'static str> {
        if let Some(pos) = self.users.iter().position(|u| u == user) {
            // Are we at the end of the queue?
            if pos >= self.len() - 1 {
                Err("You are at the end of the queue!")
            } else {
                Ok(self.users.swap(pos, pos + 1))
            }
        } else {
            Err("You are not in the queue!")
        }
    }

    pub fn to_front(&mut self, user: &User) -> Result<(), &'static str> {
        if let Some(pos) = self.users.iter().position(|u| u == user) {
            // Are we already at the front of the queue?
            if pos == 0 {
                Err("You are already holding the token!")
            } else if pos == 1 {
                Err("You are already at the start of the queue!")
            } else {
                Ok(self.users.swap(pos, 1))
            }
        } else {
            Err("You are not in the queue!")
        }
    }

    pub fn steal(&mut self, user: &User) -> Result<User, &'static str> {
        if let Some(pos) = self.users.iter().position(|u| u == user) {
            // Are we already at the front of the queue?
            if pos == 0 {
                Err("You are already holding the token!")
            } else {
                self.users.swap(pos, 0);
                // We know there is an item here, so unwrap is safe
                Ok(self.users.remove(pos).unwrap())
            }
        } else {
            Err("You are not in the queue!")
        }
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item=&'a User> {
        (&self.users).iter()
    }

    pub fn list_user_name(&self) -> Vec<&str> {
        (&self.users).iter().map(|u| u.user_name.as_str()).collect()
    }

    /// Test if the given user is holding the token
    pub fn is_holding(&self, user: &User) -> bool {
        self.users.front() == Some(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let mut t = Token::new();
        let u = User::new("id".to_string(), "name".to_string());
        t.get(u.clone()).unwrap();

        assert!(t.is_holding(&u));
    }

    #[test]
    fn test_drop() {
        let mut t = Token::new();
        let u = User::new("id".to_string(), "name".to_string());
        t.get(u.clone()).unwrap();
        assert!(t.is_holding(&u));
        t.drop(&u).unwrap();
        assert!(!t.is_holding(&u));
    }

    #[test]
    fn test_len() {
        let mut t = Token::new();
        let u0 = User::new("id0".to_string(), "name0".to_string());
        let u1 = User::new("id1".to_string(), "name1".to_string());
        let u2 = User::new("id2".to_string(), "name2".to_string());
        let u3 = User::new("id3".to_string(), "name3".to_string());
        t.get(u0.clone()).unwrap();
        assert_eq!(t.len(), 1);
        t.get(u1.clone()).unwrap();
        assert_eq!(t.len(), 2);
        t.get(u2.clone()).unwrap();
        assert_eq!(t.len(), 3);
        t.get(u3.clone()).unwrap();
        assert_eq!(t.len(), 4);

        assert!(t.is_holding(&u0));
    }

    #[test]
    fn test_list_user_name() {
        let mut t = Token::new();
        let u0 = User::new("id0".to_string(), "name0".to_string());
        let u1 = User::new("id1".to_string(), "name1".to_string());
        let u2 = User::new("id2".to_string(), "name2".to_string());
        let u3 = User::new("id3".to_string(), "name3".to_string());
        t.get(u0).unwrap();
        t.get(u1).unwrap();
        t.get(u2).unwrap();
        t.get(u3).unwrap();

        assert_eq!(t.list_user_name(), vec!["name0", "name1", "name2", "name3"]);
    }

    #[test]
    fn test_step_back() {
        let mut t = Token::new();
        let u0 = User::new("id0".to_string(), "name0".to_string());
        let u1 = User::new("id1".to_string(), "name1".to_string());
        let u2 = User::new("id2".to_string(), "name2".to_string());
        let u3 = User::new("id3".to_string(), "name3".to_string());
        t.get(u0.clone()).unwrap();
        t.get(u1.clone()).unwrap();
        t.get(u2.clone()).unwrap();
        t.get(u3.clone()).unwrap();

        t.step_back(&u0).unwrap();
        assert_eq!(t.list_user_name(), vec!["name1", "name0", "name2", "name3"]);
        t.step_back(&u0).unwrap();
        assert_eq!(t.list_user_name(), vec!["name1", "name2", "name0", "name3"]);
        t.step_back(&u0).unwrap();
        assert_eq!(t.list_user_name(), vec!["name1", "name2", "name3", "name0"]);

        assert!(t.step_back(&u0).is_err());
    }

    #[test]
    fn test_to_front() {
        let mut t = Token::new();
        let u0 = User::new("id0".to_string(), "name0".to_string());
        let u1 = User::new("id1".to_string(), "name1".to_string());
        let u2 = User::new("id2".to_string(), "name2".to_string());
        let u3 = User::new("id3".to_string(), "name3".to_string());
        t.get(u0.clone()).unwrap();
        t.get(u1.clone()).unwrap();
        t.get(u2.clone()).unwrap();
        t.get(u3.clone()).unwrap();

        t.to_front(&u2).unwrap();
        assert_eq!(t.list_user_name(), vec!["name0", "name2", "name1", "name3"]);

        assert!(t.to_front(&u2).is_err());
    }

    #[test]
    fn test_steal() {
        let mut t = Token::new();
        let u0 = User::new("id0".to_string(), "name0".to_string());
        let u1 = User::new("id1".to_string(), "name1".to_string());
        let u2 = User::new("id2".to_string(), "name2".to_string());
        let u3 = User::new("id3".to_string(), "name3".to_string());
        t.get(u0.clone()).unwrap();
        t.get(u1.clone()).unwrap();
        t.get(u2.clone()).unwrap();
        t.get(u3.clone()).unwrap();

        t.steal(&u2).unwrap();
        assert_eq!(t.list_user_name(), vec!["name2", "name1", "name3"]);
        t.steal(&u3).unwrap();
        assert_eq!(t.list_user_name(), vec!["name3", "name1"]);

        assert!(t.steal(&u2).is_err());
    }

    #[test]
    fn test_is_holding() {
        let mut t = Token::new();
        let u0 = User::new("id0".to_string(), "name0".to_string());
        let u1 = User::new("id1".to_string(), "name1".to_string());
        let u2 = User::new("id2".to_string(), "name2".to_string());
        let u3 = User::new("id3".to_string(), "name3".to_string());
        t.get(u0.clone()).unwrap();
        t.get(u1.clone()).unwrap();
        t.get(u2.clone()).unwrap();
        t.get(u3.clone()).unwrap();

        assert!(t.is_holding(&u0))
    }
}
