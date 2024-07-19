<script lang='ts'>
import { onDestroy, onMount } from "svelte";
import TaskCreationWizzard from "./lib/TaskCreationWizzard.svelte";
import TasksToolbar from "./lib/TasksToolbar.svelte";
import TaskTree from "./lib/TaskTree.svelte";
import { from_object, type TypeObjectTask, type TypeTask, type TypeTaskMap } from "./models/tasks";
import backend_adapter from "./util/backend_adapter";

let roots: TypeTaskMap = new Map();
backend_adapter.tasks.load().then(res=>{
  for (const obj of (res as TypeObjectTask[])) {
    roots.set(obj.id, from_object(obj));
  }
  roots = roots;
  console.log(roots);
})

const add_subtask = ((e: CustomEvent) => {
  const task = from_object(e.detail.new_task as TypeObjectTask);
  roots.set(task.id, task);
  roots = roots;
  console.log('added', task);
}) as EventListener

const remove_subtask = ((e: CustomEvent) => {
  const task_id = e.detail.task_id as string;
  roots.delete(task_id);
  roots = roots;
}) as EventListener

onMount(()=>{
  window.addEventListener(`add_subtask:root`, add_subtask)
  window.addEventListener(`remove_subtask:root`, remove_subtask)
})
onDestroy(() => {
  window.removeEventListener(`add_subtask:root`, add_subtask)
  window.addEventListener(`remove_subtask:root`, remove_subtask)
})

</script>

<main>
  <div class="vessel">
    <TasksToolbar/>
    <div class="tree_wrap">
      {#each roots.values() as root (root.id)}
        <TaskTree node={root}/>
      {/each}
    </div>
  </div>
</main>

<TaskCreationWizzard/>

<style>
.vessel {
  width: 800px;
  margin: auto;
}
.tree_wrap {
  width: 100%;
  border-right: 1px solid #ddd;
  border-top: 1px solid #ddd;
  margin: 0 auto 0 auto;
}
</style>
