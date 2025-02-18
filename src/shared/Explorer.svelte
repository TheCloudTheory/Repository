<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    interface DirectoryObject {
        is_directory: boolean;
        name: string;
        objects: DirectoryObject[];
        extension: string;
    }

    interface General {
        repository_name: string;
        created_on: string;
    }

    interface Repository {
        general: General;
        objects: { [key: string]: DirectoryObject };
    }
  
    let repository = $state({
        general: {
            repository_name: "",
            created_on: ""
        },
        objects: {}
    } as Repository);

    let isAddingNewFile = $state(false);
  
    async function loadRepository() {
      await invoke<Repository>("list_files").then((response) => {
        repository = response
      }).catch((error) => {
        console.error(error);
      });
    }

    onMount(async () => {
        await loadRepository();
    });

    function handleOnNewFileButtonClicked() {
        isAddingNewFile = true;
    }

    async function handleNewFileInputFocusLost(event: FocusEvent) {
        await invoke("create_file", { fileName: (event.target as HTMLInputElement).value });
        await loadRepository();

        isAddingNewFile = false;
    }

    async function handleNewFileInputKeyDown(event: KeyboardEvent) {
        if (event.key === "Enter") {
            await invoke("create_file", { fileName: (event.target as HTMLInputElement).value });
            await loadRepository();

            isAddingNewFile = false;
        }

        if (event.key === "Escape") {
            isAddingNewFile = false;
        }
    }

    async function handleRepositoryFileClicked(event: MouseEvent, key: string) {
        await invoke<string>("open_file", { key }).then((response) => {
            var fileView = document.getElementById("file-view");

            if (fileView === null) {
                return;
            }
            
            fileView.innerHTML = response;
        }).catch((error) => {
            console.error(error);
        });
    }
  </script>

<aside id="explorer">
    <div class="sidebar-header">Explorer</div>
    <div class="sidebar-content">
        <div class="sidebar-item">
            <div class="sidebar-item-header">{repository.general.repository_name}</div>
            <div class="sidebar-item-content">
                {#if isAddingNewFile}
                    <div class="sidebar-new-file-form">
                        <input type="text" placeholder="Enter file name" 
                            onblur={handleNewFileInputFocusLost}
                            onkeydown={handleNewFileInputKeyDown}
                        />
                    </div>
                {/if}
                {#if Object.keys(repository.objects).length === 0}
                    <button class="info" onclick={handleOnNewFileButtonClicked}>+ Add new file</button>
                {:else}
                <ul>
                    {#each Object.keys(repository.objects) as file}
                        <!-- svelte-ignore a11y_click_events_have_key_events -->
                        <!-- svelte-ignore a11y_missing_attribute -->
                        <!-- svelte-ignore a11y_no_static_element_interactions -->
                        <li><a onclick={(e) => handleRepositoryFileClicked(e, file)}>{repository.objects[file].name}</a></li>
                    {/each}
                </ul>
                {/if}
            </div>
        </div>
    </div>
</aside>