<script lang='ts'>
import type { NodeType, TypeNewTask } from "../models/tree";
    import backend_adapter from "../util/backend_adapter";

let node: NodeType | null = {
  id: '1',
  name: 'some task',
  notes: '',
  started: false,
  done: false,
  startdue: 0,
  deadline: 0,
  children: [],
  is_open: false,
};

let name_input = "";
let notes_input = "";

window.addEventListener('invoke_task_creation_wizzard', ((e: CustomEvent) => {
  node = e.detail.parent; 
}) as EventListener)

const create_subtask = () => {
  if (!node) return;
  const new_subtask: TypeNewTask = { name: name_input, notes: notes_input };
  backend_adapter.tasks.create(new_subtask, node).then(()=>{
    name_input = ""; notes_input = "";
    node = null;
  })
}

</script>

{#if node !== null}
  <div class="wrap">
    <main>
      <span>Create new subtask of "{node.name}"</span>
      <div class="input_wrap">
        <label for="name">Task Name:</label> <br>
        <input type="text" name="name" bind:value={name_input}>
      </div>
      <div class="input_wrap">
        <label for="notes">Task Notes:</label> <br>
        <input type="text" name="notes" bind:value={name_input}>
      </div>

      <div class="foot_wrap">
        <button style="background: red;" on:click={()=>{node=null}}>Cancel</button>
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
