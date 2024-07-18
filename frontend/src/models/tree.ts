export class xDate {
  public readonly str: string;
  public readonly ts: number;

  constructor(d: Date) {
    const year = d.getFullYear();
    const month = d.getMonth() + 1;
    const day = d.getDate();

    this.ts = d.getTime();
    this.str = `${year}/${month}/${day}`
  }

  public comp(rhs: xDate) {
    return this.ts - rhs.ts;
  }
}

export type NodeType = {
  name: string,
  notes: string,
  started: boolean,
  done: boolean,
  startdue: null | number,
  deadline: null | number,
  children: NodeType[]
  is_open: boolean,
}

export class TreeNode {
  public readonly val: NodeType;
  public readonly children: TreeNode[] = [];

  constructor(val: NodeType, children: null | TreeNode[]) {
    this.val = val;
    if (children !== null) { 
      this.children = children;
    }
  }
}
