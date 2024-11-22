use std::borrow::{Borrow, Cow};
use std::fmt;
use std::path::Path;
use std::str::FromStr;
use uuid::Uuid;

pub struct DataBase {
    connection: sqlite::Connection,
}

impl From<sqlite::Connection> for DataBase {
    fn from(connection: sqlite::Connection) -> Self {
        Self { connection }
    }
}

impl DataBase {
    pub fn open<T: AsRef<Path>>(path: T) -> sqlite::Result<Self> {
        let query = "
            CREATE TABLE IF NOT EXISTS Users (
                Id varchar(40),
                Name varchar(60),
                Email varchar(60),
                Password varchar(32)
            );
        ";
        let connection = sqlite::open(path)?;
        connection.execute(query)?;
        Ok(Self { connection })
    }

    #[inline(always)]
    pub fn execute<T: AsRef<str>>(&self, statement: T) -> sqlite::Result<()> {
        self.connection.execute(statement)
    }

    #[inline(always)]
    pub fn insert_unchaked<T: AsRef<str>>(
        &self,
        (name, email, password): (T, T, T),
    ) -> sqlite::Result<Uuid> {
        let user = User::from((name.as_ref(), email.as_ref(), password.as_ref()));
        self.connection.execute(user.to_string())?;
        Ok(user.get_id())
    }

    #[inline(always)]
    pub fn insert<T: AsRef<str>>(
        &self,
        (name, email, password): (T, T, T),
    ) -> sqlite::Result<Option<Uuid>> {
        let is_exists = self.find_by_email(email.as_ref())?.is_some();
        if is_exists {
            return Ok(None);
        }
        let user = User::from((name.as_ref(), email.as_ref(), password.as_ref()));
        self.connection.execute(user.to_string())?;
        Ok(Some(user.get_id()))
    }

    #[inline(always)]
    pub fn users(&self) -> sqlite::Result<sqlite::Statement<'_>> {
        self.connection.prepare("SELECT * FROM Users;")
    }

    #[inline]
    pub fn find_by_id(&self, id: Uuid) -> sqlite::Result<Option<User>> {
        let query = format!("SELECT * FROM Users WHERE Id = \"{id}\"");
        let id_str = id.to_string();
        let mut statement = self.connection.prepare(query)?;
        while let Ok(sqlite::State::Row) = statement.next() {
            let id = statement.read::<String, _>("Id")?;
            if id_str != id {
                continue;
            }
            let id = Uuid::from_str(id.as_str()).unwrap();
            let name = statement.read::<String, _>("Name")?;
            let email = statement.read::<String, _>("Email")?;
            let password = statement.read::<String, _>("Password")?;
            return Ok(Some(User {
                id,
                name: name.into(),
                email: email.into(),
                password: password.into(),
            }));
        }
        Ok(None)
    }

    #[inline]
    pub fn find_by_email<T: AsRef<str>>(&self, email: T) -> sqlite::Result<Option<User>> {
        let email = email.as_ref();
        let query = format!("SELECT * FROM Users WHERE Email = \"{email}\"");
        let mut statement = self.connection.prepare(query)?;
        while let Ok(sqlite::State::Row) = statement.next() {
            let tmp_email = statement.read::<String, _>("Email")?;
            if email != tmp_email.as_str() {
                continue;
            }
            let id = Uuid::from_str(statement.read::<String, _>("Id")?.as_str()).unwrap();
            let name = statement.read::<String, _>("Name")?;
            let email = statement.read::<String, _>("Email")?;
            let password = statement.read::<String, _>("Password")?;
            return Ok(Some(User {
                id,
                name: name.into(),
                email: email.into(),
                password: password.into(),
            }));
        }
        Ok(None)
    }

    #[inline]
    pub fn iter_users(&self) -> IterUsers {
        IterUsers::from(self.users().unwrap())
    }
}

pub struct IterUsers<'a> {
    statement: sqlite::Statement<'a>,
}

impl<'a> From<sqlite::Statement<'a>> for IterUsers<'a> {
    fn from(statement: sqlite::Statement<'a>) -> Self {
        Self { statement }
    }
}

impl<'a> Iterator for IterUsers<'a> {
    type Item = User<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Ok(sqlite::State::Row) = self.statement.next() {
            return Some(User {
                id: Uuid::from_str(self.statement.read::<String, _>("Id").ok()?.as_str()).ok()?,
                name: self.statement.read::<String, _>("Name").ok()?.into(),
                email: self.statement.read::<String, _>("Email").ok()?.into(),
                password: self.statement.read::<String, _>("Password").ok()?.into(),
            });
        }
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct User<'a> {
    id: Uuid,
    name: Cow<'a, str>,
    email: Cow<'a, str>,
    password: Cow<'a, str>,
}

impl<'a> User<'a> {
    pub fn new(id: Uuid, name: Cow<'a, str>, email: Cow<'a, str>, password: Cow<'a, str>) -> Self {
        Self {
            id,
            name,
            email,
            password,
        }
    }
    #[inline(always)]
    pub fn get_id(&self) -> Uuid {
        self.id
    }
    #[inline(always)]
    pub fn get_name(&self) -> &str {
        &self.name
    }
    #[inline(always)]
    pub fn get_email(&self) -> &str {
        &self.email
    }
    #[inline(always)]
    pub fn get_password(&self) -> &str {
        &self.password
    }
    #[inline]
    pub fn insert_query(&self) -> String {
        format!(
            "INSERT INTO Users VALUES (\"{}\", \"{}\", \"{}\", \"{}\");",
            self.id, self.name, self.email, self.password
        )
    }
}

impl<'a> From<(&'a str, &'a str, &'a str)> for User<'a> {
    fn from((name, email, password): (&'a str, &'a str, &'a str)) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: Cow::Borrowed(name),
            email: Cow::Borrowed(email),
            password: Cow::Borrowed(password),
        }
    }
}

impl From<(String, String, String)> for User<'_> {
    fn from((name, email, password): (String, String, String)) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: Cow::Owned(name),
            email: Cow::Owned(email),
            password: Cow::Owned(password),
        }
    }
}

impl fmt::Display for User<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ id: \"{}\", name: \"{}\", email: \"{}\", password: \"{}\" }}",
            self.id, self.name, self.email, self.password
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::database::User;

    use super::DataBase;

    #[test]
    fn test_user() {
        let db = DataBase::open("/home/eagle/development/crud-rust/database/dbs.sqlite").unwrap();
        let user = User::from(("Asikul Ali", "aali@gmail.com", "password"));
        // db.insert(user).unwrap();
    }
}
