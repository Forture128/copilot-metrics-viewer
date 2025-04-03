use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use diesel::prelude::*;
use horus_be::{
    config::AppConfig,
    infrastructure::db::create_pool,
    schema::{organizations, roles, user_roles, users},
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load application config
    let config = AppConfig::load().expect("Failed to load config");

    // Get admin credentials from environment or use defaults
    let username = env::var("ADMIN_USERNAME").unwrap_or_else(|_| "admin".to_string());
    let email = env::var("ADMIN_EMAIL").unwrap_or_else(|_| "admin@example.com".to_string());
    let password = env::var("ADMIN_PASSWORD").unwrap_or_else(|_| "admin123".to_string());

    println!("Seeding admin user: {}", username);

    // Create database pool
    let db_pool = create_pool(
        &config.database.url,
        config.database.max_connections,
        config.database.timeout_seconds,
    )
    .await
    .expect("Failed to create database pool");

    // Get a connection from the pool
    let mut conn = db_pool.get().await.expect("Failed to get DB connection");

    // Hash the password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string();

    // 1. Check if System organization exists
    let system_org_exists = diesel_async::RunQueryDsl::get_result::<bool>(
        diesel::dsl::select(diesel::dsl::exists(
            organizations::table.filter(organizations::name.eq("System")),
        )),
        &mut conn,
    )
    .await
    .expect("Failed to check if System organization exists");

    // Create or get System organization
    let org_id = if system_org_exists {
        // Get existing organization ID
        diesel_async::RunQueryDsl::get_result::<i32>(
            organizations::table
                .filter(organizations::name.eq("System"))
                .select(organizations::id),
            &mut conn,
        )
        .await
        .expect("Failed to get System organization ID")
    } else {
        // Create new organization
        diesel_async::RunQueryDsl::get_result::<i32>(
            diesel::insert_into(organizations::table)
                .values(organizations::name.eq("System"))
                .returning(organizations::id),
            &mut conn,
        )
        .await
        .expect("Failed to create System organization")
    };

    println!(
        "System organization created or retrieved with ID: {}",
        org_id
    );

    // 2. Check if super_admin role exists for this organization
    let role_exists = diesel_async::RunQueryDsl::get_result::<bool>(
        diesel::dsl::select(diesel::dsl::exists(
            roles::table
                .filter(roles::name.eq("super_admin"))
                .filter(roles::organization_id.eq(org_id)),
        )),
        &mut conn,
    )
    .await
    .expect("Failed to check if super_admin role exists");

    // Create or get super_admin role
    let role_id = if role_exists {
        // Get existing role ID
        diesel_async::RunQueryDsl::get_result::<i32>(
            roles::table
                .filter(roles::name.eq("super_admin"))
                .filter(roles::organization_id.eq(org_id))
                .select(roles::id),
            &mut conn,
        )
        .await
        .expect("Failed to get super_admin role ID")
    } else {
        // Create new role
        diesel_async::RunQueryDsl::get_result::<i32>(
            diesel::insert_into(roles::table)
                .values((
                    roles::name.eq("super_admin"),
                    roles::description.eq("Super Administrator with full system access"),
                    roles::organization_id.eq(org_id),
                ))
                .returning(roles::id),
            &mut conn,
        )
        .await
        .expect("Failed to create super_admin role")
    };

    println!("Super admin role created or retrieved with ID: {}", role_id);

    // 3. Check if user exists
    let user_exists = diesel_async::RunQueryDsl::get_result::<bool>(
        diesel::dsl::select(diesel::dsl::exists(
            users::table.filter(users::username.eq(&username)),
        )),
        &mut conn,
    )
    .await
    .expect("Failed to check if user exists");

    // Create or update user
    let user_id = if user_exists {
        // Get existing user ID
        let user_id = diesel_async::RunQueryDsl::get_result::<i32>(
            users::table
                .filter(users::username.eq(&username))
                .select(users::id),
            &mut conn,
        )
        .await
        .expect("Failed to get user ID");

        // Update user
        diesel_async::RunQueryDsl::execute(
            diesel::update(users::table.filter(users::id.eq(user_id))).set((
                users::email.eq(&email),
                users::password.eq(&password_hash),
                users::organization_id.eq(org_id),
            )),
            &mut conn,
        )
        .await
        .expect("Failed to update user");

        user_id
    } else {
        // Create new user
        diesel_async::RunQueryDsl::get_result::<i32>(
            diesel::insert_into(users::table)
                .values((
                    users::username.eq(&username),
                    users::email.eq(&email),
                    users::password.eq(&password_hash),
                    users::organization_id.eq(org_id),
                ))
                .returning(users::id),
            &mut conn,
        )
        .await
        .expect("Failed to create admin user")
    };

    println!("Admin user created or updated with ID: {}", user_id);

    // 4. Check if role is already assigned
    let role_assigned = diesel_async::RunQueryDsl::get_result::<bool>(
        diesel::dsl::select(diesel::dsl::exists(
            user_roles::table
                .filter(user_roles::user_id.eq(user_id))
                .filter(user_roles::role_id.eq(role_id)),
        )),
        &mut conn,
    )
    .await
    .expect("Failed to check if role is assigned");

    // Assign role if not already assigned
    if !role_assigned {
        diesel_async::RunQueryDsl::execute(
            diesel::insert_into(user_roles::table).values((
                user_roles::user_id.eq(user_id),
                user_roles::role_id.eq(role_id),
                user_roles::organization_id.eq(org_id),
            )),
            &mut conn,
        )
        .await
        .expect("Failed to assign super_admin role to admin");
        println!("Super admin role assigned to user");
    } else {
        println!("Super admin role was already assigned to user");
    }

    println!("Admin user seeded successfully!");
    println!("Username: {}", username);
    println!("Password: {}", password);

    Ok(())
}
