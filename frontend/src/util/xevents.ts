type Listener = (...args: any[]) => void;

let max_id = 0;
const reusable_ids = new Array<number>();
const event_listener_map = new Map<number, Listener>();
const event_id_map = new Map<string, Set<number>>();

export default {
  listen: function(name: string, listener: Listener): () => void {
    const id = reusable_ids.pop() ?? ++max_id;

    const id_set = event_id_map.get(name);

    if (id_set) id_set.add(id)
    else event_id_map.set(name, new Set([id]));

    event_listener_map.set(id, listener);

    return () => {
      const id_set = event_id_map.get(name);

      if (id_set) {
        id_set.delete(id);
        if (id_set.size === 0) {
          event_id_map.delete(name);
        }
      }

      event_listener_map.delete(id);

      reusable_ids.push(id);
    }
  },

  emit: function(name: string, ...args: []) {
    const ids = event_id_map.get(name); 
    if (!ids) return;
    for (const id of ids) {
      const listener = event_listener_map.get(id);
      listener?.call(null, args);
    }
  }
}
