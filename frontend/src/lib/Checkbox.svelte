<script lang='ts'>
export let done: boolean;
export let started: boolean;
let state: 'u' | 's' | 'd';

$: {
  if (done) state = 'd';
  else if (started) state = 's';
  else state = 'u'
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

<button 
    on:contextmenu|preventDefault={handle_right_click} 
    on:click={handle_left_click}
>
<div 
    style="background-color: {test_colors[state]};" >
</div>
<slot/>
</button>

<style>
button {
  display: flex;
  align-items: center;
  gap: 10px;
  cursor: pointer;
  background: transparent;
  font-size: 1em;
}
div {
  margin: 0;
  border: 1px solid black;
  width: 1em;
  aspect-ratio: 1/1;
  border-radius: 2px;
  position: relative;
}
button:hover {
  color: #4a79e8;
}
</style>
