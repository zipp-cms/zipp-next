import { getContext, setContext } from "svelte";
import type { ZippForm } from "./form.svelte.js";

const KEY = "zipp-form";

export function setForm(form: ZippForm) {
  setContext(KEY, form);
}

/** The form of the `Form` around a component. */
export function getForm(): ZippForm {
  return getContext<ZippForm>(KEY);
}
