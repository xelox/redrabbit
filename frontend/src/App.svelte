<script lang='ts'>
import TaskCreationWizzard from "./lib/TaskCreationWizzard.svelte";
import TasksToolbar from "./lib/TasksToolbar.svelte";
import { from_object, type TypeObjectTask, type TypeTaskMap } from "./models/tasks";
import backend_adapter from "./util/backend_adapter";
import TaskCollection from "./lib/TaskCollection.svelte";

let roots: TypeTaskMap = new Map();
backend_adapter.tasks.load().then(res=>{
  for (const obj of (res as TypeObjectTask[])) {
    roots.set(obj.id, from_object(obj));
  }
  roots = roots;
  console.log(roots);
})


</script>

<main>
  <div class="vessel">
    <TasksToolbar/>
    <div class="tree_wrap">
      <TaskCollection collection={roots} parent_id={null}/>
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
