export type NodeType = {
  id: string,
  name: string,
  notes: string,
  started: boolean,
  done: boolean,
  startdue: null | number,
  deadline: null | number,
  children: NodeType[]
  is_open: boolean,
}

export type TypeNewTask = {
  name: string,
  notes: string,
  startdue?: number,
  deadline?: number,
}
