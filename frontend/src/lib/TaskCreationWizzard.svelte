<script lang='ts'>
import { onDestroy } from "svelte";
import { type TypeTask, type TypeNewTask, from_object } from "../models/tasks";
import backend_adapter from "../util/backend_adapter";
import xevents from "../util/xevents";

let source_task: TypeTask | null | 'root' = null;

let name_input = "";
let notes_input = "";

const xlistener = xevents.listen('invoke_task_creation_wizzard', (task: TypeTask | 'root') => {
  source_task = task;
}) 

onDestroy(()=>{
  xlistener.cleanup()
})

const create_subtask = () => {
  if (source_task === null) return;
  console.log('creating subtask');
  const parent_id = source_task !== 'root' ? source_task.id : undefined;
  const new_task: TypeNewTask = { name: name_input, notes: notes_input, parent_id};
  backend_adapter.tasks.create(new_task).then(result=>{
    name_input = ""; notes_input = "";
    xevents.emit(`add_tasks:${parent_id??'root'}`, [result]);
    source_task = null;
  })
}

</script>

{#if source_task}
  <div class="wrap">
    <main>
      {#if source_task !== 'root'}
        <span>Create new subtask of "{source_task.name}"</span>
      {:else}
        <span>Create new task</span>
      {/if}
      <div class="input_wrap">
        <label for="name">Task Name:</label> <br>
        <input type="text" name="name" bind:value={name_input}>
      </div>
      <div class="input_wrap">
        <label for="notes">Task Notes:</label> <br>
        <input type="text" name="notes" bind:value={notes_input}>
      </div>

      <div class="foot_wrap">
        <button style="background: red;" on:click={()=>{source_task=null}}>Cancel</button>
        <button style="background: green;" on:click={create_subtask}>Create</button>
      </div>
    </main>
  </div>
{/if}

<style>
.foot_wrap {
  margin-top: 20px;
  display: flex;
  gap: 10px;
}
.foot_wrap>button {
  width: 100%;
  border: none;
}
.input_wrap {
  width: max-content;
  margin: auto;
}
label {
  font-size: 0.76em;
}
main {
  position: absolute;
  top: 50%; left: 50%;
  transform: translate(-50%, -50%);
  background: black;
  color: white;
  display: flex;
  flex-direction: column;
  width: max-content;
  padding: 10px;
  gap: 10px;
  border-radius: 4px;
}
.wrap {
  background: rgba(0, 0, 0, 0.5);
  position: fixed;
  top: 0; left: 0;
  width: 100vw; height: 100vh;
  backdrop-filter: blur(5px);
}
</style>
