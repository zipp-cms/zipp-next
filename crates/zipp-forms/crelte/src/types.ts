/** A form as the cms gives it, `GET /forms/{id}?site=`. */
export type FormData = {
	id: number;
	title: string;
	/** one at least, the steps of the form */
	pages: PageData[];
	submitLabel: string;
	/** the buttons between the pages */
	nextLabel: string;
	backLabel: string;
	successMessage: string;
	/** the field people don't see and bots fill, sent empty */
	honeypot: string;
};

/** A step of a form, `label` is empty for a first page without one. */
export type PageData = { label: string; fields: FieldData[] };

export type FieldType = 'text' | 'textarea' | 'email' | 'phone';

export type FieldData = {
	type: FieldType;
	/** the key of its value, unique in the form */
	handle: string;
	label: string;
	required: boolean;
	placeholder: string;
	instructions: string;
	/** percent of a row: 25, 33, 50, 67, 75 or 100 */
	width: number;
};

/**
 * What is wrong with a value: `code` is `required`, `oneLine`,
 * `tooLong`, `email`, `phone` or `text`, the message is the cms's in
 * English.
 */
export type FieldError = { path: string; code: string; message: string };
