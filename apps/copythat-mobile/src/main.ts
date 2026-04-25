import App from "./App.svelte";
import { mount } from "svelte";

import "./app.css";

const root = document.getElementById("app");
if (!root) {
  throw new Error("missing #app mount");
}

const app = mount(App, { target: root });

export default app;
