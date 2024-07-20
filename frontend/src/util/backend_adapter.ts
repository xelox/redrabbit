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
    update: {
      done_state: (changes: {id: number, new_state: boolean}[]) => {
        return backend_request('/api/tasks/update/done', {changes});
      },
      started_state: (changes: {id: number, new_state: boolean}[]) => {
        return backend_request('/api/tasks/update/started', {changes});
      },
      meta: (new_state: TypeTask) => {
        return backend_request('/api/tasks/update/meta', {new_state});
      }
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
