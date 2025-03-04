<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import Logger from "../common/logger";

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
    let isAddingNewDirectory = $state(false);
    let activeFileKey = $state("");
  
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

    async function handleNewDirectoryInputFocusLost(event: FocusEvent) {
        await invoke("create_directory", { directoryName: (event.target as HTMLInputElement).value }).catch((error) => {
            Logger.getInstance().error(error);
        });

        await loadRepository();

        isAddingNewDirectory = false;
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

    async function handleNewDirectoryInputKeyDown(event: KeyboardEvent) {
        if (event.key === "Enter") {
            await invoke("create_directory", { directoryName: (event.target as HTMLInputElement).value });
            await loadRepository();

            isAddingNewDirectory = false;
        }

        if (event.key === "Escape") {
            isAddingNewDirectory = false;
        }
    }

    async function handleRepositoryFileClicked(event: MouseEvent, key: string) {
        await invoke<string>("open_file", { key }).then((response) => {
            var fileTitle = document.getElementById("file-title");
            var fileView = document.getElementById("file-view");

            if (fileTitle === null) {
                return;
            }

            fileTitle.innerHTML = repository.objects[key].name;
            fileTitle.dataset.repositoryKey = key;

            if (fileView === null) {
                return;
            }

            fileView.innerHTML = response;
            activeFileKey = key;
        }).catch((error) => {
            Logger.getInstance().error(error);
        });
    }

    function handleOnNewDirectoryButtonClicked() {
        isAddingNewDirectory = true;
    }
  </script>

<aside id="explorer">
    <div class="sidebar-header">Explorer</div>
    <div class="sidebar-content">
        <div class="sidebar-item">
            <div class="sidebar-item-header">
                {repository.general.repository_name}
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <span class="sidebar-item-header-newfile" onclick={handleOnNewFileButtonClicked}>
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512"><!--!Font Awesome Free 6.7.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2025 Fonticons, Inc.--><path d="M0 64C0 28.7 28.7 0 64 0L224 0l0 128c0 17.7 14.3 32 32 32l128 0 0 38.6C310.1 219.5 256 287.4 256 368c0 59.1 29.1 111.3 73.7 143.3c-3.2 .5-6.4 .7-9.7 .7L64 512c-35.3 0-64-28.7-64-64L0 64zm384 64l-128 0L256 0 384 128zm48 96a144 144 0 1 1 0 288 144 144 0 1 1 0-288zm16 80c0-8.8-7.2-16-16-16s-16 7.2-16 16l0 48-48 0c-8.8 0-16 7.2-16 16s7.2 16 16 16l48 0 0 48c0 8.8 7.2 16 16 16s16-7.2 16-16l0-48 48 0c8.8 0 16-7.2 16-16s-7.2-16-16-16l-48 0 0-48z"/></svg>
                </span>
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <span class="sidebar-item-header-newdirectory" onclick={handleOnNewDirectoryButtonClicked}>
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><!--!Font Awesome Free 6.7.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2025 Fonticons, Inc.--><path d="M512 416c0 35.3-28.7 64-64 64L64 480c-35.3 0-64-28.7-64-64L0 96C0 60.7 28.7 32 64 32l128 0c20.1 0 39.1 9.5 51.2 25.6l19.2 25.6c6 8.1 15.5 12.8 25.6 12.8l160 0c35.3 0 64 28.7 64 64l0 256zM232 376c0 13.3 10.7 24 24 24s24-10.7 24-24l0-64 64 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-64 0 0-64c0-13.3-10.7-24-24-24s-24 10.7-24 24l0 64-64 0c-13.3 0-24 10.7-24 24s10.7 24 24 24l64 0 0 64z"/></svg>
                </span>
            </div>
            <div class="sidebar-item-content">
                {#if isAddingNewFile}
                    <div class="sidebar-new-file-form">
                        <input type="text" placeholder="Enter file name" 
                            onblur={handleNewFileInputFocusLost}
                            onkeydown={handleNewFileInputKeyDown}
                        />
                    </div>
                {/if}
                {#if isAddingNewDirectory}
                    <div class="sidebar-new-directory-form">
                        <input type="text" placeholder="Enter directory name" 
                            onblur={handleNewDirectoryInputFocusLost}
                            onkeydown={handleNewDirectoryInputKeyDown}
                        />
                    </div>
                {/if}
                {#if Object.keys(repository.objects).length === 0}
                    <button class="info" onclick={handleOnNewFileButtonClicked}>+ Add new file</button>
                {:else}
                <ul>
                    {#each Object.keys(repository.objects) as file}
                        {#if repository.objects[file].is_directory}
                            <li class="sidebar-item-is-directory">
                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><!--!Font Awesome Free 6.7.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2025 Fonticons, Inc.--><path d="M64 480H448c35.3 0 64-28.7 64-64V160c0-35.3-28.7-64-64-64H288c-10.1 0-19.6-4.7-25.6-12.8L243.2 57.6C231.1 41.5 212.1 32 192 32H64C28.7 32 0 60.7 0 96V416c0 35.3 28.7 64 64 64z"/></svg>
                                {repository.objects[file].name}
                            </li>
                        {:else}
                            <!-- svelte-ignore a11y_click_events_have_key_events -->
                            <!-- svelte-ignore a11y_missing_attribute -->
                            <!-- svelte-ignore a11y_no_static_element_interactions -->
                            <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
                            <li class={activeFileKey == file ? "is-active" : ""} onclick={(e) => handleRepositoryFileClicked(e, file)}><a>{repository.objects[file].name}</a></li>
                        {/if}
                    {/each}
                </ul>
                {/if}
            </div>
        </div>
    </div>
</aside>