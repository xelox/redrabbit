<script lang='ts'>
import { flip } from "svelte/animate";
import { from_object, type TypeObjectTask, type TypeTask, type TypeTaskMap } from "../models/tasks";
import TaskTree from "./TaskTree.svelte";
import { onDestroy, onMount } from "svelte";
import xevents, { XEventsCleanup } from "../util/xevents";
export let collection: TypeTaskMap;
export let parent_id: string | null;

const add_subtask = ((tasks: TypeObjectTask[]) => {
  console.log(tasks);
  for (const task of tasks) {
    collection.set(task.id, from_object(task));
  }
  collection = collection;
});

const remove_subtasks = ((task_id: string) => {
  collection.delete(task_id);
  collection = collection;
});

const listener = xevents.listen(`add_tasks:${parent_id??'root'}`, add_subtask)
      .listen(`remove_task:${parent_id??'root'}`, remove_subtasks)

onDestroy(() => {
  console.log('destroying...');
  listener.cleanup();
})
</script>

{#each Array.from(collection.values()).sort((a, b) => {return a.id > b.id ? -1 : 1}) as task (task.id)}
  <div class="task_wrap" animate:flip={{duration: 100}}>
    <TaskTree {task}/>
  </div>
{/each}

<style>

</style>
