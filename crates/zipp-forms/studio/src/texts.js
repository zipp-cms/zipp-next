// The plugin's texts in English and German. The studio sets `<html lang>`
// to its user's language; a view reads it when it opens.
const all = {
  en: {
    title: "Submissions",
    note: "What visitors sent with the forms, the newest first.",
    allForms: "All forms",
    form: "Form",
    date: "Date",
    site: "Site",
    content: "Content",
    none: "No submissions yet.",
    more: "Show more",
    delete: "Delete",
    confirmDelete: "Delete this submission for good?",
    empty: "–",
  },
  de: {
    title: "Einsendungen",
    note: "Was Besucher mit den Formularen geschickt haben, die neusten zuerst.",
    allForms: "Alle Formulare",
    form: "Formular",
    date: "Datum",
    site: "Website",
    content: "Inhalt",
    none: "Noch keine Einsendungen.",
    more: "Mehr zeigen",
    delete: "Löschen",
    confirmDelete: "Diese Einsendung endgültig löschen?",
    empty: "–",
  },
};

/** The texts in the studio's language, English for any other. */
export function texts() {
  return all[document.documentElement.lang] ?? all.en;
}
