pub mod extensions;
pub mod factories;
pub mod snitches;

use std::time::Duration;

// use diesel_migrations;
// diesel_migrations::embed_migrations!();

use store::DbConn;

pub fn setup() -> DbConn {
    let conn = conn();
    match std::env::var("TEST_ENV") {
        Ok(_) => (),
        //should run only once
        Err(_) => {
            std::env::set_var("TEST_ENV", "true");
            std::env::set_var(
                "DATABASE_URL",
                std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL env var"),
            );
            //recreate_db(&conn);
            setup_suite();
        }
    };

    clean_db(&conn);

    conn
}

pub fn setup_suite() {
    //enable to debug tests
    common::pretty_env_logger::init_timed();
}

pub fn conn() -> DbConn {
    store::db_conn().expect("db conn")
}

pub fn clean_db(conn: &DbConn) {
    use diesel::RunQueryDsl;
    use store::schema::auth_requests;
    use store::schema::dialogs;
    use store::schema::registrations;
    use store::schema::requests;
    use store::schema::responses;
    //use store::schema::transactions;

    // embedded_migrations::run(conn).expect("running migrations");
    diesel::delete(auth_requests::table)
        .execute(conn)
        .expect("deleting auth_requests");
    diesel::delete(dialogs::table)
        .execute(conn)
        .expect("deleting dialogs");
    diesel::delete(registrations::table)
        .execute(conn)
        .expect("deleting registrations");
    diesel::delete(requests::table)
        .execute(conn)
        .expect("deleting requests");
    diesel::delete(responses::table)
        .execute(conn)
        .expect("deleting responses");
    diesel::delete(responses::table)
        .execute(conn)
        .expect("deleting responses");
}

pub async fn advance_for(duration: Duration) {
    delay_for(Duration::from_millis(1)).await;
    tokio::time::pause();
    tokio::time::advance(duration).await;
    tokio::time::resume();
    delay_for(Duration::from_millis(1)).await;
}

pub async fn delay_for(duration: Duration) {
    tokio::time::sleep(duration).await;
}
