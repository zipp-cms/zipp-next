//! Mails of the cms, its plugins and the site, by SMTP with lettre:
//! `serve --smtp-url smtp://user:pass@mail.example.ch:587?tls=required
//! --mail-from "Website <cms@example.ch>"` (also `ZIPP_SMTP_URL` and
//! `ZIPP_MAIL_FROM`). Without a server a mail is only logged; in
//! development Mailpit catches them at `smtp://localhost:1025`.
//!
//! ```no_run
//! # async fn f(state: zipp::AppState) -> zipp::error::Result<()> {
//! use zipp::mail::Mail;
//! let mail = Mail::new("anna@example.ch", "Thanks", "We got your message.");
//! state.mail.send(mail).await?;
//! # Ok(())
//! # }
//! ```

use std::sync::Arc;
#[cfg(any(test, feature = "testing"))]
use std::sync::Mutex;

use lettre::message::{Mailbox, MultiPart};
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

use crate::error::{Error, Result};

pub mod api;
#[cfg(test)]
mod api_tests;
mod routes;

pub(crate) use routes::routes;

/// A mail to send: a text, and html for clients that show it.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct Mail {
	/// an address, or `Anna Muster <anna@example.ch>`
	pub to: String,
	pub subject: String,
	pub text: String,
	pub html: Option<String>,
	pub reply_to: Option<String>,
}

impl Mail {
	pub fn new(to: &str, subject: &str, text: &str) -> Self {
		Self {
			to: to.into(),
			subject: subject.into(),
			text: text.into(),
			html: None,
			reply_to: None,
		}
	}

	/// The mail as html too, the text stays for clients without.
	pub fn html(mut self, html: &str) -> Self {
		self.html = Some(html.into());
		self
	}

	/// Where answers go, like the sender of a contact form.
	pub fn reply_to(mut self, address: &str) -> Self {
		self.reply_to = Some(address.into());
		self
	}
}

/// What `zipp mail` and the studio send to check the setup.
pub(crate) fn test_mail(to: &str) -> Mail {
	let text = "This mail comes from zipp: its mails reach you.";
	Mail::new(to, "A test mail of zipp", text)
}

enum Transport {
	Smtp(AsyncSmtpTransport<Tokio1Executor>),
	/// no server: the mail is logged
	Log,
	/// the tests': kept to look at
	#[cfg(any(test, feature = "testing"))]
	Memory(Mutex<Vec<Mail>>),
}

struct Inner {
	transport: Transport,
	from: Mailbox,
	/// where mails go, without credentials, for the studio
	server: Option<String>,
}

/// Sends the mails, in [`AppState`](crate::AppState) as `mail`.
#[derive(Clone)]
pub struct Mailer(Arc<Inner>);

impl Default for Mailer {
	fn default() -> Self {
		Self::new(Transport::Log, None, None).unwrap()
	}
}

impl Mailer {
	fn new(
		transport: Transport,
		from: Option<&str>,
		server: Option<String>,
	) -> std::result::Result<Self, String> {
		let from = from.unwrap_or("zipp <zipp@localhost>");
		let from = from
			.parse()
			.map_err(|e| format!("--mail-from {from:?}: {e}"))?;
		Ok(Self(Arc::new(Inner {
			transport,
			from,
			server,
		})))
	}

	/// Mails over the SMTP server of `url` (lettre's: `smtp://` with
	/// `?tls=required` for STARTTLS, `smtps://` for TLS, user and password
	/// before the host), from `from`.
	pub fn smtp(url: &str, from: &str) -> std::result::Result<Self, String> {
		let transport = AsyncSmtpTransport::<Tokio1Executor>::from_url(url)
			.map_err(|e| format!("--smtp-url: {e}"))?
			.build();
		// the host and port, without the user and password
		let rest = url.split_once("://").map_or(url, |(_, r)| r);
		let rest = rest.rsplit_once('@').map_or(rest, |(_, r)| r);
		let server = rest.split(['/', '?']).next().map(String::from);
		Self::new(Transport::Smtp(transport), Some(from), server)
	}

	/// Keeps the mails instead of sending them, see [`Mailer::sent`].
	#[cfg(any(test, feature = "testing"))]
	pub fn memory() -> Self {
		Self::new(Transport::Memory(Mutex::default()), None, None).unwrap()
	}

	/// The mails a [`Mailer::memory`] got, none of the others.
	#[cfg(any(test, feature = "testing"))]
	pub fn sent(&self) -> Vec<Mail> {
		match &self.0.transport {
			Transport::Memory(m) => m.lock().unwrap().clone(),
			_ => vec![],
		}
	}

	/// Whether mails go to a server, else they are only logged.
	pub fn configured(&self) -> bool {
		!matches!(self.0.transport, Transport::Log)
	}

	/// The host and port mails go to.
	pub fn server(&self) -> Option<&str> {
		self.0.server.as_deref()
	}

	/// The sender of every mail.
	pub fn from(&self) -> String {
		self.0.from.to_string()
	}

	pub async fn send(&self, mail: Mail) -> Result<()> {
		let message = self.message(&mail)?;
		match &self.0.transport {
			Transport::Smtp(t) => {
				t.send(message).await.map_err(|e| {
					Error::Internal(format!("the mail to {}: {e}", mail.to))
				})?;
			}
			Transport::Log => tracing::info!(
				"no --smtp-url, not sent: a mail to {} about {:?}",
				mail.to,
				mail.subject
			),
			#[cfg(any(test, feature = "testing"))]
			Transport::Memory(m) => m.lock().unwrap().push(mail),
		}
		Ok(())
	}

	fn message(&self, mail: &Mail) -> Result<Message> {
		let address = |a: &str| {
			a.parse::<Mailbox>()
				.map_err(|e| Error::Request(format!("the address {a:?}: {e}")))
		};
		let mut b = Message::builder()
			.from(self.0.from.clone())
			.to(address(&mail.to)?)
			.subject(&mail.subject);
		if let Some(r) = &mail.reply_to {
			b = b.reply_to(address(r)?);
		}
		let built = match &mail.html {
			Some(html) => b.multipart(MultiPart::alternative_plain_html(
				mail.text.clone(),
				html.clone(),
			)),
			None => b.body(mail.text.clone()),
		};
		built.map_err(|e| Error::Internal(e.to_string()))
	}
}
