<script lang="ts">
  import type { PageProps } from "./$types";

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
  {#each data.tasksPaginationResponse.items as task (task.id)}
    <li>
      <span>{task.description}</span>
    </li>
  {/each}
</ul>
