import type { TypeTask, TypeNewTask } from "../models/tasks";
import xevents from "./xevents";

const backend_request = (path: string, payload: any, skip_deser?: boolean) => {
  let full_path = "http://localhost:8080" + path;
  console.log(full_path);
  return new Promise<any>((resolve, reject) => {
    const headers: HeadersInit = [["Content-Type", "application/json"]];
    fetch(full_path, {
      method: 'POST', 
      headers,
      body: JSON.stringify(payload)
    }).then(res => {
        if(!skip_deser) {
          res.json()
            .then(resolve)
            .catch(reject)
        } else {
          resolve({})
        }
    }).catch(reject);
  })
}

export default {
  tasks: {
    update_completion: (done: boolean, started: boolean, ids: string[]) => {
      return backend_request('/api/tasks/update_completion', {done, started, ids}, true)
    },
    expand_collapse: (st: boolean, ids: string[]) => {
      return backend_request('/api/tasks/expand_collapse', {st, ids}, true)
    },
    create: (new_task: TypeNewTask) => {
        return backend_request('/api/tasks/create', new_task);
    },
    delete: (delete_target: {id: string}) => {
        return backend_request('/api/tasks/delete', delete_target, true);
    },
    load: (id?: string) => {
        return backend_request('/api/tasks/load', {id});
    }
  } 
}
