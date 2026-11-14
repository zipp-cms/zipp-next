/** @import { FieldData, FieldError, FormData } from './types' */

/**
 * @typedef {object} FormOptions
 * @property {string} site the handle of the site the form is sent from
 * @property {Record<string, string>} [messages] texts of the errors by
 * their code (`required`, `email`, …) and `failed` when the form can't
 * send; the cms's English ones else
 */

const FAILED = "The form could not send.";

/**
 * A form of zipp-forms with what the visitor typed: `values` by handle,
 * `errors` by handle after sending, `sent` with the message after
 * sending. Made by `loadForm`. Plain JavaScript: vite compiles a
 * dependency's `.svelte.js` as it is.
 */
export class ZippForm {
  /** @type {Record<string, string>} */
  values = $state({});
  /** @type {Record<string, string>} */
  errors = $state({});
  sending = $state(false);
  /** @type {string | null} the message after sending, null before */
  sent = $state(null);
  /** why the form couldn't send, empty if it could */
  failed = $state("");

  /**
   * @param {FormData} data
   * @param {FormOptions} opts
   */
  constructor(data, opts) {
    /** @readonly */
    this.data = data;
    /** @readonly */
    this.site = opts.site;
    this.messages = opts.messages ?? {};
    this.reset();
  }

  /** @returns {FieldData[]} */
  get fields() {
    return this.data.fields;
  }

  /**
   * An id for a field's input, unique when a page shows the form twice.
   * @param {FieldData} field
   */
  inputId(field) {
    return `zipp-form-${this.data.id}-${field.handle}`;
  }

  /** Empty values, the message gone: the form as it was loaded. */
  reset() {
    /** @type {Record<string, string>} */
    const values = { [this.data.honeypot]: "" };
    for (const f of this.data.fields) values[f.handle] = "";
    this.values = values;
    this.errors = {};
    this.sent = null;
    this.failed = "";
  }

  /**
   * Sends the values through the frontend's `/forms/{id}` (see
   * `formRoutes`).
   * @returns {Promise<boolean>} whether it went
   */
  async submit() {
    this.sending = true;
    this.errors = {};
    this.failed = "";
    try {
      const site = encodeURIComponent(this.site);
      const resp = await fetch(`/forms/${this.data.id}?site=${site}`, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify(this.values),
      });
      const body = await resp.json().catch(() => ({}));
      if (body.success) {
        this.sent = body.message;
        return true;
      }
      if (body.errors) {
        /** @type {FieldError[]} */
        const errors = body.errors;
        this.errors = Object.fromEntries(
          errors.map((e) => [e.path, this.messages[e.code] ?? e.message]),
        );
      } else {
        this.failed = this.messages.failed ?? body.message ?? FAILED;
      }
    } catch {
      this.failed = this.messages.failed ?? FAILED;
    } finally {
      this.sending = false;
    }
    return false;
  }
}
