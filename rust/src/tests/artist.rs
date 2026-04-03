#[cfg(test)]
mod tests {
    use rusqlite::Connection;
    use uuid::Uuid;

    use crate::services::artist_service;

    fn create_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "
            CREATE TABLE artists (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                image_path TEXT
            );
            ",
        )
        .unwrap();
        conn
    }

    #[test]
    fn test_create_artist_success() {
        let db = create_test_db();
        let result = artist_service::create_artist(&db, "Test Artist");
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_artist_empty_name() {
        let db = create_test_db();
        let result = artist_service::create_artist(&db, "");
        assert!(result.is_err());
    }

    #[test]
    fn test_create_artist_whitespace_name() {
        let db = create_test_db();
        let result = artist_service::create_artist(&db, "   ");
        assert!(result.is_err());
    }

    #[test]
    fn test_create_artist_duplicate_name() {
        let db = create_test_db();
        artist_service::create_artist(&db, "Duplicate Artist").unwrap();
        let result = artist_service::create_artist(&db, "Duplicate Artist");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_all_artists_empty() {
        let db = create_test_db();
        let result = artist_service::get_all_artists(&db);
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_get_all_artists_with_data() {
        let db = create_test_db();
        artist_service::create_artist(&db, "Artist 1").unwrap();
        artist_service::create_artist(&db, "Artist 2").unwrap();

        let result = artist_service::get_all_artists(&db);
        assert!(result.is_ok());
        let artists = result.unwrap();
        assert_eq!(artists.len(), 2);
    }

    #[test]
    fn test_get_artist_by_id_not_found() {
        let db = create_test_db();
        let result = artist_service::get_artist(&db, &Uuid::new_v4().to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_get_artist_by_id_success() {
        let db = create_test_db();
        artist_service::create_artist(&db, "Test Artist").unwrap();

        let artists = artist_service::get_all_artists(&db).unwrap();
        let id = &artists[0].id;

        let result = artist_service::get_artist(&db, id);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "Test Artist");
    }

    #[test]
    fn test_update_artist_success() {
        let db = create_test_db();
        artist_service::create_artist(&db, "Old Name").unwrap();

        let artists = artist_service::get_all_artists(&db).unwrap();
        let id = &artists[0].id;

        let result = artist_service::update_artist(&db, id, "New Name");
        assert!(result.is_ok());

        let updated = artist_service::get_artist(&db, id).unwrap();
        assert_eq!(updated.name, "New Name");
    }

    #[test]
    fn test_update_artist_empty_name() {
        let db = create_test_db();
        artist_service::create_artist(&db, "Test Artist").unwrap();

        let artists = artist_service::get_all_artists(&db).unwrap();
        let id = &artists[0].id;

        let result = artist_service::update_artist(&db, id, "");
        assert!(result.is_err());
    }

    #[test]
    fn test_update_artist_not_found() {
        let db = create_test_db();
        let result = artist_service::update_artist(&db, &Uuid::new_v4().to_string(), "New Name");
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_artist_success() {
        let db = create_test_db();
        artist_service::create_artist(&db, "To Delete").unwrap();

        let artists = artist_service::get_all_artists(&db).unwrap();
        let id = &artists[0].id;

        let result = artist_service::delete_artist(&db, id);
        assert!(result.is_ok());

        let all = artist_service::get_all_artists(&db).unwrap();
        assert!(all.is_empty());
    }

    #[test]
    fn test_delete_artist_not_found() {
        let db = create_test_db();
        let result = artist_service::delete_artist(&db, &Uuid::new_v4().to_string());
        assert!(result.is_err());
    }
}
