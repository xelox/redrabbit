<script lang='ts'>
import TaskCreationWizzard from "./lib/TaskCreationWizzard.svelte";
import TasksToolbar from "./lib/TasksToolbar.svelte";
import TaskTree from "./lib/TaskTree.svelte";
import { type TypeTask } from "./models/tasks";
import backend_adapter from "./util/backend_adapter";

let roots: TypeTask[] = [];
backend_adapter.tasks.load().then(res=>{
  roots = res as TypeTask[];
})

</script>

<main>
  <div class="vessel">
    <TasksToolbar/>
    <div class="tree_wrap">
      {#each roots as root}
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
