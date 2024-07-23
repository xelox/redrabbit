type Callback = () => void;
type DoUndoPairs = {do_callback: Callback, undo_callback: Callback};

const undo_stack: DoUndoPairs[] = [];
const do_stack: DoUndoPairs[] = [];

export default {
  do: (do_callback: Callback, undo_callback: Callback) => {
    do_stack.push({do_callback, undo_callback});
    undo_stack.splice(0, undo_stack.length);
    do_callback();
  },
  undo: () => {
    const callbacks = do_stack.pop();
    if (!callbacks) return;
    callbacks.undo_callback();
    undo_stack.push(callbacks);
  },
  redo: () => {
    const callbacks = undo_stack.pop();
    if (!callbacks) return;
    callbacks.do_callback();
    do_stack.push(callbacks);
  }
}
