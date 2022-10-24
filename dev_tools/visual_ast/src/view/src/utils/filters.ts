import { RenderTree } from '../types/tree';
import { TreeData } from '../types/treeData';

export const render2TreeData = (root: RenderTree): TreeData => {
  const { name, children } = root;
  return {
    name,
    children: children?.map(render2TreeData),
  };
};
