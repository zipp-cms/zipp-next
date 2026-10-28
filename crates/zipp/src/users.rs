//! The users of the studio, managed by admins. Roles are code, see
//! [`Role`](crate::Role); a user without one is an admin.

use axum::extract::{Path, State};
use axum::routing::{get, put};
use axum::{Json, Router};
use rusqlite::params;
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::auth::{Authed, add_user};
use crate::error::{Error, Result};

#[derive(Debug, Serialize)]
struct User {
	id: i64,
	email: String,
	role: Option<String>,
}

async fn list_route(
	State(state): State<AppState>,
	auth: Authed,
) -> Result<Json<Vec<User>>> {
	auth.check_admin()?;
	let conn = state.db.conn();
	let mut stmt =
		conn.prepare("SELECT id, email, role FROM users ORDER BY email")?;
	let users = stmt
		.query_map([], |r| {
			Ok(User {
				id: r.get(0)?,
				email: r.get(1)?,
				role: r.get(2)?,
			})
		})?
		.collect::<rusqlite::Result<_>>()?;
	Ok(Json(users))
}

#[derive(Debug, Deserialize)]
struct UserReq {
	email: String,
	password: String,
	role: Option<String>,
}

fn check_role(state: &AppState, role: Option<&str>) -> Result<()> {
	match role {
		Some(r) if state.schema.get_role(r).is_none() => {
			Err(Error::Request(format!("unknown role {r}")))
		}
		_ => Ok(()),
	}
}

/// A new user, or a new password and role of one with this email.
async fn create_route(
	State(state): State<AppState>,
	auth: Authed,
	Json(req): Json<UserReq>,
) -> Result<()> {
	auth.check_admin()?;
	check_role(&state, req.role.as_deref())?;
	if !req.email.contains('@') || req.password.chars().count() < 8 {
		return Err(Error::Request(
			"an email and a password of at least 8 characters".into(),
		));
	}
	// bcrypt takes a while, not with the database locked
	let db = state.db.clone();
	tokio::task::spawn_blocking(move || {
		add_user(&db.conn(), &req.email, &req.password, req.role.as_deref())
	})
	.await
	.map_err(|e| Error::Internal(e.to_string()))?
}

#[derive(Debug, Deserialize)]
struct RoleReq {
	role: Option<String>,
}

async fn role_route(
	State(state): State<AppState>,
	auth: Authed,
	Path(id): Path<i64>,
	Json(req): Json<RoleReq>,
) -> Result<()> {
	auth.check_admin()?;
	check_role(&state, req.role.as_deref())?;
	if id == auth.user_id {
		return Err(Error::Request("you can't change your own role".into()));
	}
	let conn = state.db.conn();
	conn.execute(
		"UPDATE users SET role = ?1 WHERE id = ?2",
		params![req.role, id],
	)?;
	Ok(())
}

async fn delete_route(
	State(state): State<AppState>,
	auth: Authed,
	Path(id): Path<i64>,
) -> Result<()> {
	auth.check_admin()?;
	if id == auth.user_id {
		return Err(Error::Request("you can't delete yourself".into()));
	}
	let conn = state.db.conn();
	conn.execute("DELETE FROM sessions WHERE user_id = ?1", [id])?;
	conn.execute("DELETE FROM users WHERE id = ?1", [id])?;
	Ok(())
}

pub fn routes() -> Router<AppState> {
	Router::new()
		.route("/users", get(list_route).post(create_route))
		.route("/users/{id}", put(role_route).delete(delete_route))
}
