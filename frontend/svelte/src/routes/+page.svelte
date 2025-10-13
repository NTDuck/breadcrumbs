<script lang="ts">
  import type { PageProps } from "$types";
  import { Trash2 } from "lucide-svelte";

  let { data }: PageProps = $props();
</script>

<h1>Tasks</h1>

<form method="POST">
  <label>Description
    <input name="task-description" type="text"/>
  </label>
  <button formaction="?/create">Add</button>
</form>

<ul>
  <h3>Page {data.tasksPaginationResponse.pageNumber ?? "?"} of {data.tasksPaginationResponse.maxPageNumber ?? "?"}</h3>

  {#each data.tasksPaginationResponse.items as task (task.id)}
    <li>
      <form method="POST">
        <input type="hidden" name="task-id" value={task.id}/>
        <span>{task.description} ({task.status})</span>
        <span><i>[{task.id}]</i></span>
        <button formaction="?/delete"><Trash2 /></button>
      </form>
    </li>
  {/each}
</ul>
