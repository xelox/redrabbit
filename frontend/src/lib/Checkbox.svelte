<script lang='ts'>
export let done: boolean;
export let started: boolean;
let state: 'u' | 's' | 'd';

$: {
  if (done) state = 'd';
  else if (started) state = 's';
  else state = 'u'
  console.log(state);
}


const test_colors = {
  u: 'white',
  s: 'blue',
  d: 'green',
}

const handle_click = (main: boolean) => {
  if (main) {
    done = !done;
  } else if (!done) {
    started = !started;
  }
}
const handle_right_click = () => { handle_click(false) }
const handle_left_click = () => { handle_click(true) }
</script>

<button on:contextmenu|preventDefault={handle_right_click} class='main' style="background-color: {test_colors[state]};" on:click={handle_left_click}>
</button>

<style>
.main {
  margin: 0;
  border: 1px solid black;
  width: 1em;
  aspect-ratio: 1/1;
  border-radius: 4px;
  position: relative;
}
</style>
