<script lang='ts'>
import { flip } from "svelte/animate";
import { from_object, type TypeObjectTask, type TypeTaskMap } from "../models/tasks";
import TaskTree from "./TaskTree.svelte";
import { onDestroy, onMount } from "svelte";
export let collection: TypeTaskMap;
export let parent_id: string | null;

const add_subtask = ((e: CustomEvent) => {
  const task = from_object(e.detail.new_task as TypeObjectTask);
  collection.set(task.id, task);
  collection = collection;
  console.log('added', task);
}) as EventListener

const remove_subtask = ((e: CustomEvent) => {
  const task_id = e.detail.task_id as string;
  collection.delete(task_id);
  collection = collection;
}) as EventListener

onMount(()=>{
  window.addEventListener(`add_subtask:${parent_id??'root'}`, add_subtask)
  window.addEventListener(`remove_subtask:${parent_id??'root'}`, remove_subtask)
})
onDestroy(() => {
  window.removeEventListener(`add_subtask:${parent_id??'root'}`, add_subtask)
  window.addEventListener(`remove_subtask:${parent_id??'root'}`, remove_subtask)
})
</script>

{#each Array.from(collection.values()).sort((a, b) => {return a.id > b.id ? -1 : 1}) as task (task.id)}
  <div class="task_wrap" animate:flip>
    <TaskTree node={task}/>
  </div>
{/each}

<style>

</style>
