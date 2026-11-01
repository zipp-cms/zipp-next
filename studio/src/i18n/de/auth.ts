import type { Texts } from '../index.svelte';
import type en from '../en/auth';

export default {
	'auth.logIn': 'Anmelden',
	'auth.welcomeBack':
		'Willkommen zurück. Melden Sie sich mit Ihrem Konto an.',
	'auth.emailPlaceholder': 'name@beispiel.ch',
	'auth.showPassword': 'Passwort anzeigen',
	'auth.hidePassword': 'Passwort verbergen',
	'auth.stayLoggedIn': 'Angemeldet bleiben',
	'auth.forgot': 'Passwort vergessen?',
	'auth.forgotIntro':
		'Sie erhalten per E-Mail einen Link, um ein neues zu wählen.',
	'auth.forgotSent':
		'Wenn {email} ein Konto hat, ist eine E-Mail mit einem Link unterwegs. Der Link ist eine Stunde gültig.',
	'auth.sendLink': 'Link senden',
	'auth.backToLogin': 'Zurück zur Anmeldung',
	'auth.linkBroken': 'Dieser Link funktioniert nicht',
	'auth.loggingIn': 'Anmelden…',
	'auth.toStudio': 'Zum Studio',
	'auth.sessionEnded': 'Die Sitzung ist abgelaufen',
	'auth.wereLoggedInAs':
		'Sie waren als {name} angemeldet. Gehen Sie zurück zu Ihrem Konto oder melden Sie sich an.',
	'auth.backToAccount': 'Zurück zu Ihrem Konto',
	'auth.logInAgain':
		'Melden Sie sich erneut an, um weiterzuarbeiten. Ihre Änderungen bleiben erhalten.',
	'auth.logOut': 'Abmelden',
	'auth.welcome': 'Willkommen',
	'auth.newPasswordTitle': 'Ein neues Passwort',
	'auth.inviteIntro':
		'Ihr Konto ist {email}. Wählen Sie ein Passwort, um sich anzumelden.',
	'auth.resetIntro':
		'Wählen Sie ein neues Passwort für {email}. Sie werden überall sonst abgemeldet.',
	'auth.yourName': 'Ihr Name',
	'auth.namePlaceholder': 'Anna Muster',
	'auth.passwordPlaceholder': 'Mindestens 8 Zeichen',
	'auth.setAndLogIn': 'Passwort festlegen und anmelden',
	'auth.saveAndLogIn': 'Speichern und anmelden',
	'auth.linkProblem':
		'{problem}. Fordern Sie einen neuen an oder bitten Sie einen Admin um eine Einladung.',
	'auth.newPassword': 'Neues Passwort',
} satisfies Texts<typeof en>;
