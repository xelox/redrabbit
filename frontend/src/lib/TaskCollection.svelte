<script lang='ts'>
import { flip } from "svelte/animate";
import { from_object, type TypeObjectTask, type TypeTaskMap } from "../models/tasks";
import TaskTree from "./TaskTree.svelte";
import { onDestroy, onMount } from "svelte";
    import xevents, { XEventsCleanup } from "../util/xevents";
export let collection: TypeTaskMap;
export let parent_id: string | null;

const add_subtask = ((new_task: TypeObjectTask) => {
  const task = from_object(new_task);
  collection.set(task.id, task);
  collection = collection;
  console.log('added', task);
});

const remove_subtask = ((task_id: string) => {
  collection.delete(task_id);
  collection = collection;
});

let cleanup_fns: XEventsCleanup
onMount(()=>{
    cleanup_fns = xevents.listen(`add_task:${parent_id??'root'}`, add_subtask)
      .listen(`remove_task:${parent_id??'root'}`, remove_subtask)
});

onDestroy(() => {
  cleanup_fns.cleanup();
})
</script>

{#each Array.from(collection.values()).sort((a, b) => {return a.id > b.id ? -1 : 1}) as task (task.id)}
  <div class="task_wrap" animate:flip>
    <TaskTree {task}/>
  </div>
{/each}

<style>

</style>
