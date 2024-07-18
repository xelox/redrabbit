import type { NodeType } from "../tree";

const backend_request = (path: string, payload: any) => {
  return new Promise<void>((resolve, reject) => {
    fetch(path, {method: 'POST', body: JSON.stringify(payload)}).then(res => {
      res.json()
        .then(resolve)
        .catch(reject)
    }).catch(reject);
  })
}

export default {
  tasks: {
    update: {
      done_state: (new_state: boolean) => {
        return backend_request('/api/tasks/update/done', {new_state});
      },
      started_state: (new_state: boolean) => {
        return backend_request('/api/tasks/update/started', {new_state});
      },
      meta: (new_state: NodeType) => {
        return backend_request('/api/tasks/update/meta', {new_state});
      }
    },
    create: (new_task: NodeType) => {
        return backend_request('/api/tasks/create', {new_task});
    },
    delete: (new_task: NodeType) => {
        return backend_request('/api/tasks/delete', {new_task});
    }
  } 
}