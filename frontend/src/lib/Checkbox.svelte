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

export let handle_done: (() => void) = () => {
  done = !done;
  if (done) started = true;
}
export let handle_started: (() => void) = () => {
  if (!done) started = !started;
}
</script>

<button 
    on:contextmenu|preventDefault={handle_done} 
    on:click={handle_started}
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
