<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
  
    let repository = $state({
        general: {
            repository_name: "",
            created_on: "",
            files: []
        }
    });
  
    async function loadRepository() {
      repository = await invoke("list_files");
    }

    onMount(async () => {
        await loadRepository();
        console.log(repository);
    });
  </script>

<aside id="explorer">
    <div class="sidebar-header">Explorer</div>
    <div class="sidebar-content">
        <div class="sidebar-item">
            <div class="sidebar-item-header">{repository.general.repository_name}</div>
            <div class="sidebar-item-content">
                <ul>
                    {#each repository.general.files as file}
                        <li>{file}</li>
                    {/each}
                </ul>
            </div>
</aside>