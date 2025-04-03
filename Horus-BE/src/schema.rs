// @generated automatically by Diesel CLI.

diesel::table! {
    departments (id) {
        id -> Int4,
        organization_id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    departments_roles (id) {
        id -> Int4,
        organization_id -> Int4,
        department_id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    organization_configs (id) {
        id -> Int4,
        organization_id -> Int4,
        config_key -> Varchar,
        config_value -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    organizations (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    repo_branches (id) {
        id -> Int4,
        organization_id -> Int4,
        repo_id -> Int4,
        branch_name -> Varchar,
        is_active -> Bool,
        config_json -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    repos (id) {
        id -> Int4,
        organization_id -> Int4,
        name -> Varchar,
        remote_url -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        organization_id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    team_members (id) {
        id -> Int4,
        organization_id -> Int4,
        team_id -> Int4,
        user_id -> Int4,
        joined_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    team_repos (id) {
        id -> Int4,
        organization_id -> Int4,
        team_id -> Int4,
        repo_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    teams (id) {
        id -> Int4,
        organization_id -> Int4,
        department_id -> Int4,
        name -> Varchar,
        name_gh -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user_roles (id) {
        id -> Int4,
        organization_id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        organization_id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(departments -> organizations (organization_id));
diesel::joinable!(departments_roles -> departments (department_id));
diesel::joinable!(departments_roles -> organizations (organization_id));
diesel::joinable!(departments_roles -> roles (role_id));
diesel::joinable!(departments_roles -> users (user_id));
diesel::joinable!(organization_configs -> organizations (organization_id));
diesel::joinable!(repo_branches -> organizations (organization_id));
diesel::joinable!(repo_branches -> repos (repo_id));
diesel::joinable!(repos -> organizations (organization_id));
diesel::joinable!(roles -> organizations (organization_id));
diesel::joinable!(team_members -> organizations (organization_id));
diesel::joinable!(team_members -> teams (team_id));
diesel::joinable!(team_members -> users (user_id));
diesel::joinable!(team_repos -> organizations (organization_id));
diesel::joinable!(team_repos -> repos (repo_id));
diesel::joinable!(team_repos -> teams (team_id));
diesel::joinable!(teams -> departments (department_id));
diesel::joinable!(teams -> organizations (organization_id));
diesel::joinable!(user_roles -> organizations (organization_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));
diesel::joinable!(users -> organizations (organization_id));

diesel::allow_tables_to_appear_in_same_query!(
    departments,
    departments_roles,
    organization_configs,
    organizations,
    repo_branches,
    repos,
    roles,
    team_members,
    team_repos,
    teams,
    user_roles,
    users,
);
