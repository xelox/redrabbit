import backend_adapter from "../util/backend_adapter";
import xevents from "../util/xevents";

export type TypeTaskMap = Map<string, TypeTask>;

export type TypeTask = {
  id: string,
  name: string,
  notes: string,
  started: boolean,
  done: boolean,
  startdue: null | number,
  deadline: null | number,
  is_open: boolean,
  parent_id: null | string,
  children: TypeTaskMap,
  parent_recount?: ()=>void
}

type TypeTaskOjbectMap = {[id: string]: TypeObjectTask}

export type TypeObjectTask = Omit<TypeTask, 'children'> & {children: TypeTaskOjbectMap}

export type TypeNewTask = {
  name: string,
  notes: string,
  startdue?: number,
  deadline?: number,
  parent_id?: string,
}

export function compare_completion(task: TypeTask, done: boolean, started: boolean) {
  console.log('paren:', done, started);
  console.log('child:', task.done, task.started);

  const res = (() => {
    if (task.done) {
      return false;
    }
    if (task.started) {
      return done
    }
    return done || started;
  })()
  
  console.log("result:", res);
  console.log();
  return res;
}

export function from_object(obj: TypeObjectTask): TypeTask {
  const children = new Map<string, TypeTask>();
  for (const [id, c] of Object.entries(obj.children)) {
    children.set(id, from_object(c));
  }
  return {
    ...obj, children
  }
}
