<script lang='ts'>
import TaskCreationWizzard from "./lib/TaskCreationWizzard.svelte";
import TasksToolbar from "./lib/TasksToolbar.svelte";
import {type TypeTaskMap } from "./models/tasks";
import backend_adapter from "./util/backend_adapter";
import TaskCollection from "./lib/TaskCollection.svelte";
import xevents from "./util/xevents";

let roots: TypeTaskMap = new Map();
backend_adapter.tasks.load().then(result=>{
  console.log(result);
  xevents.emit(`add_tasks:root`, result);
})


</script>

<main>
  <div class="vessel">
    <TasksToolbar/>
    <div class="tree_wrap">
      <TaskCollection parent={null}/>
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
