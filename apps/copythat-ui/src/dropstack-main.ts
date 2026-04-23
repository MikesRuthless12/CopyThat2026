import { mount } from "svelte";

import DropStack from "./lib/components/DropStack.svelte";

const target = document.getElementById("app");
if (!target) {
  throw new Error("Copy That v1.25.0: missing #app mount point for Drop Stack");
}

const win = mount(DropStack, { target });

export default win;
