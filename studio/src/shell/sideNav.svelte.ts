/**
 * The sidebar of the studio when a page hides it for room: `hidden`
 * while a page wants the width (the entry editor with its live
 * preview), `open` while it slides in over the page. The AI page and
 * phones always hide it.
 */
export const sideNav = $state({ open: false, hidden: false });
