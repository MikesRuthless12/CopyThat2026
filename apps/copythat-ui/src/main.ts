import { mount } from "svelte";

import App from "./App.svelte";

const target = document.getElementById("app");
if (!target) {
  throw new Error("Copy That v1.25.0: missing #app mount point");
}

const app = mount(App, { target });

export default app;
