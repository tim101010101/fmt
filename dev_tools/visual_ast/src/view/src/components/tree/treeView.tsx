import { Box } from '@mui/material';
import { TreeChart } from '../chart';
import { RenderTree } from '../../types/tree';
import { render2TreeData } from '../../utils/filters';

const TreeView = ({ root }: { root: RenderTree }) => {
  return (
    <Box sx={{ height: 790, flexGrow: 1, overflowY: 'auto' }}>
      <TreeChart data={render2TreeData(root)} />
    </Box>
  );
};

export default TreeView;
