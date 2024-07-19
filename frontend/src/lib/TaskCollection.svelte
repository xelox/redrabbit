<script lang='ts'>
import { flip } from "svelte/animate";
import { from_object, type TypeObjectTask, type TypeTask } from "../models/tasks";
import TaskTree from "./TaskTree.svelte";
import { onDestroy, onMount } from "svelte";
import xevents from "../util/xevents";
import { slide } from "svelte/transition";
export let parent: TypeTask | null;
$: collection = parent?.children || new Map(); 
export let recount_tasks: (() => void) | undefined = undefined;

const add_subtask = ((tasks: TypeObjectTask[]) => {
  console.log(tasks);
  for (const task of tasks) {
    collection.set(task.id, from_object(task));
  }
  collection = collection;
  recount_tasks?.call(null);
});

const remove_subtasks = ((task_id: string) => {
  collection.delete(task_id);
  collection = collection;
  recount_tasks?.call(null);
});

const listener = xevents.listen(`add_tasks:${parent?.id??'root'}`, add_subtask)
      .listen(`remove_task:${parent?.id??'root'}`, remove_subtasks)

onDestroy(() => {
  console.log('destroying...');
  listener.cleanup();
})
</script>

{#if parent?.is_open || parent === null}
  <div class:children_wrap={parent!==null} transition:slide>
    {#each Array.from(collection.values()).sort((a, b) => {return a.id > b.id ? -1 : 1}) as task (task.id)}
      <div class="task_wrap" animate:flip={{duration: 100}}>
        <TaskTree {task}/>
      </div>
    {/each}
  </div>
{/if}

<style>
.children_wrap {
  padding-left: 30px;
}
</style>
