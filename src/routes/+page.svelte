<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  async function handleOnFileTitleFocusOut(e: FocusEvent) {
    const fileTitle = (e.target as HTMLDivElement).innerText;
    const key = (e.target as HTMLDivElement).dataset.repositoryKey;

    await invoke("update_file_title", { key, fileTitle }).catch(console.error);
  }
</script>

<main class="container">
  <div id="file-title" data-repository-key="" contenteditable="true" onblur="{handleOnFileTitleFocusOut}"></div>
  <div id="file-view" contenteditable="true"></div>
</main>
