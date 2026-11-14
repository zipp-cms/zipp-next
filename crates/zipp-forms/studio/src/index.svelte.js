// The studio module of the forms plugin: `page` is its page
// Submissions.
import { mount, unmount } from "svelte";
import Submissions from "./Submissions.svelte";

export function page(target, props) {
  const app = mount(Submissions, { target, props });
  return {
    destroy() {
      unmount(app);
    },
  };
}
