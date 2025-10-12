<script lang="ts">
  import type { PageProps } from "$types";

  let { data }: PageProps = $props();
</script>

<h1>Tasks</h1>

<form method="POST" action="?/create">
  <label>Description
    <input name="task-description" type="text"/>
  </label>
  <button>Add</button>
</form>

<ul>
  <h3>Page {data.tasksPaginationResponse.pageNumber ?? "?"} of {data.tasksPaginationResponse.maxPageNumber ?? "?"}</h3>

  {#each data.tasksPaginationResponse.items as task (task.id)}
    <li>
      <span>{task.description} ({task.status})</span>
      <span><i>[{task.id}]</i></span>
    </li>
  {/each}
</ul>
