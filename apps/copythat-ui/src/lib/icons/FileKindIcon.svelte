<!--
  Map a `FileIconDto.kind` to a concrete `<Icon />` choice. Keeps
  every JobRow free of a case-analysis block, and gives us one
  place to swap Phase 7's native-icon render path in.
-->
<script lang="ts">
  import Icon from "./Icon.svelte";
  import type { FileIconDto } from "../types";

  interface Props {
    info: FileIconDto;
    size?: number;
  }

  let { info, size = 18 }: Props = $props();

  const iconName = $derived.by(() => {
    switch (info.kind) {
      case "folder":
        return "folder";
      case "symlink":
        return "folder-symlink";
      case "image":
        return "file-image";
      case "audio":
        return "file-audio";
      case "video":
        return "file-video";
      case "archive":
        return "archive";
      case "text":
      case "pdf":
        return "file-text";
      case "code":
        return "file-code";
      case "binary":
        return "download";
      default:
        return "file";
    }
  });
</script>

<Icon name={iconName} {size} />
