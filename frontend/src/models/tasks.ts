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
  parent?: TypeTask,
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
  if (task.done) {
    return -1;
  }
  if (task.started) {
    return 1
  }
  if (started && !done) return 1;
  return 0;
}

export function from_object(obj: TypeObjectTask): TypeTask {
  const children = new Map<string, TypeTask>();
  for (const [id, c] of Object.entries(obj.children)) {
    children.set(id, from_object(c));
  }
  const task = { ...obj, children }
  for (const child of task.children.values()) {
    child.parent = task;
  }
  return task;
}
