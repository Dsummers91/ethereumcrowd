#[cfg(test)]

mod tests {
    use reddit::*;
    use uuid::Uuid;

    #[test]
    fn should_attach_uuid() {
        let person_uuid = Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8");
        let user = "testuser".to_string();
        let new_user =  attach_uuid(NewReddit{person_id: person_uuid.clone().unwrap(), username: user.clone()});
        let complete_user = Reddit{person_id: person_uuid.unwrap(), username: user.clone(), id: Uuid::new_v4()};
        assert_eq!(new_user.person_id, complete_user.person_id);
        assert_eq!(new_user.username, complete_user.username);
    }

    #[test]
    fn should_be_four() {

    }
}
