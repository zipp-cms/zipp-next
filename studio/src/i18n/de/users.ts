import type { Texts } from '../index.svelte';
import type en from '../en/users';

export default {
	'users.title': 'Benutzer',
	'users.new': 'Neuer Benutzer',
	'users.user': 'Benutzer',
	'users.role': 'Rolle',
	'users.lastLogin': 'Letzte Anmeldung',
	'users.you': '(Sie)',
	'users.invited': 'Eingeladen',
	'users.account': 'Konto',
	'users.yourAccount': 'Ihr Konto',
	'users.thatsYou': 'Das sind Sie',
	'users.delete': 'Benutzer löschen',
	'users.confirmDelete':
		'{name} löschen? Die Person kann sich nicht mehr anmelden.',
	'users.confirmEndOwn': 'In allen anderen Browsern abmelden?',
	'users.confirmEnd': '{name} überall abmelden?',
	'users.namePlaceholder': 'Anna Muster',
	'users.language': 'Sprache des Studios',
	'users.browserLanguage': 'Die des Browsers ({language})',
	'users.passwordNew':
		'Ohne Passwort erhält die Person einen Einladungslink, um eines zu wählen.',
	'users.passwordKeepOwn':
		'Leer lassen, um das bisherige zu behalten. Ein neues Passwort meldet Sie in allen anderen Browsern ab.',
	'users.passwordKeep':
		'Leer lassen, um das bisherige zu behalten. Ein neues Passwort meldet die Person in allen Browsern ab.',
	'users.newPassword': 'Neues Passwort',
	'users.passwordPlaceholder': 'Mindestens 8 Zeichen',
	'users.sendInvite': 'Einladung per E-Mail senden',
	'users.currentPassword': 'Ihr aktuelles Passwort',
	'users.created': 'Erstellt',
	'users.loggedIn': 'Angemeldet',
	'users.notLoggedIn': 'Nicht angemeldet',
	'users.browsers': { one: 'in {n} Browser', other: 'in {n} Browsern' },
	'users.endOwn': 'Überall sonst abmelden',
	'users.end': 'Überall abmelden',
	'users.access': 'Zugang',
	'users.invitedNote': 'Eingeladen, noch kein Passwort.',
	'users.sendInviteAgain': 'Einladung nochmals senden',
	'users.sendReset': 'Passwort-Link senden',
	'users.copyInvite': 'Einladungslink kopieren',
	'users.copyReset': 'Passwort-Link kopieren',
	'users.loginAs': 'Als {name} anmelden',
	'users.copyLogin': 'Anmeldelink kopieren',
	'users.copyLoginTitle':
		'Zum Öffnen in einem privaten Fenster, Ihre Anmeldung bleibt',
	'users.inviteFailed': 'Die Einladung konnte nicht gesendet werden: {error}',
	'users.inviteSent':
		'Die Einladung ist an {email} gesendet, sie gilt bis {until}.',
	'users.resetSent':
		'Der Passwort-Link ist an {email} gesendet, er gilt bis {until}.',
	'users.inviteCopied':
		'Die Einladung ist kopiert, sie gilt einmal bis {until}.',
	'users.resetCopied':
		'Der Passwort-Link ist kopiert, er gilt einmal bis {until}.',
	'users.loginCopied':
		'Der Anmeldelink ist kopiert: einmal öffnen, in den nächsten 10 Minuten.',
	'users.adminMay':
		'Admins dürfen alles: alle Abschnitte, Dateien, KI, Benutzer und Einrichtung.',
	'users.assets': 'Dateien',
	'users.ai': 'KI',
	'users.levels':
		'Ansehen zeigt Einträge, Bearbeiten ändert sie als Entwurf, Veröffentlichen stellt sie online, Löschen löscht sie. Die Rollen sind Code, {setup} zeigt alle.',
	'users.setup': 'Einrichtung',
} satisfies Texts<typeof en>;
