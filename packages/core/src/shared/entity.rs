pub trait Entity: Clone {
    type ID;
    type Props;
    fn id(&self) -> &Self::ID;
    fn props(&self) -> &Self::Props;
}

#[cfg(test)]
mod tests {
    use super::*;

    type ID = i32;

    #[derive(Clone, Debug, PartialEq)]
    struct User {
        id: ID,
        name: String,
    }

    #[derive(Clone, Debug, PartialEq)]
    struct UserEntity(User);

    impl Entity for UserEntity {
        type ID = ID;
        type Props = User;
        fn id(&self) -> &Self::ID {
            &self.0.id
        }
        fn props(&self) -> &Self::Props {
            &self.0
        }
    }

    impl UserEntity {
        fn new(id: &ID, name: impl ToString) -> Self {
            Self(User {
                id: *id,
                name: name.to_string(),
            })
        }
    }

    #[test]
    fn test_entity() {
        let id = 1;
        let user = UserEntity::new(&id, "John Doe");
        assert_eq!(id, 1);
        assert_eq!(user.id(), &id);
        assert_eq!(user.props().name, "John Doe");
    }
}
