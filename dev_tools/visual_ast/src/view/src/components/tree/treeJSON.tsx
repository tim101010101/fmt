import { TreeView, TreeItem } from '@mui/lab';
import { ExpandMore, ChevronRight } from '@mui/icons-material';
import { RenderTree } from '../../types/tree';

const Tree = ({ root }: { root: RenderTree }) => {
  const renderTree = (node: RenderTree) => (
    <TreeItem key={node.id} nodeId={node.id} label={node.name}>
      {Array.isArray(node.children)
        ? node.children.map(n => renderTree(n))
        : null}
    </TreeItem>
  );

  return (
    <TreeView
      aria-label='tree'
      defaultExpanded={['root']}
      defaultCollapseIcon={<ExpandMore />}
      defaultExpandIcon={<ChevronRight />}
      sx={{ height: 790, flexGrow: 1, overflowY: 'auto' }}
    >
      {renderTree(root)}
    </TreeView>
  );
};

export default Tree;
