import type { NodeType } from "../tree";

const backend_request = (path: string, payload: any) => {
  return new Promise<any>((resolve, reject) => {
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
        return new Promise<void>((resolve, reject) => {
          backend_request('/api/tasks/update_done', {new_state}).then(resolve).catch(reject);
        })
      },
      started_state: (new_state: boolean) => {
        return new Promise<void>((resolve, reject) => {
          backend_request('/api/tasks/update_started', {new_state}).then(resolve).catch(reject);
        })
      },
      meta: (new_state: NodeType) => {
        return new Promise<void>((resolve, reject) => {
          backend_request('/api/tasks/update_meta', {new_state}).then(resolve).catch(reject);
        })
      }
    }
  } 
}
