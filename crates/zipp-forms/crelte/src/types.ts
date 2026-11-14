/** A form as the cms gives it, `GET /forms/{id}?site=`. */
export type FormData = {
  id: number;
  title: string;
  fields: FieldData[];
  submitLabel: string;
  successMessage: string;
  /** the field people don't see and bots fill, sent empty */
  honeypot: string;
};

export type FieldType = "text" | "textarea" | "email" | "phone";

export type FieldData = {
  type: FieldType;
  /** the key of its value, unique in the form */
  handle: string;
  label: string;
  required: boolean;
  placeholder: string;
  instructions: string;
};

/**
 * What is wrong with a value: `code` is `required`, `oneLine`,
 * `tooLong`, `email`, `phone` or `text`, the message is the cms's in
 * English.
 */
export type FieldError = { path: string; code: string; message: string };
