use crate::sql::algorithm::{algorithm, Algorithm};
use crate::sql::base::{base, Base};
use crate::sql::comment::shouldbespace;
use crate::sql::common::take_u64;
use crate::sql::expression::{expression, Expression};
use crate::sql::ident::ident_raw;
use crate::sql::idiom::{idiom, idioms, Idiom, Idioms};
use crate::sql::statement::{statements, Statements};
use crate::sql::strand::strand_raw;
use crate::sql::view::{view, View};
use crate::sql::{
	duration::{duration, Duration},
	kind::{kind, Kind},
	permission::{permissions, Permissions},
};
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::combinator::{map, opt};
use nom::{multi::many0, IResult};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum DefineStatement {
	Namespace(DefineNamespaceStatement),
	Database(DefineDatabaseStatement),
	Login(DefineLoginStatement),
	Token(DefineTokenStatement),
	Scope(DefineScopeStatement),
	Table(DefineTableStatement),
	Event(DefineEventStatement),
	Field(DefineFieldStatement),
	Index(DefineIndexStatement),
}

impl fmt::Display for DefineStatement {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			DefineStatement::Namespace(v) => write!(f, "{}", v),
			DefineStatement::Database(v) => write!(f, "{}", v),
			DefineStatement::Login(v) => write!(f, "{}", v),
			DefineStatement::Token(v) => write!(f, "{}", v),
			DefineStatement::Scope(v) => write!(f, "{}", v),
			DefineStatement::Table(v) => write!(f, "{}", v),
			DefineStatement::Event(v) => write!(f, "{}", v),
			DefineStatement::Field(v) => write!(f, "{}", v),
			DefineStatement::Index(v) => write!(f, "{}", v),
		}
	}
}

pub fn define(i: &str) -> IResult<&str, DefineStatement> {
	alt((
		map(namespace, |v| DefineStatement::Namespace(v)),
		map(database, |v| DefineStatement::Database(v)),
		map(login, |v| DefineStatement::Login(v)),
		map(token, |v| DefineStatement::Token(v)),
		map(scope, |v| DefineStatement::Scope(v)),
		map(table, |v| DefineStatement::Table(v)),
		map(event, |v| DefineStatement::Event(v)),
		map(field, |v| DefineStatement::Field(v)),
		map(index, |v| DefineStatement::Index(v)),
	))(i)
}

// --------------------------------------------------
// --------------------------------------------------
// --------------------------------------------------

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct DefineNamespaceStatement {
	pub name: String,
}

impl fmt::Display for DefineNamespaceStatement {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "DEFINE NAMESPACE {}", self.name)
	}
}

fn namespace(i: &str) -> IResult<&str, DefineNamespaceStatement> {
	let (i, _) = tag_no_case("DEFINE")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, _) = alt((tag_no_case("NS"), tag_no_case("NAMESPACE")))(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, name) = ident_raw(i)?;
	Ok((
		i,
		DefineNamespaceStatement {
			name: String::from(name),
		},
	))
}

// --------------------------------------------------
// --------------------------------------------------
// --------------------------------------------------

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct DefineDatabaseStatement {
	pub name: String,
}

impl fmt::Display for DefineDatabaseStatement {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "DEFINE DATABASE {}", self.name)
	}
}

fn database(i: &str) -> IResult<&str, DefineDatabaseStatement> {
	let (i, _) = tag_no_case("DEFINE")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, _) = alt((tag_no_case("DB"), tag_no_case("DATABASE")))(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, name) = ident_raw(i)?;
	Ok((
		i,
		DefineDatabaseStatement {
			name: String::from(name),
		},
	))
}

// --------------------------------------------------
// --------------------------------------------------
// --------------------------------------------------

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct DefineLoginStatement {
	pub name: String,
	pub base: Base,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub pass: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hash: Option<String>,
}

impl fmt::Display for DefineLoginStatement {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "DEFINE LOGIN {} ON {}", self.name, self.base)?;
		if let Some(ref v) = self.pass {
			write!(f, " PASSWORD {}", v)?
		}
		if let Some(ref v) = self.hash {
			write!(f, " PASSHASH {}", v)?
		}
		Ok(())
	}
}

fn login(i: &str) -> IResult<&str, DefineLoginStatement> {
	let (i, _) = tag_no_case("DEFINE")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("LOGIN")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, name) = ident_raw(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("ON")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, base) = base(i)?;
	let (i, opts) = login_opts(i)?;
	Ok((
		i,
		DefineLoginStatement {
			name: String::from(name),
			base,
			pass: match opts {
				DefineLoginOption::Password(ref v) => Some(v.to_owned()),
				_ => None,
			},
			hash: match opts {
				DefineLoginOption::Passhash(ref v) => Some(v.to_owned()),
				_ => None,
			},
		},
	))
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum DefineLoginOption {
	Password(String),
	Passhash(String),
}

fn login_opts(i: &str) -> IResult<&str, DefineLoginOption> {
	alt((login_pass, login_hash))(i)
}

fn login_pass(i: &str) -> IResult<&str, DefineLoginOption> {
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("PASSWORD")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, v) = strand_raw(i)?;
	Ok((i, DefineLoginOption::Password(String::from(v))))
}

fn login_hash(i: &str) -> IResult<&str, DefineLoginOption> {
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("PASSHASH")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, v) = strand_raw(i)?;
	Ok((i, DefineLoginOption::Passhash(String::from(v))))
}

// --------------------------------------------------
// --------------------------------------------------
// --------------------------------------------------

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct DefineTokenStatement {
	pub name: String,
	pub base: Base,
	pub kind: Algorithm,
	pub code: String,
}

impl fmt::Display for DefineTokenStatement {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"DEFINE TOKEN {} ON {} TYPE {} VALUE {}",
			self.name, self.base, self.kind, self.code
		)
	}
}

fn token(i: &str) -> IResult<&str, DefineTokenStatement> {
	let (i, _) = tag_no_case("DEFINE")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("TOKEN")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, name) = ident_raw(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("ON")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, base) = base(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("TYPE")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, kind) = algorithm(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("VALUE")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, code) = strand_raw(i)?;
	Ok((
		i,
		DefineTokenStatement {
			name: String::from(name),
			base,
			kind,
			code: String::from(code),
		},
	))
}

// --------------------------------------------------
// --------------------------------------------------
// --------------------------------------------------

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct DefineScopeStatement {
	pub name: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub session: Option<Duration>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub signup: Option<Expression>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub signin: Option<Expression>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub connect: Option<Expression>,
}

impl fmt::Display for DefineScopeStatement {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "DEFINE SCOPE {}", self.name)?;
		if let Some(ref v) = self.session {
			write!(f, " {}", v)?
		}
		if let Some(ref v) = self.signup {
			write!(f, " {}", v)?
		}
		if let Some(ref v) = self.signin {
			write!(f, " {}", v)?
		}
		if let Some(ref v) = self.connect {
			write!(f, " {}", v)?
		}
		Ok(())
	}
}

fn scope(i: &str) -> IResult<&str, DefineScopeStatement> {
	let (i, _) = tag_no_case("DEFINE")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("SCOPE")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, name) = ident_raw(i)?;
	let (i, opts) = many0(scope_opts)(i)?;
	Ok((
		i,
		DefineScopeStatement {
			name: String::from(name),
			session: opts.iter().find_map(|x| match x {
				DefineScopeOption::Session(ref v) => Some(v.to_owned()),
				_ => None,
			}),
			signup: opts.iter().find_map(|x| match x {
				DefineScopeOption::Signup(ref v) => Some(v.to_owned()),
				_ => None,
			}),
			signin: opts.iter().find_map(|x| match x {
				DefineScopeOption::Signin(ref v) => Some(v.to_owned()),
				_ => None,
			}),
			connect: opts.iter().find_map(|x| match x {
				DefineScopeOption::Connect(ref v) => Some(v.to_owned()),
				_ => None,
			}),
		},
	))
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum DefineScopeOption {
	Session(Duration),
	Signup(Expression),
	Signin(Expression),
	Connect(Expression),
}

fn scope_opts(i: &str) -> IResult<&str, DefineScopeOption> {
	alt((scope_session, scope_signup, scope_signin, scope_connect))(i)
}

fn scope_session(i: &str) -> IResult<&str, DefineScopeOption> {
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("SESSION")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, v) = duration(i)?;
	Ok((i, DefineScopeOption::Session(v)))
}

fn scope_signup(i: &str) -> IResult<&str, DefineScopeOption> {
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("SIGNUP")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, v) = expression(i)?;
	Ok((i, DefineScopeOption::Signup(v)))
}

fn scope_signin(i: &str) -> IResult<&str, DefineScopeOption> {
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("SIGNIN")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, v) = expression(i)?;
	Ok((i, DefineScopeOption::Signin(v)))
}

fn scope_connect(i: &str) -> IResult<&str, DefineScopeOption> {
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("CONNECT")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, v) = expression(i)?;
	Ok((i, DefineScopeOption::Connect(v)))
}

// --------------------------------------------------
// --------------------------------------------------
// --------------------------------------------------

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct DefineTableStatement {
	pub name: String,
	pub drop: bool,
	pub full: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub view: Option<View>,
	pub permissions: Permissions,
}

impl fmt::Display for DefineTableStatement {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "DEFINE TABLE {}", self.name)?;
		if self.drop == true {
			write!(f, " DROP")?
		}
		if self.full == true {
			write!(f, " SCHEMAFULL")?
		}
		if self.full == false {
			write!(f, " SCHEMALESS")?
		}
		if self.drop == true {
			write!(f, " DROP")?
		}
		if let Some(ref v) = self.view {
			write!(f, " {}", v)?
		}
		write!(f, "{}", self.permissions)?;
		Ok(())
	}
}

fn table(i: &str) -> IResult<&str, DefineTableStatement> {
	let (i, _) = tag_no_case("DEFINE")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("TABLE")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, name) = ident_raw(i)?;
	let (i, opts) = many0(table_opts)(i)?;
	Ok((
		i,
		DefineTableStatement {
			name: String::from(name),
			drop: opts
				.iter()
				.find_map(|x| match x {
					DefineTableOption::Drop => Some(true),
					_ => None,
				})
				.unwrap_or(Default::default()),
			full: opts
				.iter()
				.find_map(|x| match x {
					DefineTableOption::Schemafull => Some(true),
					DefineTableOption::Schemaless => Some(false),
					_ => None,
				})
				.unwrap_or(Default::default()),
			view: opts.iter().find_map(|x| match x {
				DefineTableOption::View(ref v) => Some(v.to_owned()),
				_ => None,
			}),
			permissions: opts
				.iter()
				.find_map(|x| match x {
					DefineTableOption::Permissions(ref v) => Some(v.to_owned()),
					_ => None,
				})
				.unwrap_or(Default::default()),
		},
	))
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum DefineTableOption {
	Drop,
	View(View),
	Schemaless,
	Schemafull,
	Permissions(Permissions),
}

fn table_opts(i: &str) -> IResult<&str, DefineTableOption> {
	alt((table_drop, table_view, table_schemaless, table_schemafull, table_permissions))(i)
}

fn table_drop(i: &str) -> IResult<&str, DefineTableOption> {
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("DROP")(i)?;
	Ok((i, DefineTableOption::Drop))
}

fn table_view(i: &str) -> IResult<&str, DefineTableOption> {
	let (i, _) = shouldbespace(i)?;
	let (i, v) = view(i)?;
	Ok((i, DefineTableOption::View(v)))
}

fn table_schemaless(i: &str) -> IResult<&str, DefineTableOption> {
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("SCHEMALESS")(i)?;
	Ok((i, DefineTableOption::Schemaless))
}

fn table_schemafull(i: &str) -> IResult<&str, DefineTableOption> {
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("SCHEMAFULL")(i)?;
	Ok((i, DefineTableOption::Schemafull))
}

fn table_permissions(i: &str) -> IResult<&str, DefineTableOption> {
	let (i, _) = shouldbespace(i)?;
	let (i, v) = permissions(i)?;
	Ok((i, DefineTableOption::Permissions(v)))
}

// --------------------------------------------------
// --------------------------------------------------
// --------------------------------------------------

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct DefineEventStatement {
	pub name: String,
	pub what: String,
	pub when: Expression,
	pub then: Statements,
}

impl fmt::Display for DefineEventStatement {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"DEFINE EVENT {} ON {} WHEN {} THEN {}",
			self.name, self.what, self.when, self.then
		)
	}
}

fn event(i: &str) -> IResult<&str, DefineEventStatement> {
	let (i, _) = tag_no_case("DEFINE")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("EVENT")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, name) = ident_raw(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("ON")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, what) = ident_raw(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("WHEN")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, when) = expression(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("THEN")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, then) = statements(i)?;
	Ok((
		i,
		DefineEventStatement {
			name: String::from(name),
			what: String::from(what),
			when,
			then,
		},
	))
}

// --------------------------------------------------
// --------------------------------------------------
// --------------------------------------------------

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct DefineFieldStatement {
	pub name: Idiom,
	pub what: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub kind: Option<Kind>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub value: Option<Expression>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub assert: Option<Expression>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub priority: Option<u64>,
	pub permissions: Permissions,
}

impl fmt::Display for DefineFieldStatement {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "DEFINE FIELD {} ON {}", self.name, self.what)?;
		if let Some(ref v) = self.kind {
			write!(f, " TYPE {}", v)?
		}
		if let Some(ref v) = self.value {
			write!(f, " VALUE {}", v)?
		}
		if let Some(ref v) = self.assert {
			write!(f, " ASSERT {}", v)?
		}
		write!(f, "{}", self.permissions)?;
		Ok(())
	}
}

fn field(i: &str) -> IResult<&str, DefineFieldStatement> {
	let (i, _) = tag_no_case("DEFINE")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("FIELD")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, name) = idiom(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("ON")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, what) = ident_raw(i)?;
	let (i, opts) = many0(field_opts)(i)?;
	Ok((
		i,
		DefineFieldStatement {
			name,
			what: String::from(what),
			kind: opts.iter().find_map(|x| match x {
				DefineFieldOption::Kind(ref v) => Some(v.to_owned()),
				_ => None,
			}),
			value: opts.iter().find_map(|x| match x {
				DefineFieldOption::Value(ref v) => Some(v.to_owned()),
				_ => None,
			}),
			assert: opts.iter().find_map(|x| match x {
				DefineFieldOption::Assert(ref v) => Some(v.to_owned()),
				_ => None,
			}),
			priority: opts.iter().find_map(|x| match x {
				DefineFieldOption::Priority(ref v) => Some(v.to_owned()),
				_ => None,
			}),
			permissions: opts
				.iter()
				.find_map(|x| match x {
					DefineFieldOption::Permissions(ref v) => Some(v.to_owned()),
					_ => None,
				})
				.unwrap_or(Default::default()),
		},
	))
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum DefineFieldOption {
	Kind(Kind),
	Value(Expression),
	Assert(Expression),
	Priority(u64),
	Permissions(Permissions),
}

fn field_opts(i: &str) -> IResult<&str, DefineFieldOption> {
	alt((field_kind, field_value, field_assert, field_priority, field_permissions))(i)
}

fn field_kind(i: &str) -> IResult<&str, DefineFieldOption> {
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("TYPE")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, v) = kind(i)?;
	Ok((i, DefineFieldOption::Kind(v)))
}

fn field_value(i: &str) -> IResult<&str, DefineFieldOption> {
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("VALUE")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, v) = expression(i)?;
	Ok((i, DefineFieldOption::Value(v)))
}

fn field_assert(i: &str) -> IResult<&str, DefineFieldOption> {
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("ASSERT")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, v) = expression(i)?;
	Ok((i, DefineFieldOption::Assert(v)))
}

fn field_priority(i: &str) -> IResult<&str, DefineFieldOption> {
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("PRIORITY")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, v) = take_u64(i)?;
	Ok((i, DefineFieldOption::Priority(v)))
}

fn field_permissions(i: &str) -> IResult<&str, DefineFieldOption> {
	let (i, _) = shouldbespace(i)?;
	let (i, v) = permissions(i)?;
	Ok((i, DefineFieldOption::Permissions(v)))
}

// --------------------------------------------------
// --------------------------------------------------
// --------------------------------------------------

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct DefineIndexStatement {
	pub name: String,
	pub what: String,
	pub cols: Idioms,
	pub uniq: bool,
}

impl fmt::Display for DefineIndexStatement {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "DEFINE INDEX {} ON {} COLUMNS {}", self.name, self.what, self.cols)?;
		if self.uniq == true {
			write!(f, " UNIQUE")?
		}
		Ok(())
	}
}

fn index(i: &str) -> IResult<&str, DefineIndexStatement> {
	let (i, _) = tag_no_case("DEFINE")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("INDEX")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, name) = ident_raw(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("ON")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, what) = ident_raw(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("COLUMNS")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, cols) = idioms(i)?;
	let (i, uniq) = opt(|i: &str| {
		shouldbespace(i)?;
		tag_no_case("UNIQUE")(i)?;
		Ok((i, true))
	})(i)?;
	Ok((
		i,
		DefineIndexStatement {
			name: String::from(name),
			what: String::from(what),
			cols,
			uniq: uniq.is_some(),
		},
	))
}