<script lang='ts'>
import Tree from "./lib/Tree.svelte";
import { type NodeType } from "./models/tree";

let root: NodeType = {
  name: 'n1',
  notes: '',
  started: false,
  done: false,
  startdue: 0,
  deadline: 0,
  children: [
    {
      name: 'n2',
      notes: '',
      started: false,
      done: false,
      startdue: 0,
      deadline: 0,
      children: [
        {
          name: 'n3',
          notes: '',
          started: false,
          done: false,
          startdue: 0,
          deadline: 0,
          children: [],
          is_open: true,
        }
      ],
      is_open: true,
    },
    {
      name: 'n4',
      notes: '',
      started: false,
      done: false,
      startdue: 0,
      deadline: 0,
      children: [],
      is_open: true,
    },
    {
      name: 'n5',
      notes: '',
      started: false,
      done: false,
      startdue: 0,
      deadline: 0,
      children: [],
      is_open: true,
    }
  ],
  is_open: true,
}
</script>

<main>
  <div class="tree_wrap">
    <Tree node={root}/>
  </div>
</main>

<style>
.tree_wrap {
  width: 800px;
  border-right: 1px solid #ddd;
  border-top: 1px solid #ddd;
  margin: 0 auto 0 auto;
}
</style>
