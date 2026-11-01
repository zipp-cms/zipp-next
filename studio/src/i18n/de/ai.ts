import type { Texts } from '../index.svelte';
import type en from '../en/ai';

export default {
	'ai.title': 'KI',
	'ai.width': 'Breite des Chats',
	'ai.startOver': 'Neu beginnen, dieser Chat bleibt nur in diesem Tab',
	'ai.newChat': 'Neuer Chat',
	'ai.hintAgent':
		'Entwicklung: Fragen Sie nach allem, Inhalt, Design oder neuen Feldern, z. B. «Füge ein Feld für die Öffnungszeiten hinzu» oder «Mache die Buttons rund». Code-Änderungen erscheinen sofort rechts, Inhaltsänderungen sind Entwürfe, die Sie veröffentlichen.',
	'ai.hint':
		'Fragen Sie nach allem auf der Website, z. B. «Schreibe eine bessere Einleitung für Über uns», «Füge einen Anlass in Basel hinzu» oder «Übersetze den Footer ins Englische». Änderungen sind Entwürfe: Sie sehen sie rechts und veröffentlichen sie im Editor.',
	'ai.placeholder': 'Was soll sich ändern?',
	'ai.message': 'Nachricht',
	'ai.keys': 'Enter sendet, Shift+Enter neue Zeile',
	'ai.send': 'Senden',
	'ai.endedEarly': 'Die Antwort endete zu früh',
} satisfies Texts<typeof en>;
