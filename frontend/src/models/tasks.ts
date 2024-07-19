import { writable } from "svelte/store"

export type TypeTask = {
  id: string,
  name: string,
  notes: string,
  started: boolean,
  done: boolean,
  startdue: null | number,
  deadline: null | number,
  children: TypeTask[]
  is_open: boolean,
}

export type TypeNewTask = {
  name: string,
  notes: string,
  startdue?: number,
  deadline?: number,
  parent_id?: string,
}

export const Tasks = writable<TypeTask[]>([]);
