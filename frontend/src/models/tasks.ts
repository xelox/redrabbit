export type TypeTaskMap = Map<string, TypeTask>;

export type TypeTask = {
  id: string,
  name: string,
  notes: string,
  started: boolean,
  done: boolean,
  startdue: null | number,
  deadline: null | number,
  children: TypeTaskMap,
  is_open: boolean,
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

export function from_object(obj: TypeObjectTask): TypeTask {
  const children = new Map<string, TypeTask>();
  for (const [id, c] of Object.entries(obj.children)) {
    children.set(id, from_object(c));
  }
  return {
    ...obj, children
  }
}
